use std::{env, fs, io::{self, Write}, process::exit};
mod token;
mod scanner;
use scanner::Scanner;
fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    if args.len() > 2 {
        println!("Usage: cargo run [script]");
        exit(64);
    }
    else if args.len() == 2 {
        run_file(&args[1]);
    }
    else {
        run_prompt();
    }
}

fn run_file(path: &String){
    let bytes = fs::read_to_string(path)
        .expect("Should have been able to read the file");
    run(&bytes)
}

fn run_prompt(){
    let mut counter = 1;
    loop {
        print!("{counter}> ");
        io::stdout().flush().expect("Couldn't flush stdout");
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Couldn't read from stdin");
        let line = line.trim().to_string();
        if line == "" {
            break;
        }
        run(&line);
        counter = counter + 1;
    }
}

fn run(source: &String){
    let tokens: Vec<&str> = source.split('\n').collect();
    for token in tokens {
        println!("{token}");
    }
    let mut scanner = Scanner::new(String::from(source));
    let (tokens, errors) =  scanner.scan();
    for token in tokens{
        println!("{}", token)
    }
    for error in errors {
        eprintln!("{error}");
    }
}

fn error(line: u32, message: &String){
    report(line, message, &String::from(""));
}

fn report(line: u32, message: &String, location: &String){
    println!("[line {line}] Error {location}: {message}");
}