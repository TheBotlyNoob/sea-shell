#[cfg(not(target_arch = "wasm32"))]
use owo_colors::OwoColorize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
  Debug,
  Info,
  Warn,
  Error,
}

#[cfg(not(target_arch = "wasm32"))]
pub struct DefaultLogger {
  pub log_level: u8,
  unicode_supported: bool,
}

#[cfg(target_arch = "wasm32")]
pub struct DefaultLogger<T: Into<web_sys::Element>> {
  pub log_level: u8,
  pub element: T,
}

#[cfg(not(target_arch = "wasm32"))]
impl DefaultLogger {
  pub fn new(log_level: LogLevel) -> Self {
    Self {
      log_level: if let Ok(level) = std::env::var("LOG_LEVEL") {
        match &*level.to_lowercase() {
          "debug" => 4,
          "info" => 3,
          "warn" => 2,
          "error" => 1,
          _ => match log_level {
            LogLevel::Debug => 4,
            LogLevel::Info => 3,
            LogLevel::Warn => 2,
            LogLevel::Error => 1,
          },
        }
      } else {
        match log_level {
          LogLevel::Debug => 4,
          LogLevel::Info => 3,
          LogLevel::Warn => 2,
          LogLevel::Error => 1,
        }
      },
      unicode_supported: crate::supports_unicode::on(crate::supports_unicode::Stream::Stdout),
    }
  }
}

#[cfg(not(target_arch = "wasm32"))]
impl super::Logger for DefaultLogger {
  fn debug(&self, message: &str) {
    if self.log_level >= 4 {
      println!(
        "[{}]: {}",
        if self.unicode_supported {
          "debug".green().to_string()
        } else {
          "debug".into()
        },
        message.replace("<", "&lt;")
      );
    }
  }

  fn info(&self, message: &str) {
    if self.log_level >= 3 {
      println!(
        "[{}]: {}",
        if self.unicode_supported {
          "info".green().to_string()
        } else {
          "info".into()
        },
        message.replace("<", "&lt;")
      );
    }
  }

  fn warn(&self, message: &str) {
    if self.log_level >= 2 {
      println!(
        "[{}]: {}",
        if self.unicode_supported {
          "warn".yellow().to_string()
        } else {
          "warn".into()
        },
        message.replace("<", "&lt;")
      );
    }
  }

  fn error(&self, message: &str) {
    if self.log_level >= 1 {
      println!(
        "[{}]: {}",
        if self.unicode_supported {
          "error".bright_red().to_string()
        } else {
          "error".into()
        },
        message.replace("<", "&lt;")
      );
    }
  }

  fn raw(&self, message: &str) {
    println!("{}", message)
  }
}

// do the same as above, but using a DOM element instead of stdout
#[cfg(target_arch = "wasm32")]
impl<T: Into<web_sys::Element>> DefaultLogger<T> {
  fn new(log_level: LogLevel, element: T) -> Self {
    Self {
      log_level: if let Ok(level) = std::env::var("LOG_LEVEL") {
        match &*level.to_lowercase() {
          "debug" => 4,
          "info" => 3,
          "warn" => 2,
          "error" => 1,
          _ => match log_level {
            LogLevel::Debug => 4,
            LogLevel::Info => 3,
            LogLevel::Warn => 2,
            LogLevel::Error => 1,
          },
        }
      } else {
        match log_level {
          LogLevel::Debug => 4,
          LogLevel::Info => 3,
          LogLevel::Warn => 2,
          LogLevel::Error => 1,
        }
      },
      element,
    }
  }
}

#[cfg(target_arch = "wasm32")]
impl<T: Into<web_sys::Element>> crate::Logger for DefaultLogger<T> {
  fn debug(&self, message: &str) {
    if self.log_level >= 4 {
      println!(
        "[{}]: {}",
        if self.unicode_supported {
          "debug".green().to_string()
        } else {
          "debug".into()
        },
        message.replace("<", "&lt;")
      );
    }
  }

  fn info(&self, message: &str) {
    if self.log_level >= 3 {
      println!(
        "[{}]: {}",
        if self.unicode_supported {
          "info".green().to_string()
        } else {
          "info".into()
        },
        message.replace("<", "&lt;")
      );
    }
  }

  fn warn(&self, message: &str) {
    if self.log_level >= 2 {
      println!(
        "[{}]: {}",
        if self.unicode_supported {
          "warn".yellow().to_string()
        } else {
          "warn".into()
        },
        message.replace("<", "&lt;")
      );
    }
  }

  fn error(&self, message: &str) {
    if self.log_level >= 1 {
      println!(
        "[{}]: {}",
        if self.unicode_supported {
          "error".bright_red().to_string()
        } else {
          "error".into()
        },
        message.replace("<", "&lt;")
      );
    }
  }

  fn raw(&self, message: &str) {
    println!("{}", message)
  }
}
