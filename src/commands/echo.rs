pub const ECHO_COMMAND: crate::Command = crate::Command {
  name: "echo",
  handler: |ctx, args| {
    ctx.logger.raw(
      &args
        .iter()
        .skip(1)
        .map(|arg| &*arg.raw_token)
        .collect::<Vec<&str>>()
        .join(" "),
    );

    0
  },
};
