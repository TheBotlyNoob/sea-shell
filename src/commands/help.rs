pub const HELP_COMMAND: crate::Command = crate::Command {
  name: "help",
  handler: |ctx, _args| {
    for command in &ctx.state.commands {
      ctx.logger.raw(command.name);
    }

    0
  },
};
