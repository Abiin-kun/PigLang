extern crate pig_lib;

#[macro_use]
extern crate clap;
extern crate nom;

use pig_lib::evaluator::*;
use pig_lib::lexer::token::*;
use pig_lib::lexer::*;
use pig_lib::parser::*;
use nom::Err;
use std::fs::File;
use std::io::prelude::*;

use clap::{Command,Arg};



fn read_file(file_path: String) -> Result<String, ::std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {

    let matches = Command::new("PIG LANG MARK").about("Pig Lang").version("1").author("Abhinn Pandey <pandeyabhinn@gmail.com>").arg(Arg::new("s").long("source").help("Path of the Source to Run")).get_matches();
    let mat = matches.get_one::<String>("s").unwrap().to_string();
    let code_string = read_file(mat).ok();

    if let Some(code_string) = code_string {
        let mut evaluator = Evaluator::new();
        let lex_tokens = Lexer::lex_tokens(code_string.as_bytes());
        match lex_tokens {
            Ok((_, r)) => {
                let tokens = Tokens::new(&r);
                let parsed = Parser::parse_tokens(tokens);
                match parsed {
                    Ok((_, program)) => {
                        let eval = evaluator.eval_program(program);
                        println!("{}", eval);
                    }
                    Err(Err::Error(_)) => println!("Parser error"),
                    Err(Err::Failure(_)) => println!("Parser failure"),
                    Err(Err::Incomplete(_)) => println!("Incomplete parsing"),
                }
            }
            Err(Err::Error(_)) => println!("Lexer error"),
            Err(Err::Failure(_)) => println!("Lexer failure"),
            Err(Err::Incomplete(_)) => println!("Incomplete lexing"),
        }
    }
}
