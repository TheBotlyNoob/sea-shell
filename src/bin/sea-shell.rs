use rustyline::{error::ReadlineError, Editor};
use std::cell::RefCell;

#[tokio::main]
async fn main() {
  let rl = RefCell::new(Editor::<()>::new());

  let rl_history_file = format!("{}/.pirs_history", dirs::home_dir().unwrap().display());

  rl.borrow_mut().load_history(&*rl_history_file).ok();

  let mut pirs = sea_shell::SeaShell::new(
    |code, _ctx| {
      rl.borrow_mut().save_history(&rl_history_file).unwrap();

      std::process::exit(code);
    },
    sea_shell::default_logger::DefaultLogger::new(sea_shell::default_logger::LogLevel::Info),
  );

  let mut _rl = Editor::<()>::new();
  loop {
    match _rl.readline(&pirs.state.prompt) {
      Ok(input) => {
        rl.borrow_mut().add_history_entry(&input);
        pirs.handle_command(&input).await;
      }
      Err(ReadlineError::Interrupted) => pirs.logger.info("use Ctrl-D or type exit to exit"),
      _ => break,
    }
  }
}
