mod state;
pub use state::State;

pub mod commands;

pub use supports_unicode__used_for_pirs as supports_unicode;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

pub struct Pirs<'a> {
  pub state: State,
  exit_handler: Box<dyn Fn(i32) + 'a>,
  pub logger: Box<dyn Logger + 'a>,
}

impl<'a> Pirs<'a> {
  pub fn new(exit_handler: impl Fn(i32) + 'a, logger: impl Logger + 'a) -> Self {
    let supports_unicode = supports_unicode::on(supports_unicode::Stream::Stdout);

    let logger = Box::new(logger);

    logger.info(&format!("Welcome to pirs version: {}", VERSION));
    logger.info(DESCRIPTION);
    logger.info("Type 'help' for a list of commands");
    logger.raw("\n");

    Self {
      exit_handler: Box::new(exit_handler),
      state: State::new(commands::BUILT_IN_COMMANDS, supports_unicode),
      logger,
    }
  }

  pub async fn handle_command(&mut self, input: &impl AsRef<str>) {
    let input = input
      .as_ref()
      .split_whitespace()
      .filter_map(|input| {
        let trimmed = input.trim();

        if trimmed.is_empty() {
          None
        } else {
          Some(trimmed)
        }
      })
      .collect::<Vec<&str>>();

    if input.is_empty() {
      return;
    }

    let code = match self.get_command(input[0]) {
      Some(command) => {
        self.logger.debug(&format!("executing: {}...", input[0]));

        (command.handler)(self, input.iter().skip(1).copied().collect())
      }
      None => {
        self
          .logger
          .error(&format!("command not found: {}", input[0]));

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

#[cfg(feature = "exit-on-drop")]
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

#[cfg(feature = "default-logger")]
pub mod default_logger;
