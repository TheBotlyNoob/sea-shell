#[derive(Debug)]
pub struct HelpCommand;

impl crate::CommandHandler for HelpCommand {
  fn name(&self, _ctx: &crate::Pirs) -> &str {
    "help"
  }

  fn handle(&self, args: Vec<&str>, ctx: &crate::Pirs) -> i32 {
    if args.is_empty() {
      for command in &ctx.state.commands {
        ctx.logger.raw(&format!(
          "{}:\n  {}\n",
          command.name(ctx),
          command.clap(ctx),
        ));
      }
    }

    0
  }
}
