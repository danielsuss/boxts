use reqwest::Client;
use serde::Serialize;

#[derive(Serialize)]
struct TextPayload {
    text: String,
}

#[derive(Serialize)]
struct TrainModelPayload {
    filepath: String,
}

const SERVER_BASE_URL: &str = "http://127.0.0.1:8000";

pub async fn send_speak_request(text: String) -> Result<String, String> {
    let client = Client::new();
    let url = format!("{}/speak", SERVER_BASE_URL);
    
    let payload = TextPayload { text };
    
    match client.post(&url).json(&payload).send().await {
        Ok(response) => match response.text().await {
            Ok(body) => Ok(body),
            Err(_) => Err("Failed to read response body".to_string()),
        },
        Err(e) => Err(format!("Request failed: {}", e)),
    }
}

pub async fn send_train_model_request(filepath: String) -> Result<String, String> {
    let client = Client::new();
    let url = format!("{}/trainmodel", SERVER_BASE_URL);
    
    let payload = TrainModelPayload { filepath };
    
    match client.post(&url).json(&payload).send().await {
        Ok(response) => match response.text().await {
            Ok(body) => Ok(body),
            Err(_) => Err("Failed to read response body".to_string()),
        },
        Err(e) => Err(format!("Request failed: {}", e)),
    }
}

