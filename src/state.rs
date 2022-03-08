use crate::re_exports::*;

#[derive(Clone)]
pub struct State {
  pub environment: BTreeMap<String, String>,
  pub prompt: String,
  pub unicode_supported: bool,
  pub history: Vec<String>,
  pub commands: Vec<crate::Command>,
}

impl State {
  pub(crate) fn new(commands: &[crate::Command], unicode_supported: bool) -> Self {
    Self {
      environment: BTreeMap::new(),
      unicode_supported,
      prompt: if unicode_supported { "❯ " } else { "> " }.into(),
      commands: commands.to_vec(),
      history: Vec::new(),
    }
  }

  pub fn set_environment_variable(&mut self, key: impl AsRef<str>, value: impl AsRef<str>) {
    self
      .environment
      .insert(key.as_ref().into(), value.as_ref().into());

    #[cfg(feature = "std")]
    std::env::set_var(key.as_ref(), value.as_ref());
  }
}
