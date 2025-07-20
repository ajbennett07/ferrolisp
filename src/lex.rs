use crate::lex::Bracket::*;

#[derive(Debug)]
enum LexerMode {
  Neutral,
  Numeric,
  String,
  Ident,
  Key,
  SpecialCharacters,
}

#[derive(Debug)]
pub struct Lexer {
  stream: String,
  mode: LexerMode,
  pub out: Vec<Token>,
  str_buffer: Vec<char>,
  num_buffer: Vec<char>,
  ident_buffer: Vec<char>,
}

impl Lexer {
  //Following three functions are all utilities
  //from_string and new do the same function,
  //initialize a new Lexer with or without a string,
  //dump just lists out of all of the tokens collected
  pub fn from_string(str: String) -> Self {
    let mut tmp = Self::new();
    tmp.stream = str;
    tmp
  }
  pub fn new() -> Self {
    Lexer {
      stream: String::new(),
      mode: LexerMode::Neutral,
      out: Vec::new(),
      str_buffer: Vec::new(),
      num_buffer: Vec::new(),
      ident_buffer: Vec::new(),
    }
  }
  pub fn _dump(&mut self) -> Vec<String> {
    let mut ret = Vec::new();
    for (_, v) in self.out.clone().into_iter().enumerate() {
      ret.push(v._format())
    }
    ret
  }
  pub fn tokenize(&mut self) {
    for (i,c) in self.get_stream().chars().enumerate() {
      match c {
        '\"' => {
          if let LexerMode::String = self.mode {
            self.flush();
            self.mode = LexerMode::Neutral;
          }
          else {
          self.flush();
          self.mode = LexerMode::String;            
          } 
        },
        'a'..='z' | 'A'..='Z' => {
          self.match_char(c);
        },
        '0'..='9' => {
          self.match_digit(c);
        }
        ' ' | '\t' | '\n' => {
          //Whitespace flushes all buffers and returns to neutral
          //Conversly, whitespace in strings is ignored
          if let LexerMode::String = self.mode {
            self.match_char(c);
          }
          else {
            self.flush();
            self.mode = LexerMode::Neutral;
          }
        }
        '(' | ')' | '{' | '}' | '[' | ']' | '<' | '>' => {
          self.match_bracket(c);
        }
        '@' |'!' | '$'..='\'' | '*'..='/' | ';'..='?' | '^' => {
          self.match_special_char(c);
        }
        ':' => {
          self.flush();
          self.mode = LexerMode::Key;
          self.ident_buffer.push(':')
        }
        _ => {
          panic!("Unexpected token ({:?})encountered at char: {}", c, i)
        }
      }
    }
    self.flush();
    self.out.push(Token::EndOfFile);
  }
  //Flushes all lexer buffers
  //(num_buffer, ident_buffer, and str_buffer)
  //This clears all buffers and pushes their contents as Tokens
  fn flush(&mut self) {
    match self.mode {
      LexerMode::Neutral => {
        return;
      }
      LexerMode::String => {
        let s = self.str_buffer.clone().into_iter().collect::<String>();
        self.str_buffer.clear();
        self.out.push(Token::String(s));
      },
      LexerMode::Numeric => {
        let i = self.num_buffer.clone().into_iter().collect::<String>().parse::<f64>().unwrap();
        self.num_buffer.clear();
        self.out.push(Token::Number(i));
      },
      LexerMode::Ident => {
        let s = self.ident_buffer.clone().into_iter().collect::<String>();
        self.ident_buffer.clear();
        self.out.push(Token::Ident(s));
      }
      LexerMode::Key => {
        let s = self.ident_buffer.clone().into_iter().collect::<String>();
        self.ident_buffer.clear();
        self.out.push(Token::Key(s));
      }
      LexerMode::SpecialCharacters => {
        let s = self.ident_buffer.clone().into_iter().collect::<String>();
        self.ident_buffer.clear();
        self.out.push(Token::Ident(s));
      }
    }
  }
  //Returns the current stream as a string
  fn get_stream(&mut self) -> String {
    return self.stream.clone();
  }
  //This function decides what to do when a character is encountered
  //Result is based on the Lexer mode
  fn match_char(&mut self, c: char) {
    match self.mode {
      LexerMode::Neutral => {
        self.mode = LexerMode::Ident;
        self.ident_buffer.push(c);
      }
      LexerMode::String => {
        self.str_buffer.push(c);
      }
      LexerMode::Numeric => {
        self.flush();
        self.mode = LexerMode::Ident;
        self.ident_buffer.push(c);
      }
      LexerMode::Ident => {
        self.ident_buffer.push(c);
      }
      LexerMode::Key => {
        self.ident_buffer.push(c);
      }
      LexerMode::SpecialCharacters => {
        self.flush();
        self.mode = LexerMode::SpecialCharacters;
        self.ident_buffer.push(c);
      }
    }
  }
  //Just like match_char() -> (), but used for digits
  fn match_digit(&mut self, d: char) {
    match self.mode {
      LexerMode::Neutral => {
        self.mode = LexerMode::Numeric;
        self.num_buffer.push(d);
      }
      LexerMode::String => {
        self.str_buffer.push(d);
      }
      LexerMode::Numeric => {
        self.num_buffer.push(d);
      }
      LexerMode::Ident => {
        self.ident_buffer.push(d);
      }
      LexerMode::Key => {
        self.ident_buffer.push(d);
      }
      LexerMode::SpecialCharacters => {
        self.flush();
        self.mode = LexerMode::SpecialCharacters;
        self.num_buffer.push(d);
      }
    }
  }
  fn match_special_char(&mut self, c: char) {
    match self.mode {
      LexerMode::SpecialCharacters => {
        self.ident_buffer.push(c);
      }
      LexerMode::Neutral => {
        self.mode = LexerMode::SpecialCharacters;
        self.ident_buffer.push(c);
      }
      LexerMode::String => {
        self.str_buffer.push(c);
      }
      LexerMode::Numeric => {
        self.flush();
        self.mode = LexerMode::SpecialCharacters;
        self.ident_buffer.push(c);
      }
      LexerMode::Ident => {
        self.ident_buffer.push(c);
      }
      LexerMode::Key => {
        self.flush();
        self.mode = LexerMode::SpecialCharacters;
        self.ident_buffer.push(c);
      }
    }
  }
  //Just like match_char and match_digit but handles brackets
  fn match_bracket(&mut self, b: char) {
    if let LexerMode::String = self.mode {
      self.str_buffer.push(b);
      return;
    }
    self.flush();
    self.mode = LexerMode::Neutral;
    match b {
      '(' => {
        self.out.push(Token::Bracket(ParenOpen));
      }
      ')' => {
        self.out.push(Token::Bracket(ParenClose));
      }
      '[' => {
        self.out.push(Token::Bracket(BracketOpen))
      }
      ']' => {
        self.out.push(Token::Bracket(BracketClose))
      }
      '{' => {
        self.out.push(Token::Bracket(CurlyOpen));
      }
      '}' => {
        self.out.push(Token::Bracket(CurlyClose));
      }
      '<' => {
        self.out.push(Token::Bracket(AngleOpen));
      }
      '>' => {
        self.out.push(Token::Bracket(AngleClose));
      }
      _ => {
        panic!("Catastrophic error occurred during parsing of bracket tokens")
      }
    }
    self.mode = LexerMode::Neutral;
  }
  pub fn substitute(&mut self) {
    let mut new_tokens: Vec<Token> = Vec::new();
    for i in self.out.clone() {
      match i {
        Token::Ident(s) => {
          match s.as_str() {
            "true" => {
              new_tokens.push(Token::Reserved(s));
            },
            "false" => {
              new_tokens.push(Token::Reserved(s));
            },
            "nil" => {
              new_tokens.push(Token::Reserved(s));
            }
            _ => {new_tokens.push(Token::Ident(s))}
          }
        }
        _ => {
          new_tokens.push(i);
        }
      }
    }
    self.out = new_tokens;
  }
}

