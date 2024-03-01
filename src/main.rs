mod errors;
mod scanner;
mod utils;

use clap::{arg, command, value_parser, Command};
use std::{io::Read, path::PathBuf};
use utils::SimpleBuffer;

use crate::scanner::{Lexer, Token};

fn main() {
    let matches = command!()
        .subcommand(
            Command::new("lexer")
                .about("Run the lexer on a file, or standard input")
                .arg(
                    arg!(-f --file <FILE> "The file to lex")
                        .value_parser(value_parser!(PathBuf))
                        .required(false),
                )
                .arg(arg!(-t --tokens "Print tokens")),
        )
        .get_matches();

    if let Some(lexer_matches) = matches.subcommand_matches("lexer") {
        let mut input = String::new();

        if let Some(file_path) = lexer_matches.get_one::<PathBuf>("file") {
            let mut file = std::fs::File::open(file_path);

            match file {
                Ok(ref mut file) => file.read_to_string(&mut input).unwrap(),
                Err(_) => {
                    eprintln!(
                        "Could not open file {}.",
                        file_path.as_os_str().to_str().unwrap()
                    );
                    std::process::exit(1);
                }
            };
        } else {
            match std::io::stdin().read_to_string(&mut input) {
                Ok(_) => {}
                Err(_) => {
                    eprintln!("Could not read from standard input.");
                    std::process::exit(1);
                }
            }
        }

        let mut lexer: Lexer<SimpleBuffer> = Lexer::new(&input);
        let tokens = lexer.lex();

        print_tokens(tokens);
    }
}

fn print_tokens(tokens: Vec<Token>) {
    for token in tokens {
        println!("{:?}", token);
    }
}
