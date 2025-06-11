use pdf_extract;

pub fn extract_from_pdf(path: String) -> Result<String, Box<dyn std::error::Error>>{
    let text = pdf_extract::extract_text(path);

    match text{
        Ok(content) => {
            println!("Pdf extracted successfully.");
            Ok(content)
        },
        Err(e) => {
            println!("Some error occured while extracting: {}", e);
            return Err(e.into());
        }
    }
}