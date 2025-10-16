# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Requiem** is a lightweight, high-performance HTTP client built with Rust and iced. It targets 50-100MB memory usage (vs Postman's 300-500MB) with <1s startup time. The project aims to be a developer-friendly alternative to Postman with offline-first design, Git-friendly text storage, and native performance.

**Tech Stack:**
- Language: Rust 2021
- UI Framework: iced 0.13 (native Elm-architecture GUI)
- HTTP Client: reqwest 0.12 with tokio async runtime
- Syntax Highlighting: syntect 5

## Build & Development Commands

### Basic Commands
```bash
# Run in development mode (most common)
cargo run

# Build for release
cargo build --release

# Run tests
cargo test

# Check code without building
cargo check

# Format code
cargo fmt

# Lint with clippy
cargo clippy
```

### Debugging
```bash
# Run with debug logs
RUST_LOG=debug cargo run

# Run with full backtrace
RUST_BACKTRACE=1 cargo run

# Combined debug mode
RUST_LOG=trace RUST_BACKTRACE=full cargo run

# Use the debug script
./debug.sh run      # Standard debug mode
./debug.sh trace    # Full trace mode
```

### Performance Analysis
```bash
# Memory profiling with heaptrack
./debug.sh mem

# CPU profiling with perf
./debug.sh perf
```

## Architecture

### Module Structure

The codebase follows a clean modular architecture:

```
src/
├── main.rs              # Entry point, iced app setup
├── i18n.rs              # Internationalization support (English/Chinese)
├── http_client.rs       # HTTP request execution (reqwest wrapper)
├── ai_client.rs         # AI integration via Agent Client Protocol (ACP)
├── models/              # Core data models
│   ├── mod.rs
│   ├── ai_config.rs     # AI engine configuration
│   ├── body.rs          # Request body types (JSON, Form, etc.)
│   ├── collection.rs    # Collection and folder structures
│   ├── environment.rs   # Environment variables
│   ├── http_method.rs   # HTTP methods enum
│   ├── key_value.rs     # Key-value pairs (headers, params)
│   ├── request.rs       # Request model
│   ├── request_tab.rs   # Request tab types
│   ├── response.rs      # Response model
│   └── response_tab.rs  # Response tab types
├── app/                 # Application layer
│   ├── mod.rs
│   ├── state.rs         # Application state (Requiem struct)
│   ├── message.rs       # Message enum (Elm architecture)
│   └── update.rs        # State updates and business logic
├── ui/                  # User interface
│   ├── mod.rs
│   ├── view.rs          # Main view composition
│   ├── request_editor.rs    # Request configuration UI
│   ├── request_list.rs      # Collection/sidebar UI
│   ├── request_tabs.rs      # Tab management UI
│   ├── response_viewer.rs   # Response display UI
│   ├── toast.rs         # Toast notification component
│   ├── underline_input.rs   # Custom input component
│   ├── body_highlighter.rs  # Syntax highlighting
│   └── components/      # Reusable UI components
│       ├── ai_engine_picker.rs   # AI engine selector
│       ├── ai_fill_dialog.rs     # AI fill dialog
│       ├── code_editor.rs
│       ├── context_menu.rs
│       ├── environment_dialog.rs
│       ├── environment_picker.rs
│       ├── language_picker.rs
│       ├── method_picker.rs
│       ├── settings_dialog.rs
│       ├── tabs_bar.rs
│       └── textarea.rs
└── utils/               # Utility functions
    ├── mod.rs
    └── navigation.rs    # Navigation helpers
```

### State Management (Elm Architecture)

The app uses iced's Elm architecture pattern:

1. **State** (`app/state.rs`): Contains all application state in the `Requiem` struct
2. **Message** (`app/message.rs`): Enum of all possible user interactions and events
3. **Update** (`app/update.rs`): Pure function that transforms state based on messages
4. **View** (`ui/view.rs`): Renders UI from current state

Example flow:
```
User clicks "Send" → Message::SendRequest
→ update() executes HTTP request
→ Message::RequestCompleted(response)
→ update() stores response in state
→ view() re-renders with new response
```

### Key Data Models

- **Request** (`models/request.rs`): HTTP request configuration (method, URL, headers, body, params, auth, cookies)
- **Response** (`models/response.rs`): HTTP response data (status, headers, body, timing)
- **Collection** (`models/collection.rs`): Hierarchical structure of requests and folders
- **CollectionItem**: Enum of Request or Folder (tree structure)
- **BodyType** / **BodyFormat** (`models/body.rs`): JSON, Form-urlencoded, Multipart, Raw, Binary, GraphQL, Msgpack
- **HttpMethod** (`models/http_method.rs`): GET, POST, PUT, PATCH, DELETE, HEAD, OPTIONS
- **Environment** (`models/environment.rs`): Environment configuration with variables
- **KeyValue** (`models/key_value.rs`): Generic key-value pairs for headers, params, cookies, auth
- **RequestTab** / **ResponseTab**: Tab types for organizing UI tabs

