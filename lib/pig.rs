use wasm_bindgen::prelude::*;
extern crate nom;

pub mod evaluator;
pub mod lexer;
pub mod parser;
use lexer::*;
use token::*;
use parser::*;
use nom::Err;
use evaluator::*;
#[wasm_bindgen]
    pub fn evaluate(line: String) -> String{
    let mut evaluator = Evaluator::new();
    let lex_tokens = Lexer::lex_tokens(line.as_bytes());
                match lex_tokens {
                    Ok((_, r)) => {
                        let tokens = Tokens::new(&r);
                        let parsed = Parser::parse_tokens(tokens);
                        match parsed {
                            Ok((_, program)) => {
                                let eval = evaluator.eval_program(program);
                                return format!("{}",eval);
                            }
                            Err(Err::Error(_)) => format!("Parser error"),
                            Err(Err::Failure(_)) => format!("Parser failure"),
                            Err(Err::Incomplete(_)) => format!("Incomplete parsing"),
                        }

                }
Err(Err::Error(_)) => format!("Lexer error"),
                    Err(Err::Failure(_)) => format!("Lexer failure"),
                    Err(Err::Incomplete(_)) => format!("Incomplete lexing"),
                }}
