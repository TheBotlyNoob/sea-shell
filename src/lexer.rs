pub fn tokenize_command(input: impl AsRef<str>) -> Vec<Vec<Token>> {
  input
    .as_ref()
    .trim()
    .split(';')
    .map(|command| command.trim())
    .map(|input| {
      let raw_tokens = input.split_whitespace();

      raw_tokens
        .into_iter()
        .enumerate()
        .map(|(index, raw_token)| {
          if index == 0 {
            Token::new(TokenValue::Command(raw_token.into()), raw_token)
          } else if let Some(long_flag) = raw_token.strip_prefix("--") {
            Token::new(
              TokenValue::LongFlag(long_flag.into()),
              format!("--{}", raw_token),
            )
          } else if let Some(short_flag) = raw_token.strip_prefix('-') {
            Token::new(
              TokenValue::ShortFlag(short_flag.into()),
              format!("-{}", raw_token),
            )
          } else {
            Token::new(TokenValue::PositionalArgument(raw_token.into()), raw_token)
          }
        })
        .collect::<Vec<Token>>()
    })
    .filter(|tokens| !tokens.is_empty())
    .collect()
}

#[derive(Debug, Clone)]
pub struct Token {
  pub token_value: TokenValue,
  pub raw_token: String,
}

impl Token {
  pub fn new(token_value: TokenValue, raw_token: impl AsRef<str>) -> Self {
    Self {
      token_value,
      raw_token: raw_token.as_ref().into(),
    }
  }
}

impl std::fmt::Display for Token {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.raw_token)
  }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum TokenValue {
  Command(String),
  PositionalArgument(String),
  /// A long flag argument without the leading dashes.
  LongFlag(String),
  /// A short flag argument without the leading dash.
  ShortFlag(String),
}
