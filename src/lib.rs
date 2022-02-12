use std::format as f;

mod state;
pub use state::State;

pub mod commands;

pub(crate) mod lexer;

pub struct Pirs<'a> {
  pub state: State,
  exit_handler: Box<dyn Fn(i32) + 'a>,
  pub logger: Box<dyn Logger>,
}

impl<'a> Pirs<'a> {
  pub fn new(
    exit_handler: impl Fn(i32) + 'a,
    #[cfg(not(feature = "use-default-logger"))] logger: impl Logger,
    #[cfg(feature = "use-default-logger")] log_level: LogLevel,
  ) -> Self {
    let supports_unicode = supports_unicode::on(supports_unicode::Stream::Stdout);

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

  pub fn handle_command(&mut self, input: impl AsRef<str>) {
    let tokenized = lexer::tokenize_command(input);

    println!("{:#?}", tokenized);

    for tokens in tokenized {
      for token in &tokens {
        if let lexer::TokenValue::Command(given_command) = &token.token_value {
          let code = match self
            .state
            .commands
            .iter()
            .find(|command| command.name == given_command)
          {
            Some(command) => {
              self.logger.debug(&f!("executing: {}...", given_command));

              (command.handler)(self, tokens.clone())
            }
            None => {
              self
                .logger
                .error(&f!("command not found: {}", given_command));

              1
            }
          };

          self.state.set_last_exit_code(code);
        }
      }
    }
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
  handler: fn(&Pirs, Vec<lexer::Token>) -> i32,
}

pub trait Logger: Sync + std::fmt::Debug + Send + 'static {
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
