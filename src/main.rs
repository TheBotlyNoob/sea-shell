use once_cell::sync::Lazy;
use std::{
  collections::HashMap,
  error::Error,
  io::{stdin, stdout, Write as _},
  sync::Mutex,
  ops::{Deref, DerefMut}
};

pub(crate) mod commands;

pub(crate) static mut STATE: Lazy<Mutex<State>> = Lazy::new(|| Mutex::new(State::new()));

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
  tracing_subscriber::fmt().pretty().without_time().init();

  loop {
    print!("{}", STATE.lock().unwrap().prompt);
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

  STATE.lock().unwrap().set_exit_code(code);
}

pub(crate) struct State {
  pub environment: HashMap<String, String>,
  pub exit_code: i32,
  pub prompt: String,
}

impl State {
  fn new() -> Self {
    Self {
      environment: {
        let mut environment = HashMap::new();
        environment.insert("?".into(), "0".into());
        environment
      },
      exit_code: 0,
      prompt: "❯ ".into(),
    }
  }

  fn set_exit_code(&mut self, exit_code: i32) -> Option<i32> {
    self.environment.insert("?".into(), exit_code.to_string()).map(|prev_exit_code| prev_exit_code.parse::<i32>().unwrap())
  }
}
