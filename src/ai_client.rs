use agent_client_protocol::{
    Agent, Client, ClientCapabilities, ClientSideConnection, ContentBlock, Error as AcpError,
    InitializeRequest, NewSessionRequest, PromptRequest, ReadTextFileRequest, ReadTextFileResponse,
    RequestPermissionOutcome, RequestPermissionRequest, RequestPermissionResponse,
    SessionNotification, WriteTextFileRequest, V1,
};
use anyhow::{anyhow, Result};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use tokio::sync::{mpsc, oneshot};
use tracing::{debug, error, info};

use crate::models::AiEngine;

/// Commands sent to the ACP worker thread
enum AcpCommand {
    SendPrompt {
        prompt: String,
        response_tx: oneshot::Sender<Result<String>>,
    },
    Stop,
}

/// AI Client that manages connections to AI engines via ACP
pub struct AiClient {
    engine: AiEngine,
    cmd_tx: Option<mpsc::Sender<AcpCommand>>,
    worker_handle: Option<std::thread::JoinHandle<()>>,
}

impl AiClient {
    /// Create a new AI client for the specified engine
    pub fn new(engine: AiEngine) -> Self {
        Self {
            engine,
            cmd_tx: None,
            worker_handle: None,
        }
    }

    /// Start the AI agent process based on the configured engine
    pub async fn start(&mut self) -> Result<()> {
        if self.cmd_tx.is_some() {
            info!("AI agent already running");
            return Ok(());
        }

        info!("Starting AI agent: {:?}", self.engine);

        // Check if npx is available
        if !self.check_npx_available() {
            return Err(anyhow!(
                "Node.js (npx) is required for {}. Please install Node.js.",
                self.engine
            ));
        }

        let agent_package = match self.engine {
            AiEngine::ClaudeCode => "@zed-industries/claude-code-acp",
            AiEngine::Codex => "@zed-industries/codex-acp",
            AiEngine::OpenAI => {
                return Err(anyhow!(
                    "OpenAI engine is not supported via ACP. Use direct API instead."
                ));
            }
        };

        // Create command channel
        let (cmd_tx, cmd_rx) = mpsc::channel(10);

        // Spawn ACP worker thread
        let worker_handle = std::thread::spawn(move || {
            Self::run_acp_worker(agent_package, cmd_rx);
        });

        self.cmd_tx = Some(cmd_tx);
        self.worker_handle = Some(worker_handle);
        info!("AI agent worker thread started");

        Ok(())
    }

