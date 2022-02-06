use std::format as f;

pub mod commands;
pub mod state;

pub(crate) mod lexer;
pub(crate) mod parser;

pub fn handle_command(
  command: String,
  #[cfg(not(feature = "use-default-logger"))] logger: impl Logger,
  #[cfg(feature = "use-default-logger")] log_level: LogLevel,
) {
  #[cfg(feature = "use-default-logger")]
  state::LOGGER
    .set(Box::new(default_logger::DefaultLogger(log_level)))
    .ok();

  #[cfg(not(feature = "use-default-logger"))]
  state::LOGGER.set(Box::new(logger)).ok();

  let expr = parser::generate_expression_tree(lexer::get_token_stream(&command));

  println!("{:#?}", expr);

  let cmd = command
    .split_whitespace()
    .map(|arg| arg.trim_end().into())
    .collect::<Vec<String>>();

  let command_name = &cmd[0];

  let code = match state::commands()
    .iter()
    .find(|command| command.names().contains(&&**command_name))
  {
    Some(command) => {
      state::logger().debug(&f!("executing: {}...", command_name));

      command.handle(cmd.iter().skip(1).map(|arg| &**arg).collect())
    }
    None => {
      state::logger().error(&f!("command not found: {}", cmd[0]));

      1
    }
  };

  state::history_mut().push(cmd);

  state::environment_mut().insert("?".into(), code.to_string());
}

pub trait CommandHandler: Sync + Send + 'static {
  fn names(&self) -> Vec<&str>;

  fn handle(&self, args: Vec<&str>) -> i32;

  fn help(&self) -> &'static str {
    "No Help For This Command"
  }
}

pub trait Logger: Sync + Send + 'static {
  fn debug(&self, message: &dyn AsRef<str>);

  fn info(&self, message: &dyn AsRef<str>);

  fn warn(&self, message: &dyn AsRef<str>);

  fn error(&self, message: &dyn AsRef<str>);

  fn raw(&self, message: &dyn AsRef<str>);
}

#[cfg(feature = "use-default-logger")]
pub use default_logger::LogLevel;

#[cfg(feature = "use-default-logger")]
mod default_logger {
  use owo_colors::OwoColorize;

  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
  }

  pub struct DefaultLogger(pub LogLevel);

  impl DefaultLogger {
    pub fn log_level(&self) -> u8 {
      if let Ok(level) = std::env::var("LOG_LEVEL") {
        match &*level.to_lowercase() {
          "debug" => 4,
          "info" => 3,
          "warn" => 2,
          _ => 1,
        }
      } else {
        match self.0 {
          LogLevel::Debug => 4,
          LogLevel::Info => 3,
          LogLevel::Warn => 2,
          LogLevel::Error => 1,
        }
      }
    }
  }

  impl super::Logger for DefaultLogger {
    fn debug(&self, message: &dyn AsRef<str>) {
      if self.log_level() >= 4 {
        println!("[{}]: {}", "debug".blue(), message.as_ref());
      }
    }

    fn info(&self, message: &dyn AsRef<str>) {
      if self.log_level() >= 3 {
        println!("[{}]: {}", "info".green(), message.as_ref());
      }
    }

    fn warn(&self, message: &dyn AsRef<str>) {
      if self.log_level() >= 2 {
        println!("[{}]: {}", "warn".yellow(), message.as_ref());
      }
    }

    fn error(&self, message: &dyn AsRef<str>) {
      if self.log_level() >= 1 {
        println!("[{}]: {}", "error".bright_red(), message.as_ref());
      }
    }

    fn raw(&self, message: &dyn AsRef<str>) {
      println!("{}", message.as_ref());
    }
  }
}
