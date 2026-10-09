pub struct ErrorReporter{
    errors: Vec<String>
}

impl ErrorReporter {
    pub fn new() -> Self { Self { errors: vec![] } } 

    pub fn error(&mut self, line: u32, message: &str){
        self.report(line, message, &String::from(""));
    }

    pub fn report(&mut self, line: u32, message: &str, location: &str){
        if location == "" {
            self.errors.push(format!("[line {line}] Error {message}"))
        } else {
            self.errors.push(format!("[line {line}] Error {location}: {message}"))
        }
        
    }

    pub fn errors(&self) -> &[String] {
        &self.errors
    }
}

