pub const EXIT_COMMAND: crate::Command = crate::Command {
  name: "exit",
  handler: |ctx, args| {
    let mut code = 0;

    if !args.is_empty() {
      code = args[0].parse::<i32>().unwrap_or(0);
    }

    (ctx.exit_handler)(code);

    code
  },
};
