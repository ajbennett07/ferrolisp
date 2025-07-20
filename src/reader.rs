use crate::lex;

use crate::liblisp::Literal;


#[derive(Clone)]
pub struct Parser {
  pub stream: Vec<lex::Token>,
  pub out: Vec<Literal>,
  current_tok: lex::Token,
  index: usize,
  stream_length: usize,
}

impl Parser {
  pub fn from_tokenizer(to_parse: lex::Lexer) -> Parser {
    let cache = to_parse.out.clone();
    Parser { 
      stream: cache.clone(), 
      out: Vec::new(), 
      current_tok: cache[0].clone(),
      index: 0,
      stream_length: cache.into_iter().count(),
    }
  }
  pub fn out(self) -> Vec<Literal> {
    return self.out;
  }
  pub fn parse(&mut self) {
    let mut ret;
    loop {
      if self.index >= self.stream_length {
        break;
      }
      match self.current_tok.clone() {
        lex::Token::Bracket(_) => {
          ret = self.parse_list();
        }
        lex::Token::EndOfFile => {
          return;
        }
        _ => {
          ret = self.parse_atomic();
        }
      }
      self.out.push(ret.clone());
    }
  }
  fn parse_atomic(&mut self) -> Literal{
    match self.current_tok.clone() {
      lex::Token::Key(s) => {
        self.get_next_token();
        return Literal::Atom(s);
      }
      lex::Token::Ident(s) => {
        self.get_next_token();
        return Literal::Atom(s);
      }
      lex::Token::Number(i) => {
        self.get_next_token();
        return Literal::Num(i);
      }
      lex::Token::String(s) => {
        self.get_next_token();
        return Literal::String(s);
      }
      lex::Token::Reserved(s) => {
        self.get_next_token();
        return Literal::Atom(s);
      }
      lex::Token::Bracket(_b) => {
        self.get_next_token();
        return Literal::Err("Parser Error: Unexpected bracket encountered".to_string(), -1)
      }
      lex::Token::EndOfFile => {
        self.get_next_token();
        return Literal::Err("Parser Error: Unexpected EOF encountered".to_string(), -1)
      }
    }
  }
  fn parse_list(&mut self) -> Literal {
    let mut ret = Vec::new();
    self.get_next_token();
    loop {
      match self.current_tok {
        lex::Token::Bracket(b) => {
          match b {
            lex::Bracket::ParenOpen => {
              ret.push(self.parse_list());
            }
            _ => {
              self.get_next_token();
              return Literal::List(ret);
            }
          }
        }
        lex::Token::EndOfFile => {
          ret.push(Literal::Err("Parser Error: Unmatched Parenthesis".to_string(), -1));
          return Literal::List(ret);
        }
        _ =>  {
          ret.push(self.parse_atomic());
        }
      }
  }
  }
  fn get_next_token(&mut self) {
    if self.stream.clone().into_iter().count() == 0 || self.stream.clone().into_iter().count() == self.index+1 {
      self.current_tok = lex::Token::EndOfFile;
      return;
    }
    self.index = self.index+1;
    self.current_tok = (self.stream[self.index]).clone();
  }
}

