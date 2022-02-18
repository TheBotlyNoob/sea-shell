use std::format as f;

mod state;
pub use state::State;

pub mod commands;

pub const VERSION: &str = "0.0.1";

pub struct Pirs<'a> {
  pub state: State,
  exit_handler: Box<dyn Fn(i32) + 'a>,
  pub logger: Box<dyn Logger + 'a>,
}

impl<'a> Pirs<'a> {
  pub fn new(
    exit_handler: impl Fn(i32) + 'a,
    #[cfg(not(feature = "use-default-logger"))] logger: impl Logger + 'a,
    #[cfg(feature = "use-default-logger")] log_level: LogLevel,
  ) -> Self {
    let supports_unicode =
      supports_unicode__used_for_pirs::on(supports_unicode__used_for_pirs::Stream::Stdout);

    #[cfg(not(feature = "use-default-logger"))]
    let logger = Box::new(logger);
    #[cfg(feature = "use-default-logger")]
    let logger = Box::new(default_logger::DefaultLogger(log_level, supports_unicode));

    logger.info("Welcome to Pirs, A portable POSIX-like shell written in Rust");
    logger.info("Type 'help' for a list of commands");
    logger.raw("\n");

    Self {
      exit_handler: Box::new(exit_handler),
      state: State::new(commands::BUILT_IN_COMMANDS, supports_unicode),
      logger,
    }
  }

  pub fn handle_command(&mut self, input: &impl AsRef<str>) {
    let input = input.as_ref().split_whitespace().collect::<Vec<&str>>();

    let code = match self.get_command(input[0]) {
      Some(command) => {
        self.logger.debug(&f!("executing: {}...", input[0]));

        (command.handler)(self, input.iter().skip(1).copied().collect())
      }
      None => {
        self.logger.error(&f!("command not found: {}", input[0]));

        1
      }
    };

    self.state.set_last_exit_code(code);
  }

  pub fn get_command(&self, command: impl AsRef<str>) -> Option<&Command> {
    let command = command.as_ref();

    self.state.commands.iter().find(|c| c.name == command)
  }
}

impl Drop for Pirs<'_> {
  fn drop(&mut self) {
    (self.exit_handler)(0);
  }
}

#[derive(Clone)]
pub struct Command {
  name: &'static str,
  handler: fn(&Pirs, Vec<&str>) -> i32,
}

pub trait Logger {
  fn debug(&self, message: &str);

  fn info(&self, message: &str);

  fn warn(&self, message: &str);

  fn error(&self, message: &str);

  fn raw(&self, message: &str);
}

#[cfg(feature = "use-default-logger")]
pub use default_logger::LogLevel;

#[cfg(feature = "use-default-logger")]
mod default_logger;
