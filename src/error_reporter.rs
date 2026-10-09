pub struct ErrorReporter{
    errors: Vec<String>
}

impl ErrorReporter {
    pub fn new() -> Self { Self { errors: vec![] } } 

    pub fn error(&self, line: u32, message: &String){
        self.report(line, message, &String::from(""));
    }

    pub fn report(&self, line: u32, message: &String, location: &String){
        println!("[line {line}] Error {location}: {message}");
    }
}

