extern crate pig_lib;
extern crate nom;
extern crate reedline;
use pig_lib::evaluator::*;
use pig_lib::lexer::token::*;
use pig_lib::lexer::*;
use pig_lib::parser::*;
use nom::Err;
use std::borrow::Cow::{self, Borrowed, Owned};
use reedline::{DefaultPrompt, Reedline, Signal};


fn main() {
    println!();
    println!("This is the monkey language repl v0.5.0");
    println!("Press Ctrl-D or enter \"quit\" to exit.");
    println!();
 let mut evaluator = Evaluator::new();
let mut line_editor = Reedline::create();
let prompt = DefaultPrompt::default();
    loop {
        let readline = line_editor.read_line(&prompt);
        match readline {
            Ok(Signal::Success(line)) => {
                let lex_tokens = Lexer::lex_tokens(line.as_bytes());
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
            Ok(Signal::CtrlD | Signal::CtrlC) => {
                println!("CTRL-C");
                break;
            }
                    x => {
            println!("Event: {:?}", x);
        }
        }


    }

}
