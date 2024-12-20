use serde_json::{json, Value};
use reqwest;

pub async fn generate_gemini(api_key: &str, prompt: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let client = reqwest::Client::new();
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash-exp:generateContent?key={}",
        api_key
    );
    let payload = json!({
        "contents": [{
            "parts": [{
                "text": prompt
            }]
        }]
    });
    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?
        .json::<Value>()
        .await?;
    let text = response["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .ok_or("Failed to extract text from response")?
        .to_string();
    Ok(text)
}
