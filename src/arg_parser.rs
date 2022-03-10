use crate::re_exports::*;

pub struct Arg<'a> {
  pub is_required: bool,
  pub flag_type: Option<FlagType>,
  pub arg_type: ArgType<'a>,
  pub name: &'a str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArgType<'a> {
  Number,
  String,
  Boolean,
  Array(&'a ArgType<'a>, &'a str),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlagType {
  Short,
  Long,
}

impl<'a> Arg<'a> {
  pub const fn check(self) -> Self {
    if self.flag_type.is_some() && matches!(self.arg_type, ArgType::Boolean) {
      panic!("Argument cannot be a positional argument and a boolean at the same time\n\t\tIf you need to use a boolean, use a flag instead");
    }

    self
  }

  pub const fn default() -> Self {
    Self {
      is_required: false,
      flag_type: None,
      arg_type: ArgType::String,
      name: "",
    }
  }
}

impl Display for Arg<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    let mut out = String::new();

    if let Some(FlagType::Long) = self.flag_type {
      out.push_str("--");
    } else if let Some(FlagType::Short) = self.flag_type {
      out.push('-');
    }

    out.push_str(self.name);

    if self.arg_type != ArgType::Boolean {
      out.push('=');

      match self.arg_type {
        ArgType::Array(arg_type, sep) => out.push_str(&format!("<[{:#?}[{}]...]>", arg_type, sep)),
        _ => out.push_str(&format!("<{:#?}>", self.arg_type)),
      };
    }

    if self.is_required {
      out = format!("[{}]", out);
    } else {
      out = format!("{{{}}}", out);
    }

    write!(f, "{}", out)
  }
}

impl From<String> for Arg<'_> {
  fn from(s: String) -> Self {
    let mut out = Arg::default();

    if let Some(long_flag) = s.strip_prefix("--") {
      out.flag_type = Some(FlagType::Long);
      out.name = unsafe { mem::transmute(long_flag) };
    } else if let Some(short_flag) = s.strip_prefix('-') {
      out.flag_type = Some(FlagType::Short);
      out.name = unsafe { mem::transmute(short_flag) };
    }

    out
  }
}
