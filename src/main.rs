extern crate getopts;
extern crate rustc_serialize;

use getopts::Options;
use rustc_serialize::json;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process::exit;

mod ast;
mod lexer;
mod parser;
mod precedence;
mod tokens;

use lexer::Lexer;
use parser::Parser;
use tokens::Token;

fn print_usage(program: &str, opts: Options) -> ! {
    let usage = format!("Usage: {} [options] filename", program);
    println!("{}", opts.usage(&usage));
    exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("t", "tokens", "Print tokens output by lexer and halt");
    opts.optflag("", "ast", "Print json representation of the ast and halt");
    opts.optflag("h", "help", "Print this help");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => panic!(e.to_string()),
    };

    // Print help if requested
    if matches.opt_present("h") {
        print_usage(&program, opts);
    }

    // Get filename argument
    let fname = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
    };

    let contents = read_file(fname);
    let mut lexer = Lexer::new(contents);

    if matches.opt_present("t") {
        print_tokens(&mut lexer);
        return
    }

    let mut parser = Parser::new(lexer);
    let ast = parser.parse();

    if matches.opt_present("ast") {
        let out = json::encode(&ast).unwrap();
        println!("{}", out);
        return
    }
}

fn print_tokens(lexer: &mut Lexer) {
    loop {
        let token = match lexer.next_token() {
            Ok(tok) => tok,
            Err(e) => panic!("Error ocurred in lexing: {}", e.to_string()),
        };


        if token != Token::Whitespace && token != Token::Comment {
            println!("{}", token);
        }

        if token == Token::Eof {
            break;
        }
    }
}

fn read_file(fname: String) -> String {
    let mut file = match File::open(&fname) {
        Ok(f) => f,
        Err(e) => panic!("Unable to open file `{}`: {}", fname, e.to_string()),
    };

    let mut buf = String::new();

    match file.read_to_string(&mut buf) {
        Ok(_) => buf,
        Err(e) => panic!("Unable to read from file `{}`: {}", fname, e.to_string()),
    }
}
