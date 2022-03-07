#![cfg_attr(not(feature = "std"), no_std)]

pub(crate) mod re_exports {
  #[cfg(not(feature = "std"))]
  pub extern crate alloc;
  #[cfg(feature = "std")]
  pub use std as alloc;

  pub use super::alloc::{boxed::Box, format, pin::Pin, sync::Arc, vec::Vec};
  pub use core::future::Future;
}

use core::future::Future as Future_;
use re_exports::*;

mod state;

pub use state::State;

pub mod commands;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

#[derive(Clone)]
pub struct SeaShell<'a> {
  pub state: State,
  exit_handler: Arc<Box<dyn Fn(i32, &mut Self) + 'a>>,
  pub logger: Arc<Box<dyn Logger + 'a>>,
}

impl<'a> SeaShell<'a> {
  pub fn new(exit_handler: impl Fn(i32, &mut Self) + 'a, logger: impl Logger + 'a) -> Self {
    let supports_unicode = supports_unicode::on(supports_unicode::Stream::Stdout);

    logger.info(&format!("Welcome to pirs version: {}", VERSION));
    logger.info(DESCRIPTION);
    logger.info("Type 'help' for a list of commands");
    logger.raw("\n");

    Self {
      exit_handler: Arc::new(Box::new(exit_handler)),
      state: State::new(commands::BUILT_IN_COMMANDS, supports_unicode),
      logger: Arc::new(Box::new(logger)),
    }
  }

  pub async fn handle_command(&mut self, input: impl AsRef<str>) {
    let input_ = input.as_ref();

    let input = input_
      .split_whitespace()
      .filter_map(|input| {
        let trimmed = input.trim();

        if trimmed.is_empty() {
          None
        } else {
          Some(trimmed.into())
        }
      })
      .collect::<Vec<String>>();

    if input.is_empty() {
      return;
    }

    self.state.history.push(input_.into());

    let code = match self.get_command(&input[0]) {
      Some(command) => {
        self.logger.debug(&format!("executing: {}...", input[0]));

        let out = (command.handler)(self.clone(), input.into_iter().skip(1).collect()).await;

        if let Some(self_) = out.0 {
          *self = self_;
        } else {
          return;
        }

        out.1
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

#[derive(Clone)]
pub struct Command {
  name: &'static str,
  #[allow(clippy::type_complexity)]
  handler: for<'a> fn(SeaShell<'a>, Vec<String>) -> Future<'a, (Option<SeaShell<'a>>, i32)>,
}

pub trait Logger {
  fn debug(&self, message: &str);

  fn info(&self, message: &str);

  fn warn(&self, message: &str);

  fn error(&self, message: &str);

  fn raw(&self, message: &str);
}

pub(crate) type Future<'a, T> = Pin<Box<dyn Future_<Output = T> + 'a>>;

#[cfg(feature = "default-logger")]
pub mod default_logger;
