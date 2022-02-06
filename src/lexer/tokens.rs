use logos::Logos;

#[derive(Debug, PartialEq, Logos)]
pub enum LexToken {
  #[token("(")]
  OpenParen,

  #[token(")")]
  CloseParen,

  #[token("{")]
  OpenBrace,

  #[token("}")]
  CloseBrace,

  #[token("+")]
  Plus,

  #[token("-")]
  Minus,

  #[token("*")]
  Multiply,

  #[token("/")]
  Divide,

  #[token(">")]
  Greater,

  #[token("<")]
  Less,

  #[token("let")]
  Let,

  #[token("if")]
  If,

  // Match string literals and then strip the " at start and end
  #[regex("\"([^\"\\\\]|\\\\.)*\"", |lex| lex.slice()[1..lex.slice().len() - 1].to_owned())]
  String(String),

  // Non-literal strings (currently variable names)
  // #[regex("[a-zA-Z]+")]
  // Ident(String),
  #[regex(r"[+-]?([0-9]*[.])?[0-9]+", |lex| lex.slice().parse())]
  Number(f64),

  #[regex("(true|false)", |lex| lex.slice() == "true")]
  Boolean(bool),

  #[regex(r"[ \t\n\f]+", logos::skip)]
  Whitespace,

  #[error]
  Error,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
  OpenParen,

  CloseParen,

  OpenBrace,

  CloseBrace,

  Plus,

  Minus,

  Multiply,

  Divide,

  Greater,

  Less,

  Literal(LiteralKind),

  Ident(IdentKind),
}

#[derive(Clone, Debug, PartialEq)]
pub enum LiteralKind {
  Number,
  String,
  Boolean,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
  Number(f64),
  String(String),
  Boolean(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub enum IdentKind {
  Variable,
  Let,
  If,
  Greater,
  Less,
  FuncName,
  Plus,
  Minus,
  Multiply,
  Divide,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ident {
  pub kind: IdentKind,
  pub value: Option<Value>,
}

impl Ident {
  pub fn is_builtin(&self) -> bool {
    matches!(
      self.kind,
      IdentKind::Let
        | IdentKind::Divide
        | IdentKind::Plus
        | IdentKind::FuncName
        | IdentKind::If
        | IdentKind::Greater
        | IdentKind::Less
        | IdentKind::Multiply
        | IdentKind::Minus
    )
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
  pub kind: TokenKind,
  pub value: Option<Value>,
}
