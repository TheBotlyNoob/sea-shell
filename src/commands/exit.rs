use crate::re_exports::*;

pub const EXIT_COMMAND: crate::Command = crate::Command {
  name: "exit",
  description: "Exit Sea Shell",
  args: &[Arg {
    name: "code",
    is_flag: true,
    is_required: true,
    ..Arg::default()
  }],
  handler: |ctx, args| {
    create_logger_from_logger!(ctx.logger, true);

    Box::pin(async move {
      let code = args.get(0).map_or_else(
        || 0,
        |raw_exit_code| raw_exit_code.parse::<i32>().unwrap_or(0),
      );

      let out = (ctx.exit_handler)(code, ctx.clone());

      (out, code)
    })
  },
};
