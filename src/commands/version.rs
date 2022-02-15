pub const VERSION_COMMAND: crate::Command = crate::Command {
  name: "version",
  handler: |ctx, _args| {
    ctx.logger.raw(&format!("pirs version: {}", crate::VERSION));

    0
  },
};
