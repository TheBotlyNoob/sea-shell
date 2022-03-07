pub const ECHO_COMMAND: crate::Command = crate::Command {
  name: "version",
  handler: |ctx, args| {
    crate::alloc::boxed::Box::pin(async move {
      ctx.logger.raw(&args.join(" "));

      (Some(ctx), 0)
    })
  },
};
