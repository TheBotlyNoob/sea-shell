///
#[derive(argh::FromArgs, Debug)]
struct Args {}

pub const ECHO_COMMAND: crate::Command = crate::Command {
  name: "echo",
  handler: |args, ctx| {
    ctx.logger.raw(&args.join(" "));

    0
  },
};
