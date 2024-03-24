mod core;
mod generation;
mod lexing;
mod parsing;
mod semantics;
mod utils;

use clap::{command, Parser as ClapParser, Subcommand};
use console::style;
use std::{io::Write, path::PathBuf};
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
        #[clap(name = "file")]
        in_file: PathBuf,
    },
    #[clap(name = "parse")]
    /// Runs the PArL parser on the given file and prints the AST.
    Parse {
        /// The file to print.
        #[clap(name = "file")]
        in_file: PathBuf,
    },
    /// Runs the PArL formatter on the given file.
    #[clap(name = "fmt")]
    Format {
        /// The file to format.
        #[clap(name = "file")]
        in_file: PathBuf,
    },
    /// Runs the PArL semantic analyzer on the given file.
    #[clap(name = "sem")]
    Semantic {
        /// The PArL source file to analyze.
        #[clap(name = "file")]
        in_file: PathBuf,
    },
    #[clap(name = "compile")]
    /// Compiles the given file to PArIR instructions.
    Compile {
        /// The PArL source file to compile.
        #[clap(name = "file")]
        in_file: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    let in_file = match &cli.subcmd {
        Commands::Lexer { in_file } => in_file,
        Commands::Format { in_file } => in_file,
        Commands::Semantic { in_file } => in_file,
        Commands::Parse { in_file } => in_file,
        Commands::Compile { in_file } => in_file,
    };

    if !in_file.exists() {
        let msg = style("error: file not found").red().bold().for_stderr();
        eprintln!("{} `{}`...", msg, style(in_file.display()).cyan());
        std::process::exit(1);
    }

    let input = match std::fs::read_to_string(in_file) {
        Ok(input) => input,
        Err(_) => {
            let msg = style("error: could not read file")
                .red()
                .bold()
                .for_stderr();
            eprintln!("{} `{}`...", msg, style(in_file.display()).cyan());
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
            Commands::Compile { .. } => "Compiling",
        })
        .green()
        .bold(),
        style(in_file.display())
    );

    let mut lexer: Lexer<SimpleBuffer> = Lexer::new(&input, in_file, None);

    let tokens = match lexer.lex() {
        Ok(tokens) => tokens,
        Err(e) => {
            for err in e {
                eprintln!("{}", err);
            }
            std::process::exit(1);
        }
    };

    match &cli.subcmd {
        Commands::Lexer { .. } => {
            println!("{} lexed successfully.", style(in_file.display()).cyan());
            for token in &tokens {
                println!("{:?}", token);
            }
            std::process::exit(0);
        }

        Commands::Format { in_file } => {
            let mut parser = Parser::new(&tokens, in_file);
            let ast = parser.parse();

            match ast {
                Ok(ast) => {
                    let mut printer = Formatter::new(in_file);
                    printer.visit(ast).unwrap();
                    println!(
                        "{} formatted successfully.",
                        style(in_file.display()).cyan()
                    );
                }
                Err(e) => {
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            }
        }

        Commands::Semantic { in_file: file } => {
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

        Commands::Parse { in_file } => {
            let mut parser = Parser::new(&tokens, in_file);
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

        Commands::Compile { in_file } => {
            let mut parser = Parser::new(&tokens, in_file);
            let ast = parser.parse();

            let ast = match ast {
                Ok(ast) => ast,
                Err(e) => {
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            };

            let mut sem_analyzer = SemAnalyzer::new();
            let result = sem_analyzer.analyze(ast);

            // Print "warninngs" in yellow
            let warnings_yellow = style("Warnings").yellow().bold();
            let errors_red = style("Errors").red().bold();

            if result.has_warnings() {
                println!("{}\n:", warnings_yellow);

                for warn in &result.warnings {
                    eprintln!("{}", warn);
                }
            }

            if result.has_errors() {
                println!("{}:\n", errors_red);

                for err in &result.errors {
                    eprintln!("{}: {}", errors_red, err);
                }
                std::process::exit(1);
            }

            let mut gen = generation::PArIRWriter::new();
            let par_ir_instr = gen.get_program(ast);
            // get in_file, strip suffix, add .parir
            let out_file = in_file.with_extension("parir");
            let mut out_file = std::fs::File::create(out_file).unwrap();

            out_file
                .write_all(par_ir_instr.to_string().as_bytes())
                .unwrap();

            println!("{} compiled successfully.", style(in_file.display()).cyan());
        }
    }
}