### UI Components

The UI is built with reusable components in `ui/components/`:
- **code_editor**: Syntax-highlighted code editor for request/response bodies
- **context_menu**: Right-click context menu for requests/folders
- **environment_dialog**: Dialog for managing environments and variables
- **environment_picker**: Dropdown to select active environment
- **language_picker**: Language selection dropdown (i18n)
- **method_picker**: HTTP method selector (GET, POST, etc.)
- **settings_dialog**: Application settings dialog
- **tabs_bar**: Tab bar for managing multiple request tabs
- **textarea**: Multi-line text input component

Key UI state includes:
- **DragState**: Tab drag-and-drop state tracking
- **TabPressState**: Mouse press state for tab interactions
- **ContextMenu**: Context menu display state and position

## Development Guidelines

### Internationalization (i18n)

The app supports multiple languages (currently English and Chinese) through the i18n module:
- Language configuration in `src/i18n.rs`
- Translation files should be loaded from JSON (with fallback strings in code)
- UI strings accessible via `Translations::get(key)` method
- Switch languages through Settings dialog

**IMPORTANT**: When adding new UI text or modifying existing strings:
1. Add the translation key to `src/i18n.rs` in both `english_strings()` and `chinese_strings()` functions
2. Update both JSON translation files:
   - `locales/en.json` (English)
   - `locales/zh.json` (Chinese)
3. Ensure all three locations are kept in sync to avoid missing translations

### Font Configuration

The app uses Source Han Sans CN for Chinese character support, loaded from:
```
/usr/share/fonts/adobe-source-han-sans/SourceHanSansCN-Regular.otf
```

If building on a system without this font, either:
1. Install adobe-source-han-sans-otf-fonts package (Arch Linux)
2. Update font path in `src/main.rs:30`
3. Comment out font loading (lines 30-31) for default system font

### Logging

Use tracing for structured logging:
```rust
use tracing::{info, debug, warn, error};

info!("Starting request");
debug!("Request body: {:?}", body);
```

Control log level with `RUST_LOG` environment variable.

### VS Code Debugging

Press F5 to launch debugger (requires rust-analyzer and CodeLLDB extensions).
Configuration in `.vscode/launch.json` sets `RUST_LOG=debug` automatically.

### Performance Targets

Keep in mind the project's performance goals:
- Memory usage: Target <100MB, max 200MB
- Startup time: Target <1s
- Binary size: Target <30MB, max 50MB (release build with strip=true)

Monitor with `./debug.sh mem` and `./debug.sh perf`.

## Project Status

Currently in MVP phase (v0.1.0) with core HTTP client functionality implemented:
- ✅ Request editor with method, URL, headers, query params, body support
- ✅ Multiple body formats (JSON, Form-data, Raw text)
- ✅ Collection/folder organization with drag-and-drop
- ✅ Tab-based request management
- ✅ Response viewer with syntax highlighting
- ✅ Environment variables support
- ✅ Authentication (Bearer, Basic, API Key)
- ✅ Context menus for request/folder operations
- ✅ Settings dialog with language selection
- ✅ Toast notifications
- ✅ Keyboard shortcuts
- ✅ Internationalization (English/Chinese)

See `idea.md` for full feature roadmap and architectural details.

## Debugging Rules for Claude Code

**IMPORTANT**: When working on this project and needing to run or test the application:

1. **NEVER run the application automatically** - Always provide the command for the user to run manually
2. **Provide clear, ready-to-copy commands** - Format commands in code blocks
3. **Explain what to test** - Tell the user what behavior to verify after running

### Example Workflow

When implementing a feature that requires testing:

```markdown
I've implemented the feature. Please test it by running:

​```bash
cargo run
​```

Then verify:
- Click on the X button to test...
- Check that the Y behavior works...
```

### Common Test Commands to Provide

```bash
# Standard run
cargo run

# With debug logging
RUST_LOG=debug cargo run

# Clean build and run
cargo clean && cargo run

# Release mode (for performance testing)
cargo build --release && ./target/release/requiem
```

## Arch Linux Packaging (PKGBUILD)

For Arch Linux users, here's a PKGBUILD template for creating a package:

