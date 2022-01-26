pub(crate) struct EnvCommand;

impl crate::CommandHandler for EnvCommand {
  fn name(&self) -> String {
    "env".into()
  }

  fn handle(&self, _args: Vec<String>) -> i32 {
    for (key, value) in crate::ENVIRON.iter() {
      println!("{key}: {value}");
    }

    0
  }
}
