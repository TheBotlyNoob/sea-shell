#[derive(Debug)]
pub struct ExitCommand;

impl crate::CommandHandler for ExitCommand {
  fn names(&self, _ctx: &crate::Pirs) -> Vec<&str> {
    vec!["exit"]
  }

  fn handle(&self, args: Vec<&str>, ctx: &crate::Pirs) -> i32 {
    let mut code = 0;

    if !args.is_empty() {
      code = args[0].parse::<i32>().unwrap_or(0);
    }

    (ctx.exit_handler)(code);

    code
  }
}
