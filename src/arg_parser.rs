use crate::re_exports::*;

pub struct Arg<'a> {
  pub is_required: bool,
  pub is_flag: bool,
  pub name: &'a str,
}

impl Arg<'_> {
  pub const fn default() -> Self {
    Self {
      is_required: false,
      is_flag: false,
      name: "",
    }
  }
}

impl Display for Arg<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    let mut out = String::new();

    write!(f, "{}", out)
  }
}
