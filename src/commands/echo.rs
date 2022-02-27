pub const ECHO_COMMAND: crate::Command = crate::Command {
  name: "echo",
  handler: |ctx, args| {
    Box::pin({
      ctx.logger.raw(&args.join(" "));

      async { (Some(ctx), 0) }
    })
  },
};
