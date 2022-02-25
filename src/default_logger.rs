#[cfg(not(target_arch = "wasm32"))]
use owo_colors::OwoColorize;

use std::format as f;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
  Debug,
  Info,
  Warn,
  Error,
}

#[derive(Debug)]
pub struct DefaultLogger {
  pub log_level: u8,
  #[cfg(not(target_arch = "wasm32"))]
  unicode_supported: bool,
  #[cfg(target_arch = "wasm32")]
  pub element: web_sys::HtmlElement,
}

impl DefaultLogger {
  pub fn new(
    log_level: LogLevel,
    #[cfg(target_arch = "wasm32")] element: web_sys::HtmlElement,
  ) -> Self {
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
      #[cfg(not(target_arch = "wasm32"))]
      unicode_supported: crate::supports_unicode::on(crate::supports_unicode::Stream::Stdout),
      #[cfg(target_arch = "wasm32")]
      element,
    }
  }

  fn log(&self, s: impl AsRef<str>) {
    cfg_iif::cfg_iif! {
        target_arch = "wasm32" {
        self.element.set_inner_html(&format!(
          "{}{}<br>",
          self.element.inner_html(),
          s.as_ref().replace("\n", "<br>")
        ))
      } else {
        println!("{}", s.as_ref());
      }
    }
  }
}

impl super::Logger for DefaultLogger {
  fn debug(&self, message: &str) {
    if self.log_level >= 4 {
      self.log(f!(
        "[{}]: {}",
        cfg_iif::cfg_iif! {
          target_arch = "wasm32" {
            "<span style=\"color:#00BFFF\">debug</span>"
          } else {
            if self.unicode_supported {
              "debug".green().to_string()
            } else {
              "debug".into()
            }
          }
        },
        message.replace("<", "&lt;")
      ));
    }
  }

  fn info(&self, message: &str) {
    if self.log_level >= 3 {
      self.log(f!(
        "[{}]: {}",
        cfg_iif::cfg_iif! {
          target_arch = "wasm32" {
            "<span style=\"color:#00FF00\">info</span>"
          } else {
            if self.unicode_supported {
              "info".green().to_string()
            } else {
              "info".into()
            }
          }
        },
        message.replace("<", "&lt;")
      ));
    }
  }

  fn warn(&self, message: &str) {
    if self.log_level >= 2 {
      self.log(f!(
        "[{}]: {}",
        cfg_iif::cfg_iif! {
          target_arch = "wasm32" {
            "<span style=\"color:#FFFF00\">warn</span>"
          } else {
            if self.unicode_supported {
              "warn".yellow().to_string()
            } else {
              "warn".into()
            }
          }
        },
        message.replace("<", "&lt;")
      ));
    }
  }

  fn error(&self, message: &str) {
    if self.log_level >= 1 {
      self.log(f!(
        "[{}]: {}",
        cfg_iif::cfg_iif! {
          target_arch = "wasm32" {
            "<span style=\"color:#FF0000\">error</span>"
          } else {
            if self.unicode_supported {
              "error".bright_red().to_string()
            } else {
              "error".into()
            }
          }
        },
        message.replace("<", "&lt;")
      ));
    }
  }

  fn raw(&self, message: &str) {
    self.log(message)
  }
}
