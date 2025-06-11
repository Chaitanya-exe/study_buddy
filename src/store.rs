use reqwest::Client;
use serde_json::{Value, json};
use uuid::Uuid;

pub async fn create_collection() -> Option<bool> {
    let client = Client::new();

    let request_body = json!({
        "vectors":{
            "size": 768,
            "distance": "Cosine"
        }
    });

    let response = client
        .put("http://localhost:6333/collections/notes")
        .json(&request_body)
        .send()
        .await;

    match response {
        Ok(res) => {
            if !res.status().is_success() {
                println!("collection already exists");
                None
            } else {
                println!("collection created successfully");
                Some(true)
            }
        }
        Err(e) => {
            println!("some error occured creating collection: {}", e);
            None
        }
    }
}

pub async fn upsert_vector(vector: Vec<f32>, text: String) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let id = Uuid::new_v4().to_string();
    let request_body = json!({
        "points":[{
            "id" : &id,
            "vector": &vector,
            "payload" : { "text": &text }
        }]
    });

    let response = client
        .put("http://localhost:6333/collections/notes/points")
        .json(&request_body)
        .send()
        .await;
    
    match response {
        Ok(res) => {
            if !res.status().is_success(){
                return Err(format!("problem upserting vector: {}", res.text().await?).into())
            } 
            println!("vector upserted successfully");
            Ok(())
        }
        Err(e) => {
            println!("some error occured upserting: {}", e);
            Err(e.into())
        }
    }

}

pub async fn search_vector(query: Vec<f32>, top_k: usize) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let client = Client::new();

    let request_body = json!({
        "vector": query,
        "top": top_k,
        "with_payload": true
    });

    let response = client
        .post("http://localhost:6333/collections/notes/points/search")
        .json(&request_body)
        .send()
        .await;

    match response{
        Ok(res) => {
            let res = res.json::<Value>().await?;
            let texts = res["result"]  
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v["payload"]["text"].as_str().unwrap().to_string())
                .collect();
            Ok(texts)
        }
        Err(e) => {
            Err(format!("error occured while searching: {}",e).into())
        } 
    }
}