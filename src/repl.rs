use std::io::{self, Write};

use clox::virtual_machine::VM;

pub fn run() {
    let vm = VM::new();

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut code = String::new();
        match io::stdin().read_line(&mut code) {
            Ok(_) => (),
            Err(_) => eprintln!("\nError reading input"),
        };
        let code = code.trim();

        if code == "quit()" {
            break;
        }

        // Check to see if it is a complete statement
        // and if it doesn't end in a semicolon, add one
        
        let tokens = scanner.build_scanner(code);
        let chunks = compiler.compile(tokens);
        let result = vm.interpret(chunks);
    }
}
