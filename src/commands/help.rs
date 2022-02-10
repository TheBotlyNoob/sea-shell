pub const HELP_COMMAND: crate::Command = crate::Command {
  name: "help",
  handler: |ctx, args| {
    if args.is_empty() {
      for command in &ctx.state.commands {
        ctx.logger.raw(&format!(
          "{}:\n  {}\n",
          command.name,
          // command.clap(ctx),
          ""
        ));
      }
    }

    0
  },
};
