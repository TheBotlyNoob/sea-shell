pub const EXIT_COMMAND: crate::Command = crate::Command {
  name: "exit",
  handler: |mut ctx, args| {
    Box::pin(async move {
      let code = args.get(0).map_or_else(
        || 0,
        |raw_exit_code| raw_exit_code.parse::<i32>().unwrap_or(0),
      );

      let exit_handler = ctx.exit_handler.clone();

      exit_handler(code, &mut ctx);

      (None, code)
    })
  },
};
