use chumsky::prelude::*;

#[derive(Debug, Clone, PartialEq)]
enum Token {
  Number(f32),
  Add(Box<Token>, Box<Token>),
  Minus(Box<Token>),
  Multiply(Box<Token>, Box<Token>),
  Subtract(Box<Token>, Box<Token>)
}

fn parser() -> impl Parser<char, Token, Error = Simple<char>> {
  let int = text::int(10)
    .map(|s: String| Token::Number(s.parse().unwrap()))
    .padded();

  let atom = int;

  let op = |c: char| just(c).padded();

  let unary = op('-')
      .repeated()
      .then(atom)
      .foldr(|_op, rhs| Token::Minus(Box::new(rhs)));

  let product = unary.clone()
      .then(op('*').to(Token::Multiply as fn(_, _) -> _)
          .then(unary)
          .repeated())
      .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

  let sum = product.clone()
      .then(op('+').to(Token::Add as fn(_, _) -> _)
        .or(op('-').to(Token::Subtract as fn (_, _) -> _))
          .then(product)
          .repeated())
      .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

  sum.then_ignore(end())
}

fn eval(expr: &Token) -> Result<f32, String> {
  match expr {
      Token::Number(x) => Ok(*x),
      Token::Minus(a) => Ok(-eval(a)?),
      Token::Add(a, b) => Ok(eval(a)? + eval(b)?),
      Token::Subtract(a, b) => Ok(eval(a)? - eval(b)?),
      Token::Multiply(a, b) => Ok(eval(a)? * eval(b)?),
      _ => panic!()
  }
}

pub fn example() {
  let input = "1 + 2 + 3 + 4 + 5 * 2";
  let tree = parser().parse(input).unwrap();
  let result = eval(&tree).unwrap();
  print!("\ninput:{input}\ntree:{tree:?}\noutput:{result:?}\n");
}