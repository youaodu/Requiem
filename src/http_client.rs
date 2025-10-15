use crate::models::{BodyType, HttpMethod, Request, Response};
use anyhow::Result;
use reqwest::Client;
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
    for (key, value) in response.headers() {
        if let Ok(v) = value.to_str() {
            headers_map.insert(key.to_string(), v.to_string());
        }
    }

    let body = response.text().await?;

    Ok(Response::new(
        status,
        status_text,
        headers_map,
        body,
        elapsed.as_millis(),
    ))
}
