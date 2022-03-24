use crate::re_exports::*;

pub const ECHO_COMMAND: crate::Command = crate::Command {
  name: "echo",
  args: &[Arg {
    name: "text",
    arg_type: ArgType::Array(&ArgType::String, " "),
    is_required: true,
    ..Arg::default()
  }
  .check()],
  description: "Echo back the arguments",
  handler: |ctx, args| {
    create_log_from_logger!(ctx.logger, true);

    Box::pin(async move {
      log!(raw, args.iter().map(|arg| arg.to_string()).join(" "));

      (Some(ctx), 0)
    })
  },
};
