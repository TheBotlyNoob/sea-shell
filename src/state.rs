use std::collections::HashMap;

#[derive(Clone)]
pub struct State {
  pub environment: HashMap<String, String>,
  pub prompt: String,
  pub supports_unicode: bool,
  pub history: Vec<String>,
  pub commands: Vec<crate::Command>,
  pub last_exit_code: i32,
}

impl State {
  pub(crate) fn new(commands: &[crate::Command], supports_unicode: bool) -> Self {
    Self {
      environment: {
        let mut environment = HashMap::new();
        environment.insert("last-exit-code".into(), "0".into());
        environment
      },
      supports_unicode,
      prompt: if supports_unicode { "❯ " } else { "> " }.into(),
      commands: commands.to_vec(),
      history: Vec::new(),
      last_exit_code: 0,
    }
  }

  pub fn set_last_exit_code(&mut self, code: i32) {
    self.last_exit_code = code;
    self.environment.insert("last-exit-code".into(), "0".into());
  }
}
