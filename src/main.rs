//! src/main.rs

use std::env;
use std::fs;
use std::process;

use rdp::{Lexer, Parser};

//-------------------------------------------------------------------------
// Main Entry Point
//-------------------------------------------------------------------------

// This entry point handles command-line argument parsing, input source determination
// (file or direct input), and orchestrates the lexing and parsing of the provided source code.
//
// ## Usage
//
// The application can be executed with the following command-line arguments:
//
// ```bash
// <program_name> <file.pfl>
// <program_name> "<source_code>"
// ```
//
// - `<file.pfl>`: Path to a `.pfl` file containing source code.
// - `<source_code>`: Direct source code input as a string.
//
// The program will read the input, tokenize it using the `Lexer`, parse the
// tokens into an Abstract Syntax Tree (AST) using the `Parser`, and then
// output the AST in a pretty-printed format.
fn main() {
    // Retrieve command-line arguments
    let args: Vec<String> = env::args().collect();

    // Ensure at least one argument is provided (the program name is args[0])
    if args.len() < 2 {
        eprintln!("Usage:");
        eprintln!("  {} <file.pfl>", args[0]);
        eprintln!("  {} \"<source_code>\"", args[0]);
        process::exit(1);
    }

    // Determine the input source
    let input = if args.len() == 2 && args[1].ends_with(".pfl") {
        // Input is a .pfl file
        match fs::read_to_string(&args[1]) {
            Ok(content) => content,
            Err(err) => {
                eprintln!("Error reading file '{}': {}", args[1], err);
                process::exit(1);
            }
        }
    } else {
        // Input is source code provided directly or multiple arguments
        // Join all arguments beyond the program name with spaces
        args[1..].join(" ")
    };

    // Initialize the lexer with the input source code
    let mut lexer = Lexer::new(&input);

    // Tokenize the input
    let tokens = match lexer.tokenize() {
        Ok(toks) => toks,
        Err(err) => {
            eprintln!("Lexing Error: {}", err);
            process::exit(1);
        }
    };

    // Initialize the parser with the tokens
    let mut parser = Parser::new(tokens);

    // Parse the tokens into an AST (Program)
    let program = match parser.parse_program() {
        Ok(prog) => prog,
        Err(err) => {
            eprintln!("Parsing Error: {}", err);
            process::exit(1);
        }
    };

    // Output the AST in a pretty-printed format
    println!("{:#?}", program);
}
