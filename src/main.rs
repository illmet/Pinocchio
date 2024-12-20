use std::env;
use serde_json::{json, Value};
use reqwest;

async fn generate_gemini(api_key: &str, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("GEMINI_API_KEY")
        .expect("GEMINI_API_KEY environment variable not found");
    
    let prompt = "Write a story about a magic backpack.";
    let response = generate_gemini(&api_key, prompt).await?;
    println!("{}", response);
    
    Ok(())
}
