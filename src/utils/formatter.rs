/// Format JSON string with indentation
pub fn format_json(json_str: &str) -> Result<String, String> {
    match serde_json::from_str::<serde_json::Value>(json_str) {
        Ok(value) => {
            match serde_json::to_string_pretty(&value) {
                Ok(formatted) => Ok(formatted),
                Err(e) => Err(format!("Failed to format JSON: {}", e)),
            }
        }
        Err(e) => Err(format!("Invalid JSON: {}", e)),
    }
}

/// Format XML string with indentation
pub fn format_xml(xml_str: &str) -> Result<String, String> {
    // Simple XML formatting - add newlines and indentation
    let trimmed = xml_str.trim();
    if trimmed.is_empty() {
        return Err("Empty XML content".to_string());
    }

    // Basic XML formatting using regex-like approach
    let mut formatted = String::new();
    let mut indent_level: usize = 0;
    let mut in_tag = false;
    let mut current_tag = String::new();

    let chars: Vec<char> = trimmed.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];

        if c == '<' {
            in_tag = true;
            current_tag.clear();

            // Check if it's a closing tag
            if i + 1 < chars.len() && chars[i + 1] == '/' {
                indent_level = indent_level.saturating_sub(1);
                formatted.push('\n');
                formatted.push_str(&"  ".repeat(indent_level));
            } else if !formatted.is_empty() && formatted.chars().last() != Some('\n') {
                formatted.push('\n');
                formatted.push_str(&"  ".repeat(indent_level));
            }

            formatted.push(c);
        } else if c == '>' {
            in_tag = false;
            formatted.push(c);

            // Check if it's not a self-closing tag or XML declaration
            if !current_tag.ends_with('/') && !current_tag.starts_with('?') && !current_tag.starts_with('/') {
                indent_level += 1;
            }
            current_tag.clear();
        } else {
            if in_tag {
                current_tag.push(c);
            }
            formatted.push(c);
        }

        i += 1;
    }

    Ok(formatted)
}

/// Format HTML string with indentation
pub fn format_html(html_str: &str) -> Result<String, String> {
    // For HTML, we can use similar logic to XML
    // In a production app, you might want to use a proper HTML parser
    format_xml(html_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_json() {
        let json = r#"{"name":"test","value":123}"#;
        let result = format_json(json);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("  "));
    }

    #[test]
    fn test_format_invalid_json() {
        let json = r#"{"name":"test""#;
        let result = format_json(json);
        assert!(result.is_err());
    }
}
