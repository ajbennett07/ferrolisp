mod lex;
use lex::Lexer;
mod reader;
use reader::Parser;

use crate::{env::ExecutionEnv, liblisp::Literal};
mod env;
mod liblisp;


fn main() {
  let mut lexer: Lexer = Lexer::from_string("(progn () 1 2 (/ 1 2))".to_string());
  lexer.tokenize();
  lexer.substitute();
  let mut parser = Parser::from_tokenizer(lexer);
  parser.parse();
  let mut env = ExecutionEnv::root();
  let returns: Vec<Literal> = parser.out().into_iter().map(|a| env.evaluate(a)).collect();
  println!("{:?}", returns);
}