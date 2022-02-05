pub struct ExitCommand;

impl crate::CommandHandler for ExitCommand {
  fn names(&self) -> Vec<&'static str> {
    vec!["exit"]
  }

  fn handle(&self, args: Vec<&str>) -> i32 {
    let mut code = 0;

    if !args.is_empty() {
      code = args[0].parse::<i32>().unwrap_or(0);
    }

    std::process::exit(code);
  }
}
