use crate::re_exports::*;

#[derive(Clone)]
pub struct State {
  pub environment: BTreeMap<String, String>,
  pub prompt: String,
  pub unicode_supported: bool,
  pub history: Vec<String>,
  pub commands: Vec<Arc<crate::Command>>,
}

impl State {
  pub(crate) fn new(commands: &[crate::Command], unicode_supported: bool) -> Self {
    Self {
      environment: BTreeMap::new(),
      unicode_supported,
      prompt: if unicode_supported { "❯ " } else { "> " }.to_owned(),
      commands: commands.iter().map(|command| Arc::new(*command)).collect(),
      history: Vec::new(),
    }
  }

  pub fn set_environment_variable(&mut self, key: impl AsRef<str>, value: impl AsRef<str>) {
    self
      .environment
      .insert(key.as_ref().to_owned(), value.as_ref().to_owned());

    #[cfg(feature = "std")]
    std::env::set_var(key.as_ref(), value.as_ref());
  }
}
