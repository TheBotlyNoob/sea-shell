pub const EXIT_COMMAND: crate::Command = crate::Command {
  name: "exit",
  handler: |ctx, args| {
    let code = args.get(0).map_or_else(
      || 0,
      |raw_exit_code| raw_exit_code.parse::<i32>().unwrap_or(0),
    );

    (ctx.exit_handler)(code);

    code
  },
};
