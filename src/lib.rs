#![cfg_attr(not(feature = "std"), no_std)]

pub(crate) mod re_exports {
  #[cfg(not(feature = "std"))]
  pub extern crate alloc;
  #[cfg(feature = "std")]
  pub use std as alloc;

  pub use crate::logger::create_logger_from_logger;
  pub use alloc::{
    boxed::Box,
    collections::BTreeMap,
    format,
    string::{String, ToString},
    sync::Arc,
    vec::Vec,
  };
  pub use core::{
    fmt::{self, Display, Formatter, Result as FmtResult},
    future::Future,
    mem,
    pin::Pin,
  };
  pub use itertools::Itertools;
}

use core::future::Future as Future_;
use re_exports::*;

mod state;

pub use state::State;

pub mod commands;
pub mod logger;

pub(crate) mod macro_helpers;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

#[derive(Clone)]
pub struct SeaShell<'a> {
  pub state: State,
  #[allow(clippy::type_complexity)]
  pub exit_handler: Arc<Box<dyn Fn(i32, Self) -> Option<Self> + 'a>>,
  pub logger: Arc<Box<dyn logger::Logger + 'a>>,
}

impl<'a> SeaShell<'a> {
  pub fn new(
    exit_handler: impl Fn(i32, Self) -> Option<Self> + 'a,
    logger_: impl logger::Logger + 'a,
    unicode_supported: bool,
  ) -> Self {
    create_logger_from_logger!(logger_, true);

    log!(info, "Welcome to Sea Shell version: {}", VERSION);
    log!(info, DESCRIPTION);
    log!(info, "Type 'help' for a list of commands");
    log!();

    Self {
      exit_handler: Arc::new(Box::new(exit_handler)),
      state: State::new(commands::BUILT_IN_COMMANDS, unicode_supported),
      logger: Arc::new(Box::new(logger_)),
    }
  }

  pub async fn handle_command(&mut self, input: impl AsRef<str>) {
    let raw_input = input.as_ref().trim();
    let mut input = raw_input.split_whitespace();

    let command = input.next();

    if command.is_none() {
      return;
    }

    let raw_command = unsafe { command.unwrap_unchecked() };

    let command = self.get_command(raw_command);

    create_logger_from_logger!(self.logger, true);

    if command.is_none() {
      log!(error, "command not found: {}", raw_command);

      self.state.set_environment_variable("exit", "1".to_owned());

      return;
    }

    let command = unsafe { command.unwrap_unchecked() };

    let args = (command.parse_args)(input.collect());

    self.state.history.push(raw_input.into());

    log!(debug, "executing: {}...", command);

    let out = (command.handler)(self.clone(), args).await;

    if let Some(self_) = out.0 {
      *self = self_;
    }

    self
      .state
      .set_environment_variable("exit", out.1.to_string());
  }

  pub fn get_command(&self, command: impl AsRef<str>) -> Option<&Command> {
    let command = command.as_ref();

    self.state.commands.iter().find(|c| c.name == command)
  }
}

#[derive(Clone)]
pub struct Command {
  pub name: &'static str,
  pub description: &'static str,
  pub parse_args: fn(Vec<&str>) -> Box<dyn argwerk_no_std::TryIntoInput>,
  #[allow(clippy::type_complexity)]
  pub handler: for<'a> fn(
    SeaShell<'a>,
    Box<dyn argwerk_no_std::TryIntoInput>,
  ) -> Future<'a, (Option<SeaShell<'a>>, i32)>,
}

impl Display for Command {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    // let mut help_text = String::new();

    // help_text.push_str(self.name);

    // if !self.args.is_empty() {
    //   help_text.push(' ');

    //   for arg in self.args {
    //     help_text.push_str(&format!("{}", arg));
    //   }
    // }

    // help_text.push_str(": ");
    // help_text.push_str(self.description);

    write!(f, "{{}}",)
  }
}

pub(crate) type Future<'a, T> = Pin<Box<dyn Future_<Output = T> + 'a>>;
