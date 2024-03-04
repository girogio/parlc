mod core;
mod scanner;
mod utils;

use clap::{command, value_parser, Arg, Command};
use console::style;
use std::{io::Read, path::PathBuf};
use utils::SimpleBuffer;

use crate::scanner::Lexer;

fn main() {
    let matches = command!()
        .subcommand(
            Command::new("lex")
                .about("Run the lexer on a file")
                .arg(Arg::new("file").value_parser(value_parser!(PathBuf))),
        )
        .get_matches();

    if let Some(lexer_matches) = matches.subcommand_matches("lex") {
        let mut input = String::new();

        if let Some(file_path) = lexer_matches.get_one::<PathBuf>("file") {
            let mut file = std::fs::File::open(file_path);

            match file {
                Ok(ref mut file) => file.read_to_string(&mut input).unwrap(),
                Err(_) => {
                    let msg = style("File not found:").red().bold().for_stderr();
                    eprintln!("{} `{}`...", msg, style(file_path.display()).cyan());
                    std::process::exit(1);
                }
            };
        }

        println!(
            "\n{} `{}`\n",
            style("Lexing").green().bold(),
            style(lexer_matches.get_one::<PathBuf>("file").unwrap().display()),
        );

        let mut lexer: Lexer<SimpleBuffer> = Lexer::new(&input);
        let tokens = lexer.lex();

        match tokens {
            Ok(tokens) => {
                for token in tokens {
                    println!("  {}", token);
                }
            }
            Err(e) => {
                for err in e {
                    eprintln!("{}", err);
                }
                std::process::exit(1);
            }
        }
    }
}
