use reqwest::Client;
use serde::Serialize;

#[derive(Serialize)]
struct TextPayload {
    text: String,
}

#[derive(Serialize)]
struct CloneVoicePayload {
    filepath: String,
}

#[derive(Serialize)]
struct StartPayload {
    voice: String,
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

pub async fn send_clonevoice_request(filepath: String) -> Result<String, String> {
    let client = Client::new();
    let url = format!("{}/clonevoice", SERVER_BASE_URL);
    
    let payload = CloneVoicePayload { filepath };
    
    match client.post(&url).json(&payload).send().await {
        Ok(response) => match response.text().await {
            Ok(body) => Ok(body),
            Err(_) => Err("Failed to read response body".to_string()),
        },
        Err(e) => Err(format!("Request failed: {}", e)),
    }
}

pub async fn send_start_request(voice: String) -> Result<String, String> {
    let client = Client::new();
    let url = format!("{}/start", SERVER_BASE_URL);
    
    let payload = StartPayload { voice };
    
    match client.post(&url).json(&payload).send().await {
        Ok(response) => match response.text().await {
            Ok(body) => Ok(body),
            Err(_) => Err("Failed to read response body".to_string()),
        },
        Err(e) => Err(format!("Request failed: {}", e)),
    }
}

pub async fn send_volume_request() -> Result<String, String> {
    let client = Client::new();
    let url = format!("{}/volume", SERVER_BASE_URL);
    
    match client.post(&url).send().await {
        Ok(response) => match response.text().await {
            Ok(body) => Ok(body),
            Err(_) => Err("Failed to read response body".to_string()),
        },
        Err(e) => Err(format!("Request failed: {}", e)),
    }
}

pub async fn send_listdevices_request() -> Result<String, String> {
    let client = Client::new();
    let url = format!("{}/listdevices", SERVER_BASE_URL);
    
    match client.post(&url).send().await {
        Ok(response) => match response.text().await {
            Ok(body) => Ok(body),
            Err(_) => Err("Failed to read response body".to_string()),
        },
        Err(e) => Err(format!("Request failed: {}", e)),
    }
}

pub async fn send_stop_request() -> Result<String, String> {
    let client = Client::new();
    let url = format!("{}/stop", SERVER_BASE_URL);
    
    match client.post(&url).send().await {
        Ok(response) => match response.text().await {
            Ok(body) => Ok(body),
            Err(_) => Err("Failed to read response body".to_string()),
        },
        Err(e) => Err(format!("Request failed: {}", e)),
    }
}

pub async fn send_changevoice_request(voice: String) -> Result<String, String> {
    let client = Client::new();
    let url = format!("{}/changevoice", SERVER_BASE_URL);
    
    let request_body = serde_json::json!({
        "voice": voice
    });
    
    match client.post(&url).json(&request_body).send().await {
        Ok(response) => match response.text().await {
            Ok(body) => Ok(body),
            Err(_) => Err("Failed to read response body".to_string()),
        },
        Err(e) => Err(format!("Request failed: {}", e)),
    }
}

pub async fn send_ready_request() -> Result<String, String> {
    let client = Client::new();
    let url = format!("{}/ready", SERVER_BASE_URL);
    
    match client.post(&url).send().await {
        Ok(response) => match response.text().await {
            Ok(body) => Ok(body),
            Err(_) => Err("Failed to read response body".to_string()),
        },
        Err(e) => Err(format!("Request failed: {}", e)),
    }
}

pub async fn send_outputdevice_request() -> Result<String, String> {
    let client = Client::new();
    let url = format!("{}/outputdevice", SERVER_BASE_URL);
    
    match client.post(&url).send().await {
        Ok(response) => match response.text().await {
            Ok(body) => Ok(body),
            Err(_) => Err("Failed to read response body".to_string()),
        },
        Err(e) => Err(format!("Request failed: {}", e)),
    }
}

