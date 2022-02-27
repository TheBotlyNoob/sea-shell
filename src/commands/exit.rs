pub const EXIT_COMMAND: crate::Command = crate::Command {
  name: "exit",
  handler: |mut ctx, args| {
    Box::pin(async move {
      let code = args.get(0).map_or_else(
        || 0,
        |raw_exit_code| raw_exit_code.parse::<i32>().unwrap_or(0),
      );

      if let Some(exit_handler) = ctx.exit_handler.take() {
        (exit_handler)(code, ctx);
      }

      (None, code)
    })
  },
};
