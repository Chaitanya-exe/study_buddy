pub fn chunk_text(text: &str, max_len: usize) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut current = String::new();

    for sentence in text.split_terminator('.'){
        if current.len() + sentence.len() > max_len {
            chunks.push(current.clone());
            current.clear();
        }
        current.push_str(sentence);
        current.push('.');
    }
    if !current.is_empty(){
        chunks.push(current);
    }

    chunks
}