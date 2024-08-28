pub struct Tokenizer{
    file_content : String
}

impl Tokenizer {
    pub fn new(&self, file_content : &str) -> Tokenizer {
        return Tokenizer{file_content:"".to_owned()};
    }
}