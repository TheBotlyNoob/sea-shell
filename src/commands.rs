pub(crate) mod echo;
pub(crate) mod env;
pub(crate) mod exit;

use once_cell::sync::Lazy;

pub(crate) static COMMANDS: Lazy<[Box<dyn crate::CommandHandler>; 3]> = Lazy::new(|| {
  [
    Box::new(echo::EchoCommand),
    Box::new(exit::ExitCommand),
    Box::new(env::EnvCommand),
  ]
});
