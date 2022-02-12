use rustyline::{error::ReadlineError, Editor};

fn main() {
  let mut rl = Editor::<()>::new();

  let rl_history_file = format!("{}/.pirs_history", dirs::home_dir().unwrap().display());

  rl.load_history(&rl_history_file).ok();

  let mut pirs = pirs::Pirs::new(
    #[allow(clippy::redundant_closure)]
    |code| std::process::exit(code),
    pirs::LogLevel::Info,
  );

  loop {
    match rl.readline(&pirs.state.prompt) {
      Ok(input) => {
        rl.add_history_entry(&input);
        pirs.handle_command(input);
      }
      Err(ReadlineError::Interrupted) => continue,
      _ => break,
    }
  }

  rl.save_history(&rl_history_file).ok();
}
