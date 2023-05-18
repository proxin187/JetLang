mod lexer;
mod ast;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::env;
use std::process;


fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("Err: Not enough arguments");
        process::exit(1);
    }
    let fd = File::open(&args[1]);
    if fd.is_err() {
        println!("Err: Failed to open '{}': {:?}", &args[1], fd.unwrap_err());
        process::exit(1);
    }
    let reader = BufReader::new(fd.unwrap());

    let mut line_buffer: String = String::new();
    let mut indentation_counter: usize = 0;

    for byte in reader.bytes() {
        if byte.is_err() {
            println!("Err: failed to read byte");
            process::exit(1);
        }
        let character = String::from_utf8(vec![byte.unwrap()]).unwrap();
        if &character == "\n" && indentation_counter == 0 {
            line_buffer = line_buffer + &character;
            let tokens = lexer::tokenize(&line_buffer);
            println!("Tokens: {:?}", tokens);
            let tree = ast::build_ast(tokens);
            println!("Ast: {:?}", tree);
            line_buffer = String::new();
            continue;
        } else if &character == "{" {
            indentation_counter += 1;
        } else if &character == "}" {
            indentation_counter -= 1;
        }
        line_buffer = line_buffer + &character;
    }
}


