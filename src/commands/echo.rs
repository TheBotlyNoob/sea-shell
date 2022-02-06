#[derive(Debug)]
pub struct EchoCommand;

impl crate::CommandHandler for EchoCommand {
  fn names(&self, _ctx: &crate::Pirs) -> Vec<&str> {
    vec!["echo"]
  }

  fn handle(&self, args: Vec<&str>, ctx: &crate::Pirs) -> i32 {
    ctx.logger.raw(&args.join(" "));

    0
  }
}
