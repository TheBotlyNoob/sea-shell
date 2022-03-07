use crate::re_exports::*;

#[derive(Clone)]
pub struct State {
  pub environment: BTreeMap<String, String>,
  pub prompt: String,
  pub unicode_supported: bool,
  pub history: Vec<String>,
  pub commands: Vec<crate::Command>,
  pub last_exit_code: i32,
}

impl State {
  pub(crate) fn new(commands: &[crate::Command], unicode_supported: bool) -> Self {
    Self {
      environment: {
        let mut environment = BTreeMap::new();
        environment.insert("last-exit-code".into(), "0".into());
        environment
      },
      unicode_supported,
      prompt: if unicode_supported { "❯ " } else { "> " }.into(),
      commands: commands.to_vec(),
      history: Vec::new(),
      last_exit_code: 0,
    }
  }

  pub fn set_environment_variable(&mut self, key: impl AsRef<str>, value: impl AsRef<str>) {
    self
      .environment
      .insert(key.as_ref().into(), value.as_ref().into());

    #[cfg(feature = "std")]
    std::env::set_var(key.as_ref(), value.as_ref());
  }

  pub fn set_last_exit_code(&mut self, code: i32) {
    self.last_exit_code = code;
    self.set_environment_variable("last-exit-code", "0");
  }
}
