use serde::{Deserialize};

#[derive(Deserialize)]
pub struct Question{
    pub ques: String,
    pub marks: usize
}