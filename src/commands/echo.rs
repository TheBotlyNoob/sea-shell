pub const ECHO_COMMAND: crate::Command = crate::Command {
  name: "echo",
  handler: |ctx, args| {
    ctx.logger.raw(
      &args
        .iter()
        .map(|arg| &**arg)
        .collect::<Vec<&str>>()
        .join(" "),
    );

    0
  },
};
