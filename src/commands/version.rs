pub const VERSION_COMMAND: crate::Command = crate::Command {
  name: "version",
  handler: |ctx, _args| {
    Box::pin(async move {
      ctx.logger.raw(&format!("pirs version: {}", crate::VERSION));

      (Some(ctx), 0)
    })
  },
};
