use std::{
  error::Error,
  io::{stdin, stdout, Write},
};

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
  tracing_subscriber::fmt().pretty().without_time().init();

  loop {
    pirs::handle_command({
      print!("{}", pirs::state::prompt());

      stdout().flush()?;
      let mut command = String::new();
      stdin().read_line(&mut command)?;
      command
    });
  }
}