    /// Run ACP worker in a dedicated thread with LocalSet
    fn run_acp_worker(agent_package: &'static str, mut cmd_rx: mpsc::Receiver<AcpCommand>) {
        // Create a new tokio runtime for this thread
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed to create tokio runtime");

        runtime.block_on(async {
            // Start the agent process
            let mut child = match tokio::process::Command::new("npx")
                .arg(agent_package)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
            {
                Ok(child) => child,
                Err(e) => {
                    error!("Failed to start ACP agent: {}", e);
                    return;
                }
            };

            let stdin = child.stdin.take().expect("Failed to get stdin");
            let stdout = child.stdout.take().expect("Failed to get stdout");
            let stderr = child.stderr.take().expect("Failed to get stderr");

            // Log stderr
            tokio::spawn(async move {
                use tokio::io::{AsyncBufReadExt, BufReader};
                let reader = BufReader::new(stderr);
                let mut lines = reader.lines();
                while let Ok(Some(line)) = lines.next_line().await {
                    debug!("[ACP Agent] {}", line);
                }
            });

            // Convert to futures-compatible types
            use tokio_util::compat::{TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt};
            let stdin_compat = stdin.compat_write();
            let stdout_compat = stdout.compat();

            // Create LocalSet for non-Send futures
            let local_set = tokio::task::LocalSet::new();

            local_set
                .run_until(async move {
                    // Response buffer shared between tasks
                    let response_buffer: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

                    // Create ACP client
                    let client = RequiemAcpClient {
                        response_buffer: response_buffer.clone(),
                    };

                    // Create ClientSideConnection
                    let (conn, io_task) =
                        ClientSideConnection::new(client, stdin_compat, stdout_compat, |fut| {
                            tokio::task::spawn_local(fut);
                        });

                    // Spawn IO task
                    tokio::task::spawn_local(async move {
                        if let Err(e) = io_task.await {
                            error!("ACP IO task failed: {}", e);
                        }
                    });

                    // Initialize ACP protocol
                    if let Err(e) = conn
                        .initialize(InitializeRequest {
                            protocol_version: V1,
                            client_capabilities: ClientCapabilities::default(),
                            meta: None,
                        })
                        .await
                    {
                        error!("Failed to initialize ACP: {}", e);
                        return;
                    }

                    info!("ACP protocol initialized");

                    // Create a session
                    let session_response = match conn
                        .new_session(NewSessionRequest {
                            mcp_servers: Vec::new(),
                            cwd: std::env::current_dir().unwrap_or_default(),
                            meta: None,
                        })
                        .await
                    {
                        Ok(resp) => resp,
                        Err(e) => {
                            error!("Failed to create ACP session: {}", e);
                            return;
                        }
                    };

                    let session_id = session_response.session_id;
                    info!("ACP session created: {:?}", session_id);

                    // Process commands
                    while let Some(cmd) = cmd_rx.recv().await {
                        match cmd {
                            AcpCommand::SendPrompt {
                                prompt,
                                response_tx,
                            } => {
                                // Clear response buffer
                                response_buffer.lock().unwrap().clear();

                                // Send prompt
                                let result = conn
                                    .prompt(PromptRequest {
                                        session_id: session_id.clone(),
                                        prompt: vec![prompt.into()],
                                        meta: None,
                                    })
                                    .await;

                                let response = match result {
                                    Ok(_) => {
                                        // Collect response from buffer
                                        let chunks = response_buffer.lock().unwrap();
                                        Ok(chunks.join(""))
                                    }
                                    Err(e) => Err(anyhow!("ACP prompt failed: {}", e)),
                                };

                                let _ = response_tx.send(response);
                            }
                            AcpCommand::Stop => {
                                info!("Stopping ACP worker");
                                break;
                            }
                        }
                    }
                })
                .await;

            // Kill child process
            let _ = child.kill().await;
        });
    }