#[derive(Clone, Debug)]
pub enum Token {
  Bracket(Bracket),
  String(String),
  Number(f64),
  Ident(String),
  Key(String),
  Reserved(String),
  EndOfFile,
}

impl Token {
  fn _format(self) -> String {
    match self {
      Token::Bracket(a) => {
        format!("Token Type: Bracket\nToken Val: {}", a._format())
      }
      Token::String(a) => {
        format!("Token Type: String\nToken Val: {:?}", a)
      }
      Token::Number(a) => {
        format!("Token Type: Number\nToken Val: {:?}", a)
      }
      Token::Ident(a) => {
        format!("Token Type: Identifier\nToken Val: {:?}", a)
      }
      Token::Key(a) => {
        format!("Token Type: Key\nToken Val: {:?}", a)
      }
      Token::Reserved(a) => {
        format!("Token Type: Reserved Ident\nToken Val: {:?}", a)
      }
      Token::EndOfFile => {
        format!("Token Type: End Of Stream")
      }
    }
  }
}

#[derive(Clone, Debug, Copy)]
pub enum Bracket {
  ParenOpen,
  ParenClose,
  BracketOpen,
  BracketClose,
  CurlyOpen,
  CurlyClose,
  AngleOpen,
  AngleClose,
}

impl Bracket {
  fn _format(self) -> String {
    match self {
      Bracket::ParenOpen => {
        "(".to_string()
      }
      Bracket::ParenClose => {
        ")".to_string()
      }
      Bracket::BracketOpen => {
        "[".to_string()
      }
      Bracket::BracketClose => {
        "]".to_string()
      }
      Bracket::CurlyOpen => {
        "{".to_string()
      }
      Bracket::CurlyClose => {
        "}".to_string()
      }
      Bracket::AngleOpen => {
        "<".to_string()
      }
      Bracket::AngleClose => {
        ">".to_string()
      }
    }
  }
}