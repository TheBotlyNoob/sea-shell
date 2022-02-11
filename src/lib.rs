use std::format as f;

mod state;
use lexer::tokenize_command;
pub use state::State;

pub mod commands;

pub(crate) mod lexer;

pub struct Pirs {
  pub state: State,
  exit_handler: fn(i32),
  pub logger: Box<dyn Logger>,
}

impl Pirs {
  pub fn new(
    exit_handler: fn(i32),
    #[cfg(not(feature = "use-default-logger"))] logger: impl Logger,
    #[cfg(feature = "use-default-logger")] log_level: LogLevel,
  ) -> Self {
    let supports_unicode = supports_unicode::on(supports_unicode::Stream::Stdout);

    #[cfg(not(feature = "use-default-logger"))]
    let logger = Box::new(logger);
    #[cfg(feature = "use-default-logger")]
    let logger = Box::new(default_logger::DefaultLogger(log_level, supports_unicode));

    logger.info(&"Welcome to Pirs, A POSIX-like shell written in Rust");
    logger.info(&"Type 'help' for a list of commands");
    logger.raw(&"\n");

    Self {
      exit_handler,
      state: State::new(commands::BUILT_IN_COMMANDS, supports_unicode),
      logger,
    }
  }

  pub fn handle_command(&mut self, input: impl AsRef<str>) {
    let tokenized = tokenize_command(input);

    for token in tokenized {
      let code = match self
        .state
        .commands
        .iter()
        .find(|command| command.name == token[0])
      {
        Some(command) => {
          self.logger.debug(&f!("executing: {}...", token[0]));

          (command.handler)(self, token.iter().skip(1).map(|arg| &**arg).collect())
        }
        None => {
          self.logger.error(&f!("command not found: {}", token[0]));

          1
        }
      };

      self.state.set_last_exit_code(code);
    }
  }
}

#[derive(Clone)]
pub struct Command {
  name: &'static str,
  handler: fn(&Pirs, Vec<&str>) -> i32,
}

pub trait Logger: Sync + std::fmt::Debug + Send + 'static {
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

  #[derive(Debug)]
  pub struct DefaultLogger(pub LogLevel, pub bool);

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
        println!(
          "[{}]: {}",
          if self.1 {
            "debug".bright_blue().to_string()
          } else {
            "debug".into()
          },
          message.as_ref()
        );
      }
    }

    fn info(&self, message: &dyn AsRef<str>) {
      if self.log_level() >= 3 {
        println!(
          "[{}]: {}",
          if self.1 {
            "info".green().to_string()
          } else {
            "info".into()
          },
          message.as_ref()
        );
      }
    }

    fn warn(&self, message: &dyn AsRef<str>) {
      if self.log_level() >= 2 {
        println!(
          "[{}]: {}",
          if self.1 {
            "warn".yellow().to_string()
          } else {
            "warn".into()
          },
          message.as_ref()
        );
      }
    }

    fn error(&self, message: &dyn AsRef<str>) {
      if self.log_level() >= 1 {
        println!(
          "[{}]: {}",
          if self.1 {
            "error".bright_red().to_string()
          } else {
            "error".into()
          },
          message.as_ref()
        );
      }
    }

    fn raw(&self, message: &dyn AsRef<str>) {
      println!("{}", message.as_ref());
    }
  }
}
