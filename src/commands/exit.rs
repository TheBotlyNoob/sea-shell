pub(crate) struct ExitCommand;

impl crate::CommandHandler for ExitCommand {
  fn name(&self) -> String {
    "exit".into()
  }

  fn handle(&self, args: Vec<String>) -> i32 {
    let mut code = 0;

    if !args.is_empty() {
      code = args[0].parse::<i32>().unwrap_or_else(|_| {
        eprintln!("Failed to parse exit code, exiting with code 0");

        0
      });
    }

    std::process::exit(code);
  }
}
