mod exit;

use once_cell::sync::Lazy;

pub static COMMANDS: Lazy<Vec<Box<dyn crate::CommandHandler>>> =
  Lazy::new(|| vec![Box::new(exit::ExitCommand)]);
