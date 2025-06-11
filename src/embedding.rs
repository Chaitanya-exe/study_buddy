use reqwest::Client;
use serde_json::{json, Value};

pub async fn prepare_embedding(chunk: &str) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
    let client = Client::new();

    let request_body = json!({
        "model":"nomic-embed-text",
        "prompt": chunk
    });

    let response = client
        .post("http://localhost:11434/api/embeddings")
        .json(&request_body)
        .send()
        .await?
        .json::<Value>()
        .await?;

    let embeddings = response["embedding"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_f64().unwrap() as f32)
        .collect();


    Ok(embeddings   )
}