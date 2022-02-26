#[cfg(not(target_arch = "wasm32"))]
use owo_colors::OwoColorize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LogLevel {
  Debug = 4,
  Info = 3,
  Warn = 2,
  Error = 1,
}

#[derive(Debug, Clone)]
#[cfg(not(target_arch = "wasm32"))]
pub struct DefaultLogger {
  pub log_level: u8,
  unicode_supported: bool,
}

#[derive(Debug, Clone)]
#[cfg(target_arch = "wasm32")]
pub struct DefaultLogger<T: Clone + Into<web_sys::Element>> {
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
          _ => log_level as u8,
        }
      } else {
        log_level as u8
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
        message
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
        message
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
        message
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
        message
      );
    }
  }

  fn raw(&self, message: &str) {
    println!("{}", message)
  }
}

// do the same as above, but using a DOM element instead of stdout
#[cfg(target_arch = "wasm32")]
impl<T: Clone + Into<web_sys::Element>> DefaultLogger<T> {
  pub fn new(log_level: LogLevel, element: T) -> Self {
    Self {
      log_level: log_level as u8,
      element,
    }
  }

  fn log(&self, s: String) {
    let element = self.element.clone().into();

    element.set_inner_html(&format!(
      "{}{}<br>",
      element.inner_html(),
      s.replace('\n', "<br>")
    ));
  }
}

#[cfg(target_arch = "wasm32")]
impl<T: Clone + Into<web_sys::Element>> crate::Logger for DefaultLogger<T> {
  fn debug(&self, message: &str) {
    if self.log_level >= 4 {
      self.log(format!(
        "[<span style=\"color:#00BFFF\">debug</span>]: {}",
        message.replace('<', "&lt;")
      ));
    }
  }

  fn info(&self, message: &str) {
    if self.log_level >= 3 {
      self.log(format!(
        "[<span style=\"color:#00FF00\">info</span>]: {}",
        message.replace('<', "&lt;")
      ));
    }
  }

  fn warn(&self, message: &str) {
    if self.log_level >= 2 {
      self.log(format!(
        "[<span style=\"color:#FFFF00\">warn</span>]: {}",
        message.replace('<', "&lt;")
      ));
    }
  }

  fn error(&self, message: &str) {
    if self.log_level >= 1 {
      self.log(format!(
        "[<span style=\"color:#FF0000\">error</span>]: {}",
        message.replace('<', "&lt;")
      ));
    }
  }

  fn raw(&self, message: &str) {
    self.log(message.replace('<', "&lt;"));
  }
}
