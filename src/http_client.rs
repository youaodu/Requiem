use crate::models::{BodyType, HttpMethod, KeyValue, Request, Response};
use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Instant;

/// Execute an HTTP request
pub async fn execute_request(request: &Request) -> Result<Response> {
    let start = Instant::now();

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    // Build URL with query parameters
    let mut url = reqwest::Url::parse(&request.url)?;
    for param in &request.query_params {
        if param.enabled {
            url.query_pairs_mut().append_pair(&param.key, &param.value);
        }
    }

    // Build request
    let mut req_builder = match request.method {
        HttpMethod::GET => client.get(url),
        HttpMethod::POST => client.post(url),
        HttpMethod::PUT => client.put(url),
        HttpMethod::PATCH => client.patch(url),
        HttpMethod::DELETE => client.delete(url),
        HttpMethod::HEAD => client.head(url),
        HttpMethod::OPTIONS => client.request(reqwest::Method::OPTIONS, url),
    };

    // Add headers
    for header in &request.headers {
        if header.enabled {
            req_builder = req_builder.header(&header.key, &header.value);
        }
    }

    // Add body
    req_builder = match &request.body {
        BodyType::None => req_builder,
        BodyType::Json(json) => req_builder.body(json.clone()),
        BodyType::Xml(xml) => req_builder.body(xml.clone()),
        BodyType::Text(text) => req_builder.body(text.clone()),
        BodyType::Binary(bytes) => req_builder.body(bytes.clone()),
        BodyType::FormUrlEncoded(fields) => {
            let form_data: Vec<(String, String)> = fields
                .iter()
                .filter(|kv| kv.enabled)
                .map(|kv| (kv.key.clone(), kv.value.clone()))
                .collect();
            req_builder.form(&form_data)
        }
        BodyType::FormData(fields) => {
            let mut form = reqwest::multipart::Form::new();
            for field in fields {
                if field.enabled {
                    form = form.text(field.key.clone(), field.value.clone());
                }
            }
            req_builder.multipart(form)
        }
    };

    // Send request
    let response = req_builder.send().await?;
    let elapsed = start.elapsed();

    // Extract response data
    let status = response.status().as_u16();
    let status_text = response.status().to_string();

    let mut headers_map = HashMap::new();
    let mut cookies = Vec::new();

    for (key, value) in response.headers() {
        if let Ok(v) = value.to_str() {
            // Extract cookies from Set-Cookie headers
            if key.as_str().to_lowercase() == "set-cookie" {
                // Parse cookie: extract name=value part before first semicolon
                if let Some(cookie_pair) = v.split(';').next() {
                    if let Some((name, value)) = cookie_pair.split_once('=') {
                        cookies.push(KeyValue::new(name.trim().to_string(), value.trim().to_string()));
                    }
                }
            }
            headers_map.insert(key.to_string(), v.to_string());
        }
    }

    let body = response.text().await?;

    Ok(Response::new(
        status,
        status_text,
        headers_map,
        cookies,
        body,
        elapsed.as_millis(),
    ))
}

/// OpenAI API request structures
#[derive(Debug, Serialize)]
struct OpenAIMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
}

/// OpenAI API response structures
#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    choices: Vec<OpenAIChoice>,
}

#[derive(Debug, Deserialize)]
struct OpenAIChoice {
    message: OpenAIMessageResponse,
}

#[derive(Debug, Deserialize)]
struct OpenAIMessageResponse {
    content: String,
}

/// Call OpenAI API to generate URL
pub async fn call_openai_api(
    api_url: &str,
    api_key: &str,
    model: &str,
    user_prompt: &str,
) -> Result<String> {
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    // Construct full API endpoint
    let endpoint = format!("{}/chat/completions", api_url.trim_end_matches('/'));

    // Build request body with system message
    let system_message = OpenAIMessage {
        role: "system".to_string(),
        content: "You are an API request generator assistant. Your task is to generate complete HTTP API request configurations based on user descriptions. \
                  \n\nRules:\n\
                  - Return ONLY a valid JSON object, no explanations or additional text\n\
                  - The JSON must contain these fields: method, url, headers, params, body\n\
                  - Use common API conventions (RESTful style)\n\
                  - Headers and params should be arrays of {\"key\": \"...\", \"value\": \"...\"} objects\n\
                  - Body should be a JSON string (empty string if no body needed)\n\
                  \n\nJSON Format:\n\
                  {\n\
                    \"method\": \"GET|POST|PUT|PATCH|DELETE\",\n\
                    \"url\": \"https://api.example.com/path\",\n\
                    \"headers\": [{\"key\": \"Content-Type\", \"value\": \"application/json\"}],\n\
                    \"params\": [{\"key\": \"page\", \"value\": \"1\"}],\n\
                    \"body\": \"{\\\"name\\\": \\\"value\\\"}\"\n\
                  }\n\
                  \n\nExample:\n\
                  User: \"Get user profile by ID 123\"\n\
                  You: {\"method\":\"GET\",\"url\":\"https://api.example.com/v1/users/123\",\"headers\":[],\"params\":[],\"body\":\"\"}\n\
                  \n\nUser: \"Create a new user with name John and email john@example.com\"\n\
                  You: {\"method\":\"POST\",\"url\":\"https://api.example.com/v1/users\",\"headers\":[{\"key\":\"Content-Type\",\"value\":\"application/json\"}],\"params\":[],\"body\":\"{\\\"name\\\":\\\"John\\\",\\\"email\\\":\\\"john@example.com\\\"}\"}".to_string(),
    };

    let user_message = OpenAIMessage {
        role: "user".to_string(),
        content: user_prompt.to_string(),
    };

    let request_body = OpenAIRequest {
        model: model.to_string(),
        messages: vec![system_message, user_message],
    };

    // Send request
    let response = client
        .post(&endpoint)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request_body)
        .send()
        .await?;

    // Check status
    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Err(anyhow::anyhow!("OpenAI API error {}: {}", status, error_text));
    }

    // Parse response
    let openai_response: OpenAIResponse = response.json().await?;

    // Extract generated text
    let generated_text = openai_response
        .choices
        .first()
        .map(|choice| choice.message.content.clone())
        .ok_or_else(|| anyhow::anyhow!("No response from OpenAI"))?;

    Ok(generated_text)
}
