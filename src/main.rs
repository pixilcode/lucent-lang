extern crate clox;

mod repl;

use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        repl::run();
    } else if args.len() == 2 {
        // runFile(args[0]);
    } else {
        eprintln!("Usage: clox [path]");
        process::exit(64);
    }
}
