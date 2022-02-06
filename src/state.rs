use std::collections::HashMap;

#[derive(Debug)]
pub struct State {
  pub environment: HashMap<String, String>,
  pub prompt: String,
  pub supports_unicode: bool,
  pub history: Vec<String>,
  pub commands: Vec<Box<dyn crate::CommandHandler>>,
  pub last_exit_code: i32,
}

impl State {
  pub fn new(commands: Vec<Box<dyn crate::CommandHandler>>) -> Self {
    let supports_unicode = if std::env::consts::OS == "windows" {
      // Just a handful of things!
      std::env::var("CI").is_ok()
  || std::env::var("WT_SESSION").is_ok() // Windows Terminal
  || std::env::var("ConEmuTask") == Ok("{cmd:Cmder}".into()) // ConEmu and cmder
  || std::env::var("TERM_PROGRAM") == Ok("vscode".into())
  || std::env::var("TERM") == Ok("xterm-256color".into())
  || std::env::var("TERM") == Ok("alacritty".into())
    } else if std::env::var("TERM") == Ok("linux".into()) {
      // Linux kernel console. Maybe redundant with the below?...
      false
    } else {
      // From https://github.com/iarna/has-unicode/blob/master/index.js
      let ctype = std::env::var("LC_ALL")
        .or_else(|_| std::env::var("LC_CTYPE"))
        .or_else(|_| std::env::var("LANG"))
        .unwrap_or_else(|_| "".into())
        .to_uppercase();
      ctype.ends_with("UTF8") || ctype.ends_with("UTF-8")
    };

    Self {
      environment: {
        let mut environment = HashMap::new();
        environment.insert("last-exit-code".into(), "0".into());
        environment
      },
      supports_unicode,
      prompt: if supports_unicode { "❯ " } else { "> " }.into(),
      commands,
      history: Vec::new(),
      last_exit_code: 0,
    }
  }

  pub fn set_last_exit_code(&mut self, code: i32) {
    self.last_exit_code = code;
    self.environment.insert("last-exit-code".into(), "0".into());
  }
}
