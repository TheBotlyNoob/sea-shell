#[derive(Debug)]
pub struct HelpCommand;

impl crate::CommandHandler for HelpCommand {
  fn names(&self, _ctx: &crate::Pirs) -> Vec<&str> {
    vec!["help", "man"]
  }

  fn handle(&self, args: Vec<&str>, ctx: &crate::Pirs) -> i32 {
    if args.is_empty() {
      for command in &ctx.state.commands {
        let command_names = command.names(ctx);

        ctx.logger.raw(&format!(
          "{}:\n{}  Description: {}\n",
          command_names[0],
          if command_names.len() > 1 {
            format!("  Aliases: {}\n", command_names[1..].join(", "))
          } else {
            String::new()
          },
          command.description(ctx)
        ));
      }
    }

    0
  }
}
