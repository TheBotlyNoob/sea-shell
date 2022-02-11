use std::iter;

pub fn tokenize_command<'a>(
  input: impl AsRef<str> + 'a,
) -> iter::Map<iter::Map<std::str::Split<'a, char>, fn(&'a str) -> String>, fn(String) -> Vec<String>>
{
  input
    .as_ref()
    .trim()
    .split(';')
    .map(|command| command.trim().into())
    .map(|input: String| {
      let tokens = input.split_whitespace();

      for token in tokens.clone() {
        match token {
          _ if token.starts_with("#") => return Vec::new(),
          _ => (),
        }
      }

      tokens
        .map(|token| token.trim().to_owned())
        .collect::<Vec<String>>()
    })
  // .filter(|tokens| !tokens.is_empty())
}

#[derive(Debug, Clone)]
pub struct Token {
  pub token_type: TokenType,
  pub token_value: Option<TokenValue>,
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum TokenType {
  Command,
  PositionalArgument,
  /// A large flag argument without the leading dashes.
  LargeFlagArgument,
  /// A short flag argument without the leading dash.
  ShortFlagArgument,
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum TokenValue {
  Command(String),
  PositionalArgument(String),
  /// A large flag argument without the leading dashes.
  LargeFlagArgument(String),
  /// A short flag argument without the leading dash.
  ShortFlagArgument(String),
}
