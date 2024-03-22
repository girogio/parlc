mod core;
mod lexing;
mod parsing;
mod semantics;
mod utils;

use clap::{command, value_parser, Arg, Command};
use console::style;
use std::{io::Read, path::PathBuf};
use utils::SimpleBuffer;

use crate::{
    lexing::Lexer,
    parsing::{
        ast::Visitor,
        visitors::{AstFormatter, ScopeChecker},
        Parser,
    },
};

fn main() {
    let matches = command!()
        .subcommand(
            Command::new("lex")
                .about("Run the lexer on a PArL source file")
                .arg(Arg::new("file").value_parser(value_parser!(PathBuf))),
        )
        .subcommand(
            Command::new("fmt")
                .about("Format a PArL source file")
                .arg(Arg::new("file").value_parser(value_parser!(PathBuf))),
        )
        .subcommand(
            Command::new("sem")
                .about("Run the semantic analyzer on a PArL source file")
                .arg(Arg::new("file").value_parser(value_parser!(PathBuf))),
        )
        .get_matches();

    if let Some(lexer_matches) = matches.subcommand_matches("lex") {
        let mut input = String::new();

        if let Some(file_path) = lexer_matches.get_one::<PathBuf>("file") {
            let mut file = std::fs::File::open(file_path);

            match file {
                Ok(ref mut file) => {
                    file.read_to_string(&mut input).unwrap();

                    println!(
                        "{} `{}`\n",
                        style("Lexing").green().bold(),
                        style(file_path.display())
                    );

                    let mut lexer: Lexer<SimpleBuffer> = Lexer::new(&input, file_path, None);
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
                Err(_) => {
                    let msg = style("File not found:").red().bold().for_stderr();
                    eprintln!("{} `{}`...", msg, style(file_path.display()).cyan());
                    std::process::exit(1);
                }
            };
        }
    }

    if let Some(parser_matches) = matches.subcommand_matches("fmt") {
        let file = parser_matches.get_one::<PathBuf>("file");
        let mut input = String::new();

        if let Some(file_path) = file {
            let mut file = std::fs::File::open(file_path);

            match file {
                Ok(ref mut file) => {
                    file.read_to_string(&mut input).unwrap();

                    println!(
                        "\n{} `{}`\n",
                        style("Formatting").green().bold(),
                        style(file_path.display())
                    );

                    let mut lexer: Lexer<SimpleBuffer> = Lexer::new(&input, file_path, None);

                    let tokens = match lexer.lex() {
                        Ok(tokens) => tokens,
                        Err(e) => {
                            for err in e {
                                eprintln!("{}", err);
                            }
                            std::process::exit(1);
                        }
                    };

                    let mut parser = Parser::new(&tokens, file_path);
                    let ast = parser.parse();

                    match ast {
                        Ok(ast) => {
                            let mut printer = AstFormatter::new(file_path);
                            printer.visit(ast);
                        }
                        Err(e) => {
                            eprintln!("{}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Err(_) => {
                    let msg = style("File not found:").red().bold().for_stderr();
                    eprintln!("{} `{}`...", msg, style(file_path.display()).cyan());
                    std::process::exit(1);
                }
            };
        }
    }

    if let Some(parser_matches) = matches.subcommand_matches("sem") {
        let file = parser_matches.get_one::<PathBuf>("file");
        let mut input = String::new();

        if let Some(file_path) = file {
            let mut file = std::fs::File::open(file_path);

            match file {
                Ok(ref mut file) => {
                    file.read_to_string(&mut input).unwrap();

                    println!(
                        "\n{} `{}`\n",
                        style("Analyzing").green().bold(),
                        style(file_path.display())
                    );

                    let mut lexer: Lexer<SimpleBuffer> = Lexer::new(&input, file_path, None);

                    let tokens = match lexer.lex() {
                        Ok(tokens) => tokens,
                        Err(e) => {
                            for err in e {
                                eprintln!("{}", err);
                            }
                            std::process::exit(1);
                        }
                    };

                    let mut parser = Parser::new(&tokens, file_path);
                    let ast = parser.parse();

                    match ast {
                        Ok(ast) => {
                            let mut sem = ScopeChecker::new();
                            match sem.visit(ast) {
                                Ok(_) => {
                                    println!("Semantic analysis completed successfully.");
                                }
                                Err(e) => {
                                    eprintln!("{}", e);
                                    std::process::exit(1);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("{}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Err(_) => {
                    let msg = style("File not found:").red().bold().for_stderr();
                    eprintln!("{} `{}`...", msg, style(file_path.display()).cyan());
                    std::process::exit(1);
                }
            };
        }
    }
}
