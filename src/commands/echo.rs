pub const ECHO_COMMAND: crate::Command = crate::Command {
  name: "echo",
  handler: |ctx, args| {
    ctx.logger.raw(&args.join(" "));

    0
  },
};
