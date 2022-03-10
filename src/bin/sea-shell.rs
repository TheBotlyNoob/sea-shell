use reedline::{DefaultPrompt, Reedline, Signal};
use std::process::exit;

#[tokio::main]
async fn main() {
  let mut line_editor = Reedline::create().unwrap();
  let prompt = DefaultPrompt::default();

  let mut shell = sea_shell::SeaShell::new(
    |code, _shell| exit(code),
    sea_shell::logger::default::DefaultLogger::new(sea_shell::logger::LogLevel::Info, true),
    true,
  );

  loop {
    let sig = line_editor.read_line(&prompt).unwrap();

    match sig {
      Signal::Success(input) => {
        shell.handle_command(&input).await;
      }
      Signal::CtrlD | Signal::CtrlC => {
        line_editor.print_crlf().unwrap();
        break;
      }
      Signal::CtrlL => {
        line_editor.clear_screen().unwrap();
      }
    }
  }
}
