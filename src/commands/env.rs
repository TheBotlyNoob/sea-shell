pub(crate) struct EnvCommand;

impl crate::CommandHandler for EnvCommand {
  fn name(&self) -> String {
    "env".into()
  }

  fn handle(&self, args: Vec<String>) -> i32 {
    if args.is_empty() {
      for (key, value) in &*crate::state::environment() {
        println!("{key}: {value}");
      }
    } else {
      let env_var = &args[0];
      match crate::state::environment()
        .iter()
        .find(|(key, _value)| &env_var == key)
      {
        Some((_key, value)) => println!("{value}"),
        None => tracing::error!("environment variable {env_var} not found"),
      }
    }

    0
  }
}
