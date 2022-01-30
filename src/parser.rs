#[derive(Debug, PartialEq)]
pub struct Number(pub i64);

impl std::str::FromStr for Number {
  type Err = std::num::ParseIntError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(Self(s.parse()?))
  }
}

#[derive(Debug, PartialEq)]
pub enum MathOperator {
  Add,
  Sub,
  Mul,
  Div,
}

impl MathOperator {
  pub fn from(s: &str) -> Option<Self> {
    match s {
      "+" => Some(Self::Add),
      "-" => Some(Self::Sub),
      "*" => Some(Self::Mul),
      "/" => Some(Self::Div),
      _ => None,
    }
  }
}
