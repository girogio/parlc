mod errors;
mod scanner;
mod utils;

use clap::{arg, command, value_parser, Arg, Command};
use console::style;
use console::Term;
use std::{io::Read, path::PathBuf};
use utils::SimpleBuffer;

use crate::scanner::Lexer;

const BANNER: &str = r#"
 _____                                         _____
( ___ )                                       ( ___ )
 |   |~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~|   |
 |   |                    _                    |   |
 |   |                   | |                   |   |
 |   |   __ _ _ __   __ _| | __ _ _ __   __ _  |   |
 |   |  / _` | '_ \ / _` | |/ _` | '_ \ / _` | |   |
 |   | | (_| | | | | (_| | | (_| | | | | (_| | |   |
 |   |  \__, |_| |_|\__,_|_|\__,_|_| |_|\__, | |   |
 |   |   __/ |                           __/ | |   |
 |   |  |___/                           |___/  |   |
 |___|~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~|___|
(_____)                                       (_____)

"#;

fn main() {
    println!("{}", BANNER);

    let _term = Term::stdout();
    let _err = Term::stderr();

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
                    eprintln!(
                        "{} `{}`...",
                        msg,
                        style(file_path.display()).cyan().italic()
                    );
                    std::process::exit(1);
                }
            };
        }

        println!(
            "{} `{}`\n\n",
            style("Lexing").green().bold(),
            style(lexer_matches.get_one::<PathBuf>("file").unwrap().display()).italic(),
        );

        let mut lexer: Lexer<SimpleBuffer> = Lexer::new(&input);
        let tokens = lexer.lex();

        if let Ok(tokens) = tokens {
            for token in tokens {
                println!("{}", token);
            }
        } else {
            eprintln!("Error: Could not lex input.");
            std::process::exit(1);
        }
    }
}
