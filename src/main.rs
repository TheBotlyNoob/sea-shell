fn main() {
  // let clap_app = clap::app_from_crate!().get_matches();

  let mut rl = rustyline::Editor::<()>::new();

  let rl_history_file = format!("{}/.pirs_history", dirs::home_dir().unwrap().display());

  rl.load_history(&rl_history_file).ok();

  let mut pirs = pirs::Pirs::new(
    #[allow(clippy::redundant_closure)]
    |code| std::process::exit(code),
    pirs::LogLevel::Info,
  );

  while let Ok(input) = rl.readline(&pirs.state.prompt) {
    rl.add_history_entry(&input);
    pirs.handle_command(input);
  }

  rl.save_history(&rl_history_file).ok();
}
