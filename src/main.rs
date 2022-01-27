use std::{
  error::Error,
  io::{stdin, stdout, Write as _},
};

pub(crate) mod commands;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
  tracing_subscriber::fmt().pretty().without_time().init();

  loop {
    print!("{}", state::prompt());
    stdout().flush().unwrap();
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    handle_command(input);
  }
}

pub(crate) trait CommandHandler: Sync + Send + 'static {
  fn name(&self) -> String;

  fn handle(&self, args: Vec<String>) -> i32;

  fn help(&self) -> String {
    "No Help For This Command".into()
  }
}

fn handle_command(command: String) {
  let cmd = command
    .split_whitespace()
    .map(|arg| arg.trim_end().to_owned())
    .collect::<Vec<String>>();

  let code = match commands::COMMANDS
    .iter()
    .find(|command| command.name() == cmd[0])
  {
    Some(command) => {
      tracing::trace!("executing command {}", command.name());

      command.handle(cmd.iter().skip(1).cloned().collect())
    }
    None => {
      tracing::error!("command {} not found", cmd[0]);

      1
    }
  };

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

  pub(crate) fn environment() -> MutexGuard<'static, HashMap<String, String>> {
    ENVIRONMENT.lock().unwrap()
  }

  static PROMPT: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new("❯ ".into()));

  pub(crate) fn prompt() -> MutexGuard<'static, String> {
    PROMPT.lock().unwrap()
  }
}
