mod scanner;
mod utils;

use clap::{arg, command, value_parser, Command};
use scanner::lexer::delta;
use std::{io::Read, path::PathBuf};

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

        if let Some(file) = lexer_matches.get_one::<PathBuf>("file") {
            let mut file = std::fs::File::open(file).unwrap();
            file.read_to_string(&mut input).unwrap();
        } else {
            std::io::stdin().read_to_string(&mut input).unwrap();
        }

        let mut lexer = Lexer::new(&input, delta, &[2, 3]);
        let tokens = lexer.lex();

        print_tokens(tokens);
    }
}

fn print_tokens(tokens: Vec<Token>) {
    println!("Tokens:\n");
    for token in tokens {
        println!("{:?}", token);
    }
}
