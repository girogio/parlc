mod core;
mod lexing;
mod parsing;
mod semantics;
mod utils;

use clap::{command, Parser as ClapParser, Subcommand};
use console::style;
use std::path::PathBuf;
use utils::SimpleBuffer;

use crate::{
    lexing::Lexer,
    parsing::{ast::Visitor, Parser},
    semantics::visitors::{Formatter, SemAnalyzer, TreePrinter},
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
    #[clap(name = "lex")]
    Lexer {
        /// The file to lex.
        file: PathBuf,
    },
    #[clap(name = "parse")]
    /// Runs the PArL parser on the given file and prints the AST.
    Parse {
        /// The file to print.
        file: PathBuf,
    },
    /// Runs the PArL formatter on the given file.
    #[clap(name = "fmt")]
    Format {
        /// The file to format.
        file: PathBuf,
    },
    /// Runs the PArL semantic analyzer on the given file.
    #[clap(name = "sem")]
    Semantic {
        /// The file to analyze.
        file: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    let file = match &cli.subcmd {
        Commands::Lexer { file } => file,
        Commands::Format { file } => file,
        Commands::Semantic { file } => file,
        Commands::Parse { file } => file,
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
            Commands::Lexer { .. } => "Lexing",
            Commands::Format { .. } => "Formatting",
            Commands::Semantic { .. } => "Analyzing",
            Commands::Parse { .. } => "Printing",
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

    if let Commands::Lexer { .. } = &cli.subcmd {
        for token in &tokens {
            println!("{:?}", token);
        }
    }

    if let Commands::Format { .. } = &cli.subcmd {
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

    if let Commands::Semantic { .. } = &cli.subcmd {
        let mut parser = Parser::new(&tokens, file);
        let ast = parser.parse();

        match ast {
            Ok(ast) => {
                let mut sem_analyzer = SemAnalyzer::new();
                let result = sem_analyzer.analyze(ast);

                if result.has_warnings() {
                    for warn in &result.warnings {
                        eprintln!("{}", warn);
                    }
                }

                if result.has_errors() {
                    for err in &result.errors {
                        eprintln!("{}", err);
                    }
                    std::process::exit(1);
                }

                println!("{} analyzed successfully.", style(file.display()).cyan());
            }
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    }

    if let Commands::Parse { .. } = &cli.subcmd {
        let mut parser = Parser::new(&tokens, file);
        let ast = parser.parse();

        match ast {
            Ok(ast) => {
                let mut printer = TreePrinter::new();
                printer.visit(ast).unwrap();
            }
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    }
}
