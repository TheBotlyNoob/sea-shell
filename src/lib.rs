pub mod commands;
pub mod parser;

pub fn handle_command(command: String) {
  let cmd = command
    .split_whitespace()
    .map(|arg| arg.trim_end().into())
    .collect::<Vec<String>>();

  let command_name = &cmd[0];

  let code = match state::commands()
    .iter()
    .find(|command| command.names().contains(command_name))
  {
    Some(command) => {
      println!("{}: executing...", command_name);

      command.handle(cmd.iter().skip(1).collect())
    }
    None => {
      println!("{}: command not found", cmd[0]);

      1
    }
  };

  state::history_mut().push(cmd);

  state::environment_mut().insert("?".into(), code.to_string());
}

pub trait CommandHandler: Sync + Send + 'static {
  fn names(&self) -> Vec<String>;

  fn handle(&self, args: Vec<&String>) -> i32;

  fn help(&self) -> String {
    "No Help For This Command".into()
  }
}

pub mod state {
  use once_cell::sync::Lazy;
  use std::{
    collections::HashMap,
    sync::{RwLock, RwLockReadGuard, RwLockWriteGuard},
  };

  static ENVIRONMENT: Lazy<RwLock<HashMap<String, String>>> = Lazy::new(|| {
    let mut hashmap = HashMap::new();
    hashmap.insert("?".into(), "0".into());
    RwLock::new(hashmap)
  });

  #[inline(always)]
  pub fn environment() -> RwLockReadGuard<'static, HashMap<String, String>> {
    ENVIRONMENT.read().unwrap()
  }

  #[inline(always)]
  pub fn environment_mut() -> RwLockWriteGuard<'static, HashMap<String, String>> {
    ENVIRONMENT.write().unwrap()
  }

  static PROMPT: Lazy<RwLock<String>> = Lazy::new(|| {
    if if std::env::consts::OS == "windows" {
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
    } {
      RwLock::new("❯ ".into())
    } else {
      RwLock::new("> ".into())
    }
  });

  #[inline(always)]
  pub fn prompt() -> RwLockReadGuard<'static, String> {
    PROMPT.read().unwrap()
  }

  #[inline(always)]
  pub fn prompt_mut() -> RwLockWriteGuard<'static, String> {
    PROMPT.write().unwrap()
  }

  static HISTORY: Lazy<RwLock<Vec<Vec<String>>>> = Lazy::new(|| RwLock::new(Vec::new()));

  #[inline(always)]
  pub fn history() -> RwLockReadGuard<'static, Vec<Vec<String>>> {
    HISTORY.read().unwrap()
  }

  #[inline(always)]
  pub fn history_mut() -> RwLockWriteGuard<'static, Vec<Vec<String>>> {
    HISTORY.write().unwrap()
  }

  static COMMANDS: Lazy<RwLock<Vec<Box<dyn crate::CommandHandler>>>> =
    Lazy::new(|| RwLock::new(vec![Box::new(crate::commands::ExitCommand)]));

  #[inline(always)]
  pub fn commands() -> RwLockReadGuard<'static, Vec<Box<dyn crate::CommandHandler>>> {
    COMMANDS.read().unwrap()
  }

  #[inline(always)]
  pub fn commands_mut() -> RwLockWriteGuard<'static, Vec<Box<dyn crate::CommandHandler>>> {
    COMMANDS.write().unwrap()
  }
}

#[cfg(test)]
mod tests {
  use crate::parser::*;

  #[test]
  fn parse_number() {
    assert_eq!("123".parse::<Number>().unwrap(), Number(123));
  }

  #[test]
  fn parse_add_op() {
    assert_eq!(MathOperator::from("+"), Some(MathOperator::Add));
  }

  #[test]
  fn parse_sub_op() {
    assert_eq!(MathOperator::from("-"), Some(MathOperator::Sub));
  }

  #[test]
  fn parse_mul_op() {
    assert_eq!(MathOperator::from("*"), Some(MathOperator::Mul));
  }

  #[test]
  fn parse_div_op() {
    assert_eq!(MathOperator::from("/"), Some(MathOperator::Div));
  }

  #[test]
  fn invalid_op() {
    assert_eq!(MathOperator::from(""), None)
  }
}
