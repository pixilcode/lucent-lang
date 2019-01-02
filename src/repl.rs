use std::io::{self, Write};

use lucent_lang::scanner::{self, TokenType};
use lucent_lang::virtual_machine::VM;

pub fn run() {
    let _vm = VM::new();

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut code = String::new();
        match io::stdin().read_line(&mut code) {
            Ok(_) => (),
            Err(_) => eprintln!("\nError reading input"),
        };
        let code = code.trim();

        if code == ":quit" {
            break;
        }

        // Check to see if it is a complete statement
        // and if it doesn't end in a semicolon, add one
        
        let mut tokens = scanner::build_scanner(code);
        
        loop {
			if tokens.current_token().token_type() == TokenType::EOF {
				break;
			}
			
			println!("{:?}", tokens.current_token());
			tokens = tokens.scan_token();
        }
        //let chunks = compiler::compile(tokens);
        //let result = vm.interpret(chunks);
    }
}
