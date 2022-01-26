use std::{
  error::Error,
  io::{stdin, stdout, Write as _},
};

pub(crate) mod commands;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
  println!("Hello, world!");
  let prompt = "$ ";

  loop {
    print!("{}", prompt);
    stdout().flush().unwrap();
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    handle_command(input);
  }
}

pub(crate) trait CommandHandler: Sync {
  fn name(&self) -> String;

  fn handle(&self, args: Vec<String>) -> i32;
}

fn handle_command(command: String) {
  let cmd = command
    .split_whitespace()
    .map(|arg| arg.trim_end().to_owned())
    .collect::<Vec<String>>();

  for command in commands::COMMANDS.iter() {
    if command.name() == cmd[0] {
      command.handle(cmd.iter().skip(1).cloned().collect());
    }
  }
}
