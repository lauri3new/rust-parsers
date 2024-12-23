mod chumsky_parser;
use chumsky_parser::example;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Token {
  Number(f32),
  Plus,
  Minus,
  Multiply,
  Stop
}

fn is_number(a: &str) -> bool {
  match a.parse::<f32>() {
    Ok(a) => true,
    _ => false
  }
}

fn tokenise_single(s: &str) -> Token {
  match s {
    a if is_number(a) => a.parse::<f32>().map(|a| Token::Number(a)).unwrap(),
    b if b == String::from('+') => Token::Plus,
    c if c == String::from('*') => Token::Multiply,
    d if d == String::from('-') => Token::Minus,
    _ => panic!()
  }
}

fn tokenise(input: &str) -> Vec<Token> {
  let tokens = input.split_ascii_whitespace().map(tokenise_single).collect::<Vec<Token>>();
  tokens
}

#[derive(Debug)]
enum Tree {
  Leaf(Token),
  MinusNode(Box<Tree>),
  BinaryMinusNode(Box<Tree>, Box<Tree>),
  PlusNode(Box<Tree>, Box<Tree>),
  MultiplyNode(Box<Tree>, Box<Tree>)
}

#[derive(Debug)]
struct Parser <'a>{
  tokens: &'a Vec<Token>,
  current_pos: usize
}

fn display(tokens: &Vec<&Token>) -> String {
  ".. ".to_string() + &tokens.iter().cloned().map(|t| {
    match t {
      Token::Number(x) => x.to_string(),
      Token::Minus => "-".to_string(),
      Token::Plus => "+".to_string(),
      Token::Multiply => "*".to_string(),
      _ => "".to_string()
    }
  }).collect::<Vec<_>>().join(" ")
}

impl Parser <'_>{
  fn new<'a>(input: &'a Vec<Token>) -> Parser<'_> {
    Parser {
      tokens: input,
      current_pos: 0 as usize
    }
  }

  fn consume_token(self: &mut Self) -> Token {
    let token = *self.tokens.get(self.current_pos).unwrap_or(&Token::Stop);
    self.current_pos += 1;
    token
  }

  fn match_token(self: &mut Self, token: &Token) -> bool {
    let t: Token = *self.tokens.get(self.current_pos).unwrap_or(&Token::Stop);
    let matched = match t {
      Token::Number(_) => match token {
        Token::Number(_) => true,
        _ => false
      },
      Token::Minus => match token {
        Token::Minus => true,
        _ => false
      },
      Token::Plus => match token {
        Token::Plus => true,
        _ => false
      },
      Token::Multiply => match token {
        Token::Multiply => true,
        _ => false
      },
      Token::Stop => match token {
        Token::Stop => true,
        _ => false
      }
    };
    if matched {
      self.current_pos += 1;
    }
    matched
  }

  fn previous(self: &Self) -> Token {
    *self.tokens.get(self.current_pos - 1).unwrap()
  }

  fn number<'a>(self: &mut Self) -> Tree {
    let number = self.consume_token();
    match number {
      Token::Number(x) => Tree::Leaf(Token::Number(x)),
      _ => {
        let context = self.tokens.iter()
          .enumerate()
          .take_while(|(i, _)| {
            if (self.current_pos < 2) {
              return true
            }
            print!("i: {i}");
            ((*i as i32) < self.current_pos as i32 - 2)
          })
          .map(|(_, t)| t)
          .collect::<Vec<_>>();
        let text = display(&context[(&context.len() - 5)..].to_vec());
        panic!("invalid token after: {text}")
      }
    }
  }

  fn minus<'a>(self: &mut Self) -> Tree {
    while (self.match_token(&Token::Minus)) {
      let right = self.minus();
      return Tree::MinusNode(Box::new(right))
    }

    self.number()
  }

  fn multiply<'a>(self: &mut Self) -> Tree {
    let mut expression = self.minus();

    while (self.match_token(&Token::Multiply)) {
      let right = self.minus();
      expression = Tree::MultiplyNode(Box::new(expression), Box::new(right));
    }

    expression
  }

  fn binary<'a>(self: &mut Self) -> Tree {
    let mut expression = self.multiply();

    while (self.match_token(&Token::Plus) | self.match_token(&Token::Minus)) {
      let previous = self.previous();
      let right = self.multiply();
      expression = match previous {
        Token::Plus => Tree::PlusNode(Box::new(expression), Box::new(right)),
        Token::Minus => Tree::BinaryMinusNode(Box::new(expression), Box::new(right)),
        _ => panic!()
      }
    }

    expression
  }

  fn exec(self: &mut Self) -> Tree {
    self.binary()
  }
  
}

fn evaluate(input: &Tree) -> f32 {
  match input {
    Tree::Leaf(t) => match t {
      Token::Number(i) => *i,
      _ => 0 as f32
    },
    Tree::MinusNode(a) => - evaluate(a),
    Tree::PlusNode(a, b) => evaluate(a) + evaluate(b),
    Tree::MultiplyNode(a, b) => evaluate(a) * evaluate(b),
    Tree::BinaryMinusNode(a, b) => evaluate(a) - evaluate(b)
  }
}

fn main() {
  let input = "1 + 2 + 3 + 4 + 5 * 2";
  let tokens = tokenise(&input);
  let mut parser = Parser::new(&tokens);
  let tree = &parser.exec();
  let evaled = evaluate(tree);
  print!("\n\ninput: {input:?}\ntokens: {tokens:?}\ntree: {tree:?}\noutput: {evaled:?}\n\n");
  example();
}
