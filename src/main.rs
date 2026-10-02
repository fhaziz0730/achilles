use std::{env, fs, io::{self, Write}, process::exit};
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
    loop {
        print!("> ");
        io::stdout().flush().expect("Couldn't flush stdout");
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Couldn't read from stdin");
        if line == "" {
            break;
        }
        run(&line);
    }
}

fn run(source: &String){
    let tokens = ["token", source];
    for token in tokens {
        print!("{token}");
    }
}
