use std::{
  error::Error,
  io::{stdin, stdout, Write},
};

pub(crate) mod commands;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
  tracing_subscriber::fmt().pretty().without_time().init();

  loop {
    handle_command({
      print!("{}", state::prompt());
      stdout().flush()?;
      let mut command = String::new();
      stdin().read_line(&mut command)?;
      command
    });
  }
}

pub(crate) trait CommandHandler: Sync + Send + 'static {
  fn name(&self) -> String;

  fn handle(&self, args: Vec<&String>) -> i32;

  fn help(&self) -> String {
    "No Help For This Command".into()
  }
}

fn handle_command(command: String) {
  let cmd = command
    .split_whitespace()
    .map(|arg| arg.trim_end().into())
    .collect::<Vec<String>>();

  let code = match commands::COMMANDS
    .iter()
    .find(|command| command.name() == cmd[0])
  {
    Some(command) => {
      tracing::trace!("{}: executing...", command.name());

      command.handle(cmd.iter().skip(1).collect())
    }
    None => {
      tracing::error!("{}: command not found", cmd[0]);

      1
    }
  };

  state::history().push(cmd);

  state::environment().insert("?".into(), code.to_string());
}

pub(crate) mod state {
  use once_cell::sync::Lazy;
  use std::{
    collections::HashMap,
    sync::{Mutex, MutexGuard},
  };

  static ENVIRONMENT: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| {
    let mut hashmap = HashMap::new();
    hashmap.insert("?".into(), "0".into());
    Mutex::new(hashmap)
  });

  #[inline(always)]
  pub(crate) fn environment() -> MutexGuard<'static, HashMap<String, String>> {
    ENVIRONMENT.lock().unwrap()
  }

  static PROMPT: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new("❯ ".into()));

  #[inline(always)]
  pub(crate) fn prompt() -> MutexGuard<'static, String> {
    PROMPT.lock().unwrap()
  }

  static HISTORY: Lazy<Mutex<Vec<Vec<String>>>> = Lazy::new(|| Mutex::new(Vec::new()));

  #[inline(always)]
  pub(crate) fn history() -> MutexGuard<'static, Vec<Vec<String>>> {
    HISTORY.lock().unwrap()
  }
}
