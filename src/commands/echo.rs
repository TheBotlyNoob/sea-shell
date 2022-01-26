pub(crate) struct EchoCommand;

impl crate::CommandHandler for EchoCommand {
  fn name(&self) -> String {
    "echo".into()
  }

  fn handle(&self, args: Vec<String>) -> i32 {
    println!("{}", args.join(" "));

    0
  }
}