```bash
# Maintainer: Your Name <you@example.com>
pkgname=requiem
pkgver=0.1.0
pkgrel=1
pkgdesc="A lightweight, high-performance HTTP client built with Rust and iced"
arch=('x86_64')
url="https://github.com/yourusername/requiem"
license=('MIT')
depends=('gcc-libs' 'fontconfig' 'adobe-source-han-sans-otf-fonts')
makedepends=('rust' 'cargo')
source=("$pkgname-$pkgver.tar.gz::https://github.com/yourusername/$pkgname/archive/v$pkgver.tar.gz")
sha256sums=('SKIP')  # Update with actual checksum

build() {
  cd "$pkgname-$pkgver"
  cargo build --release --locked
}

check() {
  cd "$pkgname-$pkgver"
  cargo test --release --locked
}

package() {
  cd "$pkgname-$pkgver"

  # Install binary
  install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"

  # Install license
  install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"

  # Install desktop file (if available)
  # install -Dm644 "$pkgname.desktop" "$pkgdir/usr/share/applications/$pkgname.desktop"

  # Install icon (if available)
  # install -Dm644 "assets/icon.png" "$pkgdir/usr/share/pixmaps/$pkgname.png"
}
```

### Building the Package

```bash
# Build the package
makepkg -si

# Build without installing
makepkg

# Clean build
makepkg -c

# Update checksums
updpkgsums
```

### Runtime Dependencies

The package depends on:
- `gcc-libs`: C runtime libraries
- `fontconfig`: Font configuration (required by iced)
- `adobe-source-han-sans-otf-fonts`: Chinese font support (see Font Configuration section)

To check actual runtime dependencies after building:

```bash
ldd target/release/requiem
```

## AI Integration (Claude Code / Codex)

Requiem supports AI-powered features through the Agent Client Protocol (ACP), allowing integration with Claude Code and Codex agents.

### Architecture

The AI integration is implemented through:
- **`src/ai_client.rs`**: ACP client that manages AI agent processes
- **`src/models/ai_config.rs`**: AI engine configuration (OpenAI, Claude Code, Codex)
- **`agent-client-protocol`** crate: Rust SDK for ACP communication

### Prerequisites

To use Claude Code or Codex engines, you need Node.js installed:

```bash
# Check if Node.js is available
npx --version

# On Arch Linux
sudo pacman -S nodejs npm

# On Ubuntu/Debian
sudo apt install nodejs npm
```

### How It Works

1. **Agent Process**: Requiem spawns an AI agent as a subprocess using `npx`:
   - Claude Code: `npx @zed-industries/claude-code-acp`
   - Codex: `npx @zed-industries/codex-acp`

2. **ACP Communication**: The agent communicates with Requiem via stdin/stdout using JSON-RPC

3. **Client Implementation**: `RequiemAcpClient` implements the ACP `Client` trait to handle:
   - Permission requests (auto-approved for now)
   - File read/write operations
   - Session notifications (progress updates)

### Current Status

**Implemented:**
- ✅ ACP client infrastructure
- ✅ Agent process management (start/stop)
- ✅ File system access for agents
- ✅ AI engine configuration model

**TODO:**
- ⏸ Full ACP connection setup with `ClientSideConnection`
- ⏸ UI integration for AI features
- ⏸ User permission dialog
- ⏸ Streaming response display

### Usage (When Complete)

```rust
use crate::ai_client::AiClient;
use crate::models::AiEngine;

// Create AI client
let mut client = AiClient::new(AiEngine::ClaudeCode);

// Start the agent
client.start().await?;

// Send a prompt
let response = client.send_prompt("Generate a GET request for https://api.github.com/users/octocat").await?;

// Stop the agent
client.stop().await?;
```

### PKGBUILD Considerations

For Arch Linux packaging with AI features:

```bash
# Add optional dependencies
optdepends=(
  'nodejs: Required for Claude Code and Codex AI features'
  'npm: Required for Claude Code and Codex AI features'
)
```

**Note:** AI features are optional. The application works without Node.js installed, but AI functionality will be disabled.

### Performance Impact

- **Without AI**: No impact, AI client is not initialized
- **With AI active**:
  - Additional ~50-100MB for Node.js runtime
  - Agent process memory varies by engine
  - Still within acceptable range for power users

### Security Considerations

Currently, the `RequiemAcpClient` implementation:
- Auto-approves file read/write permissions (for development)
- Allows file access to any path requested by the agent
- Does not execute shell commands

**Production TODO:**
- Implement user permission dialogs
- Restrict file access to project directories
- Add audit logging for agent actions
