use rustyline::{error::ReadlineError, Editor};

#[tokio::main]
async fn main() {
  let rl = std::cell::RefCell::new(Editor::<()>::new());

  let rl_history_file = format!("{}/.pirs_history", dirs::home_dir().unwrap().display());

  rl.borrow_mut().load_history(&*rl_history_file).ok();

  let mut pirs = pirs::Pirs::new(
    |code, _ctx| {
      rl.borrow_mut().save_history(&rl_history_file).unwrap();

      std::process::exit(code);
    },
    pirs::default_logger::DefaultLogger::new(pirs::default_logger::LogLevel::Info),
  );

  let mut _rl = Editor::<()>::new();
  loop {
    match _rl.readline(&pirs.state.prompt) {
      Ok(input) => {
        rl.borrow_mut().add_history_entry(&input);
        // cannot be `None`
        pirs = pirs.handle_command(&input).await.unwrap();
      }
      Err(ReadlineError::Interrupted) => pirs.logger.info("use Ctrl-D or type exit to exit"),
      _ => break,
    }
  }
}
