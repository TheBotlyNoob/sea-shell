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
