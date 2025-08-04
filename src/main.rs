mod lex;
use lex::Lexer;
mod reader;
use reader::Parser;
use std::rc::Rc;
use std::cell::RefCell;
use crate::{env::ExecutionEnv, liblisp::Literal};
mod env;
mod liblisp;


fn main() {
  let mut lexer: Lexer = Lexer::from_string("(defn add-3 (a) (+ a 3)) (add-3 3)".to_string());
  lexer.tokenize();
  lexer.substitute();
  let mut parser = Parser::from_tokenizer(lexer);
  parser.parse();
  let env = Rc::new(RefCell::new(ExecutionEnv::root()));
  let returns: Vec<Literal> = parser.out().into_iter().map(|a| ExecutionEnv::evaluate(env.clone(), a)).collect();
  print_literals(returns);
}

fn print_literals(inp: Vec<Literal>) -> () {
  for i in inp {
    println!("{}",i);
  }
}