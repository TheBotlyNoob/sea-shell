pub struct EchoCommand;

impl crate::CommandHandler for EchoCommand {
  fn names(&self) -> Vec<&'static str> {
    vec!["echo"]
  }

  fn handle(&self, args: Vec<&str>) -> i32 {
    crate::state::logger().raw(&args.join(" "));

    0
  }
}
