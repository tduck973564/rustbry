mod scanner;

use rustyline::Editor;
use std::{env, error::Error, fs, process};

fn repl() {
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => interpret(line),
            Err(_) => println!("\n"),
        }
    }
}

fn run_file(path: &str) -> Result<(), Box<dyn Error>> {
    let source = fs::read_to_string(path)?;
    match interpret(source) {
        Ok(_) => (),
        Err(x) => process::exit(65),
    }
    Ok(())
}

fn main() {
    init_vm();

    let args: Vec<String> = env::args().collect();
    if let Some(x) = args.get(1) {
        run_file(x).expect("File not found.");
        process::exit(64);
    } else {
        repl();
    }

    free_vm();
}
