use crate::re_exports::*;

pub struct Arg<'a> {
  pub is_required: bool,
  pub is_flag: bool,
  pub flag_type: FlagType,
  pub name: &'a str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlagType {
  Number,
  String,
  Boolean,
}

impl Arg<'_> {
  pub const fn default() -> Self {
    Self {
      is_required: false,
      is_flag: false,
      flag_type: FlagType::Boolean,
      name: "",
    }
  }
}

impl Display for Arg<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    let mut out = String::new();

    if self.is_flag {
      out.push_str("--");
    }

    out.push_str(self.name);

    if self.is_required {
      out = format!("[{}]", out);
    } else {
      out = format!("{{{}}}", out);
    }

    write!(f, "{}", out)
  }
}