    /// Check if npx is available on the system
    fn check_npx_available(&self) -> bool {
        Command::new("npx")
            .arg("--version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    /// Send a prompt to the AI agent and get a response
    pub async fn send_prompt(&mut self, prompt: &str) -> Result<String> {
        if self.cmd_tx.is_none() {
            self.start().await?;
        }

        let cmd_tx = self
            .cmd_tx
            .as_ref()
            .ok_or_else(|| anyhow!("AI agent not started"))?;

        info!("Sending prompt to AI agent: {} chars", prompt.len());

        // Create response channel
        let (response_tx, response_rx) = oneshot::channel();

        // Send command to worker
        cmd_tx
            .send(AcpCommand::SendPrompt {
                prompt: prompt.to_string(),
                response_tx,
            })
            .await
            .map_err(|_| anyhow!("Failed to send command to ACP worker"))?;

        // Wait for response
        response_rx
            .await
            .map_err(|_| anyhow!("Failed to receive response from ACP worker"))?
    }

    /// Stop the AI agent process
    pub async fn stop(&mut self) -> Result<()> {
        if let Some(cmd_tx) = self.cmd_tx.take() {
            info!("Stopping AI agent");
            let _ = cmd_tx.send(AcpCommand::Stop).await;
        }

        if let Some(_handle) = self.worker_handle.take() {
            // Give worker thread time to clean up
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            // Note: We can't join the thread in async context
            // The thread will terminate on its own
        }

        Ok(())
    }

    /// Check if the agent is running
    pub fn is_running(&self) -> bool {
        self.cmd_tx.is_some()
    }
}

impl Drop for AiClient {
    fn drop(&mut self) {
        if let Some(cmd_tx) = self.cmd_tx.take() {
            // Try to send stop command
            let _ = cmd_tx.try_send(AcpCommand::Stop);
        }
    }
}

/// Implementation of the Client trait for ACP communication
/// This handles requests from the AI agent back to the client
pub struct RequiemAcpClient {
    response_buffer: Arc<Mutex<Vec<String>>>,
}

#[async_trait::async_trait(?Send)]
impl Client for RequiemAcpClient {
    async fn request_permission(
        &self,
        args: RequestPermissionRequest,
    ) -> Result<RequestPermissionResponse, AcpError> {
        // For now, auto-approve all permissions by selecting the first AllowOnce option
        // In production, this should prompt the user
        debug!("Permission requested for session: {:?}", args.session_id);
        debug!("Tool call: {:?}", args.tool_call);

        // Find the first "allow_once" option
        if let Some(option) = args.options.iter().find(|opt| {
            matches!(
                opt.kind,
                agent_client_protocol::PermissionOptionKind::AllowOnce
            )
        }) {
            Ok(RequestPermissionResponse {
                outcome: RequestPermissionOutcome::Selected {
                    option_id: option.id.clone(),
                },
                meta: None,
            })
        } else {
            // If no allow option, return cancelled
            Ok(RequestPermissionResponse {
                outcome: RequestPermissionOutcome::Cancelled,
                meta: None,
            })
        }
    }

    async fn session_notification(&self, args: SessionNotification) -> Result<(), AcpError> {
        debug!(
            "Session notification for {:?}: {:?}",
            args.session_id, args.update
        );

        // Collect agent message chunks
        match args.update {
            agent_client_protocol::SessionUpdate::AgentMessageChunk { content } => {
                let text = match content {
                    ContentBlock::Text(text_content) => text_content.text,
                    ContentBlock::Image(_) => "<image>".to_string(),
                    ContentBlock::Audio(_) => "<audio>".to_string(),
                    ContentBlock::ResourceLink(link) => link.uri,
                    ContentBlock::Resource(_) => "<resource>".to_string(),
                };
                self.response_buffer.lock().unwrap().push(text);
            }
            _ => {
                // Ignore other notification types for now
            }
        }

        Ok(())
    }

    async fn write_text_file(
        &self,
        args: WriteTextFileRequest,
    ) -> Result<agent_client_protocol::WriteTextFileResponse, AcpError> {
        debug!("AI agent requesting to write file: {}", args.path.display());

        tokio::fs::write(&args.path, &args.content)
            .await
            .map_err(|e| {
                AcpError::internal_error().with_data(format!("Failed to write file: {}", e))
            })?;

        Ok(agent_client_protocol::WriteTextFileResponse::default())
    }

    async fn read_text_file(
        &self,
        args: ReadTextFileRequest,
    ) -> Result<ReadTextFileResponse, AcpError> {
        debug!("AI agent requesting to read file: {}", args.path.display());

        let content = tokio::fs::read_to_string(&args.path).await.map_err(|e| {
            AcpError::internal_error().with_data(format!("Failed to read file: {}", e))
        })?;

        // Handle line/limit parameters if specified
        let content = if let Some(line) = args.line {
            let lines: Vec<&str> = content.lines().collect();
            let start = (line.saturating_sub(1) as usize).min(lines.len());
            let end = if let Some(limit) = args.limit {
                (start + limit as usize).min(lines.len())
            } else {
                lines.len()
            };

            lines[start..end].join("\n")
        } else {
            content
        };

        Ok(ReadTextFileResponse {
            content,
            meta: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_check_npx_available() {
        let client = AiClient::new(AiEngine::ClaudeCode);
        // This test will pass or fail depending on whether Node.js is installed
        let available = client.check_npx_available();
        println!("npx available: {}", available);
    }
}
