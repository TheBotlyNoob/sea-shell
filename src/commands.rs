pub(crate) mod echo;

lazy_static::lazy_static! {
  pub(crate) static ref COMMANDS: [Box<dyn crate::CommandHandler>; 1] =
    [Box::new(echo::EchoCommand)];
}
