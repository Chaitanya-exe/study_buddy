use reqwest::Client;
use serde_json::{Value, json};

pub async fn ask_ollama(user_question: &str, context: Vec<String>) -> Result<String, Box<dyn std::error::Error>>{
    let client = Client::new();

    let prompt = format!(
        "Use the following context from study notes to answer the following question and answer very thoroughly using full knowledge from notes, in such a that the answer is suitable for exams:\n\n{}\n\nQuestion: {}\nAnswer:",
        context.join("\n---\n"),
        user_question
    );

    let request_body = json!({
        "model":"llama3.2",
        "messages":[{
            "role":"user",
            "content": &prompt
        }],
        "stream":false
    });


    let response  = client
    .post("http://localhost:11434/api/chat")
    .json(&request_body)
        .send()
        .await?
        .json::<Value>()
        .await?;

    if let Some(content) = response["message"]["content"].as_str(){
        let content = content.to_string();
        Ok(content)
    } else {
        Err(format!("some error occured while answering").into())
    }
}