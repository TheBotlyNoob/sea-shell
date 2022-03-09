use crate::re_exports::*;

pub const ECHO_COMMAND: crate::Command = crate::Command {
  name: "echo",
  args: &[],
  description: "Echo back the arguments",
  handler: |ctx, args| {
    create_logger_from_logger!(ctx.logger, true);

    Box::pin(async move {
      log!(raw, args.join(" "));

      (Some(ctx), 0)
    })
  },
};
