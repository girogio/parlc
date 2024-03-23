mod core;
mod lexing;
mod parsing;
mod semantics;
mod utils;

use clap::{command, value_parser, Arg, Command, Parser as ClapParser, Subcommand};
use console::style;
use std::{io::Read, path::PathBuf};
use utils::SimpleBuffer;

use crate::{
    lexing::Lexer,
    parsing::{ast::Visitor, Parser},
    semantics::visitors::{Formatter, Printer, ScopeChecker, TypeChecker},
};

#[derive(ClapParser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    subcmd: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Runs the PArL lexer on the given file.
    Lex {
        /// The file to lex.
        file: PathBuf,
    },
    /// Runs the PArL formatter on the given file.
    Fmt {
        /// The file to format.
        file: PathBuf,
    },
    /// Runs the PArL semantic analyzer on the given file.
    Sem {
        /// The file to analyze.
        file: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    let file = match &cli.subcmd {
        Commands::Lex { file } => file,
        Commands::Fmt { file } => file,
        Commands::Sem { file } => file,
    };

    if !file.exists() {
        let msg = style("error: file not found").red().bold().for_stderr();
        eprintln!("{} `{}`...", msg, style(file.display()).cyan());
        std::process::exit(1);
    }

    let input = match std::fs::read_to_string(file) {
        Ok(input) => input,
        Err(_) => {
            let msg = style("error: could not read file")
                .red()
                .bold()
                .for_stderr();
            eprintln!("{} `{}`...", msg, style(file.display()).cyan());
            std::process::exit(1);
        }
    };

    println!(
        "\n{} {}\n",
        style(match &cli.subcmd {
            Commands::Lex { .. } => "Lexing",
            Commands::Fmt { .. } => "Formatting",
            Commands::Sem { .. } => "Analyzing",
        })
        .green()
        .bold(),
        style(file.display())
    );

    let mut lexer: Lexer<SimpleBuffer> = Lexer::new(&input, file, None);

    let tokens = match lexer.lex() {
        Ok(tokens) => tokens,
        Err(e) => {
            for err in e {
                eprintln!("{}", err);
            }
            std::process::exit(1);
        }
    };

    if let Commands::Lex { .. } = &cli.subcmd {
        for token in &tokens {
            println!("{:?}", token);
        }
    }

    if let Commands::Fmt { .. } = &cli.subcmd {
        let mut parser = Parser::new(&tokens, file);
        let ast = parser.parse();

        match ast {
            Ok(ast) => {
                let mut printer = Formatter::new(file);
                printer.visit(ast).unwrap();
                println!("{} formatted successfully.", style(file.display()).cyan());
            }
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    }

    if let Commands::Sem { .. } = &cli.subcmd {
        let mut parser = Parser::new(&tokens, file);
        let ast = parser.parse();

        match ast {
            Ok(ast) => {
                let mut scope_check = ScopeChecker::new();
                match scope_check.visit(ast) {
                    Ok(_) => {
                        let mut type_check = TypeChecker::new();
                        match type_check.visit(ast) {
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
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    }
}
