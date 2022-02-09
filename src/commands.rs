mod echo;
mod exit;
mod help;

pub static BUILT_IN_COMMANDS: &[crate::Command] =
  &[exit::EXIT_COMMAND, echo::ECHO_COMMAND, help::HELP_COMMAND];
