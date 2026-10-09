pub struct ErrorReporter{
    errors: Vec<String>
}

impl ErrorReporter {
    pub fn new() -> Self { Self { errors: vec![] } } 

    pub fn error(&mut self, line: u32, message: &str){
        self.report(line, message, &String::from(""));
    }

    pub fn report(&mut self, line: u32, message: &str, location: &str){
        println!("[line {line}] Error {location}: {message}");
        self.errors.push(format!("[line {line}] Error {location}: {message}"))
    }
}

