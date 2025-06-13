mod extract;
mod chunk;
mod structs;
mod embedding;
mod store;
mod ollama;
use tokio::main;
use serde_json::{self, Value};
use std::io::BufReader;
use std::fs::File;



#[main]
async fn main() -> Result<() , Box<dyn std::error::Error>> {
    // extract content from the notes
    let notes = extract::extract_from_pdf(String::from("./DS_notes.pdf")).unwrap();

    let chunks = chunk::chunk_text(&notes, 500);
    
    //create vector collection if doesn't exist
    store::create_collection().await;

    // storing notes in qdrant
    for chunk in chunks.iter().enumerate(){
        let embedding = embedding::prepare_embedding(chunk.1).await?;
        store::upsert_vector(embedding, chunk.1.to_string()).await?;
    }

    //preparing questions
    let question_file = File::open("./exam_questions.json").unwrap();
    let reader = BufReader::new(question_file);

    let questions: Value = serde_json::from_reader(reader).unwrap();

    //preparing answers
    if let Some(questions_list) = questions["questions"].as_array() {
        for question in questions_list.iter(){
            let ques_embedding = embedding::prepare_embedding(&question["question"].as_str().unwrap()).await?;
            let search_result = store::search_vector(ques_embedding, 10).await?;
    
            let answer = ollama::ask_ollama(&question["question"].as_str().unwrap(), search_result).await?;
    
            println!("{}", answer);
            println!("\n\n================================");
        }     
    }else{
        println!("some error occured while parsing Questions");
    }

    Ok(())
}
