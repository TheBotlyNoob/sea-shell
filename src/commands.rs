pub(crate) mod echo;
pub(crate) mod exit;

lazy_static::lazy_static! {
  pub(crate) static ref COMMANDS: [Box<dyn crate::CommandHandler>; 2] =
    [Box::new(echo::EchoCommand), Box::new(exit::ExitCommand)];
}
