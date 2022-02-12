use owo_colors::OwoColorize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
  Debug,
  Info,
  Warn,
  Error,
}

#[derive(Debug)]
pub struct DefaultLogger(pub LogLevel, pub bool);

impl DefaultLogger {
  pub fn log_level(&self) -> u8 {
    if let Ok(level) = std::env::var("LOG_LEVEL") {
      match &*level.to_lowercase() {
        "debug" => 4,
        "info" => 3,
        "warn" => 2,
        _ => 1,
      }
    } else {
      match self.0 {
        LogLevel::Debug => 4,
        LogLevel::Info => 3,
        LogLevel::Warn => 2,
        LogLevel::Error => 1,
      }
    }
  }
}

impl super::Logger for DefaultLogger {
  fn debug(&self, message: &str) {
    if self.log_level() >= 4 {
      println!(
        "[{}]: {}",
        if self.1 {
          "debug".bright_blue().to_string()
        } else {
          "debug".into()
        },
        message
      );
    }
  }

  fn info(&self, message: &str) {
    if self.log_level() >= 3 {
      println!(
        "[{}]: {}",
        if self.1 {
          "info".green().to_string()
        } else {
          "info".into()
        },
        message
      );
    }
  }

  fn warn(&self, message: &str) {
    if self.log_level() >= 2 {
      println!(
        "[{}]: {}",
        if self.1 {
          "warn".yellow().to_string()
        } else {
          "warn".into()
        },
        message
      );
    }
  }

  fn error(&self, message: &str) {
    if self.log_level() >= 1 {
      println!(
        "[{}]: {}",
        if self.1 {
          "error".bright_red().to_string()
        } else {
          "error".into()
        },
        message
      );
    }
  }

  fn raw(&self, message: &str) {
    println!("{}", message);
  }
}
