pub const HELP_COMMAND: crate::Command = crate::Command {
  name: "help",
  handler: |ctx, _args| {
    ctx
      .logger
      .raw(&format!("pirs version {}\n", crate::VERSION));

    for command in &ctx.state.commands {
      ctx.logger.raw(command.name);
    }

    0
  },
};
