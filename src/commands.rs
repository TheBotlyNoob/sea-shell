mod exit;
pub use exit::ExitCommand;

mod echo;
pub use echo::EchoCommand;

pub static BUILT_IN_COMMANDS: fn() -> Vec<Box<dyn crate::CommandHandler>> =
  || vec![Box::new(ExitCommand), Box::new(EchoCommand)];
