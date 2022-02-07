mod exit;
pub use exit::ExitCommand;

mod echo;
pub use echo::EchoCommand;

mod help;
pub use help::HelpCommand;

pub static BUILT_IN_COMMANDS: fn() -> Vec<Box<dyn crate::CommandHandler>> = || {
  vec![
    Box::new(ExitCommand),
    Box::new(EchoCommand),
    Box::new(HelpCommand),
  ]
};
