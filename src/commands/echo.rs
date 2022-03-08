pub const ECHO_COMMAND: crate::Command = crate::Command {
  name: "version",
  handler: |ctx, args| {
    crate::logger::create_logger_from_logger!(ctx.logger, true);

    crate::alloc::boxed::Box::pin(async move {
      log!(raw, "{}", args.join(" "));

      (Some(ctx), 0)
    })
  },
};
