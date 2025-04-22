use crate::{fetch_code, utils};
use reqwest::Client;
use serde_json::json;
pub async fn request_api(user_query: &str,api_key:&str,main_prompt:&str,model_name:&str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let files = fetch_code::get_all_files(".");
    let client = Client::new();
    let response = match client
        .post("https://openrouter.ai/api/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!({
            "model":model_name,
            "messages":[
                {
                    "role": "system",
                    "content": format!(
                        "{} {}", main_prompt,
                        files.join(", ")
                    )
                    
                },                  
                {
                    "role":"user",
                    "content":user_query
                }
            ]
        }))
        .send()
        .await {
            Ok(res) => res,
            Err(e) => return Err(format!("API request failed: {}", e).into()),
        };

    let response_json = match response.json::<serde_json::Value>().await {
        Ok(json) => json,
        Err(e) => return Err(format!("Failed to parse API response as JSON: {}", e).into()),
    };
    let content = match response_json["choices"][0]["message"]["content"].as_str() {
        Some(content) => content,
        None => return Err("No content in API response".into()),
    };
    match utils::extract_json(content) {
        Some(json_str) => match serde_json::from_str::<serde_json::Value>(json_str) {
            Ok(json_value) => {
                if let Some(folder_paths) = json_value.get("folder_paths") {
                    let paths = folder_paths
                        .as_array()
                        .unwrap_or(&Vec::new())
                        .iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect();
                    utils::create_directory_from_arr(&paths);
                }

                if let Some(folder_paths) = json_value.get("file_paths") {
                    let paths = folder_paths
                        .as_array()
                        .unwrap_or(&Vec::new())
                        .iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect();
                    utils::create_files_from_arr(&paths);
                }
                if let Some(file_changes) = json_value.get("file_changes").and_then(|v| v.as_array()) {
                    for file_change in file_changes {
                        if let Some(file_path) = file_change.get("file_path").and_then(|v| v.as_str()) {
                            if let Some(changes) = file_change.get("changes").and_then(|v| v.as_array()) {
                                for change in changes {
                                    if let Some(line_no) = change.get("line_no").and_then(|v| v.as_u64()) {
                                        if let Some(change_type) = change.get("type").and_then(|v| v.as_str()) {
                                            if let Some(content) = change.get("content").and_then(|v| v.as_str()) {
                                                let _ = utils::modify_line_in_file(file_path, line_no, content, change_type);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                if let Some(message) = json_value.get("message") {
                    println!("{}", message);
                }
                let read_files_paths = match json_value.get("read_files") {
                    Some(read_files) => read_files
                        .as_array()
                        .unwrap_or(&Vec::new())
                        .iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect::<Vec<String>>(),
                    None => Vec::new(),
                };
                
                Ok(json!(read_files_paths))
            },
            Err(e) => Err(format!("Failed to parse extracted JSON: {}", e).into()),
        },
        None => Err("Failed to extract valid JSON from response".into()),
    }
}
