use std::{env, fs, io::{self, Write}, process::exit};
mod token;
mod scanner;
mod error_reporter;
mod expr;
use scanner::Scanner;
use crate::{expr::Expr::{self, Binary, Grouping, Literal, Unary}, token::{Literal::Number, TokenType::{Minus, Star}}};
fn main() {
    let expression = Binary(
        Box::new(Unary(
            token::Token::new(Minus, String::from("-"), None, 1),
            Box::new(Literal(Number(123.0)))
        )),
        token::Token::new(Star, String::from("*"), None, 1),
        Box::new(Grouping(Box::new(Literal(Number(45.67)))))
    );
    println!("{}", expression);
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
    let scanner = Scanner::new(String::from(source));
    let mut error_reporter = error_reporter::ErrorReporter::new();
    let tokens =  scanner.scan(&mut error_reporter);
    for token in tokens{
        println!("{}", token)
    }
    for error in error_reporter.errors() {
        eprintln!("{error}");
    }
}