extern crate lucent_lang;

mod repl;

use std::{env, process, fs};

use lucent_lang::scanner;
use lucent_lang::compiler;
use lucent_lang::virtual_machine::{VM, VMResult};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        repl::run();
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        eprintln!("Usage: clox [path]");
        process::exit(64);
    }
}

fn run_file(path: &str) {
    let source = read_file(path);
    let vm = VM::new();
    
    let scanner = scanner::build_scanner(&source);
    let chunk = compiler::compile(scanner);
    let result = vm.interpret(&chunk);
    
    match result {
        VMResult::Okay(_) => process::exit(0),
        VMResult::CompileError => process::exit(65),
        VMResult::RuntimeError => process::exit(70),
    }
}

fn read_file(_path: &str) -> String {
    let code = String::new();
    fs::read_to_string(&code).unwrap();
    code
}
