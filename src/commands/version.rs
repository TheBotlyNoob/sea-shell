use crate::re_exports::*;

pub const VERSION_COMMAND: crate::Command = crate::Command {
  name: "version",
  description: "Display the version",
  args: &[],
  handler: |ctx, _args| {
    create_log_from_logger!(ctx.logger, true);

    Box::pin(async move {
      log!(raw, crate::VERSION);

      (Some(ctx), 0)
    })
  },
};
