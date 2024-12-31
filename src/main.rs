//! src/main.rs

/*******************************************************************************
 *                                MAIN MODULE
 *-------------------------------------------------------------------------------
 * This is the entry point for our language processing tool. It reads a `.pfl` file
 * or raw source code from the command line, tokenizes it with the `Lexer`, then
 * parses it with the `Parser` to produce an AST, which is printed for inspection.
 ******************************************************************************/

use std::env;
use std::fs;
use std::process;

use rdp::{Lexer, Parser};

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // We need at least 2 arguments: the program name and the input source (file or code).
    if args.len() < 2 {
        eprintln!("Usage:");
        eprintln!("  {} <file.pfl>", args[0]);
        eprintln!("  {} \"<source_code>\"", args[0]);
        process::exit(1);
    }

    // Decide how to interpret the argument(s):
    //  - If there's exactly one argument beyond the program name and it ends in `.pfl`,
    //    read from that file.
    //  - Otherwise, treat all subsequent arguments as direct source code, joined by spaces.
    let input = if args.len() == 2 && args[1].ends_with(".pfl") {
        match fs::read_to_string(&args[1]) {
            Ok(content) => content,
            Err(err) => {
                eprintln!("Error reading file '{}': {}", args[1], err);
                process::exit(1);
            }
        }
    } else {
        // Join arguments beyond index 1 with spaces for direct source code.
        args[1..].join(" ")
    };

    // Create a lexer to tokenize the input.
    let mut lexer = Lexer::new(&input);
    let tokens = match lexer.tokenize() {
        Ok(toks) => toks,
        Err(err) => {
            eprintln!("Lexing Error: {}", err);
            process::exit(1);
        }
    };

    // Create a parser to convert tokens into an AST (Program).
    let mut parser = Parser::new(tokens);
    let program = match parser.parse_program() {
        Ok(prog) => prog,
        Err(err) => {
            eprintln!("Parsing Error: {}", err);
            process::exit(1);
        }
    };

    // Print the resulting AST in debug format.
    println!("{:#?}", program);
}
