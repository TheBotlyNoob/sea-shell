pub const HELP_COMMAND: crate::Command = crate::Command {
  name: "help",
  handler: |ctx, _args| {
    Box::pin(async move {
      ctx
        .logger
        .raw(&format!("pirs version {}\n", crate::VERSION));

      for command in &ctx.state.commands {
        ctx.logger.raw(command.name);
      }

      (Some(ctx), 0)
    })
  },
};
