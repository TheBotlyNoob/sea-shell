pub trait Logger {
  fn debug(&self, message: String);

  fn info(&self, message: String);

  fn warn(&self, message: String);

  fn error(&self, message: String);

  fn raw(&self, message: String);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LogLevel {
  Debug = 4,
  Info = 3,
  Warn = 2,
  Error = 1,
  None = 0,
}

#[macro_export]
macro_rules! create_logger_from_logger {
  ( $logger:expr, $newlines:expr ) => {
    crate::macro_helpers::with_dollar_sign! {
      ( $escape:tt ) => {
        #[allow(unused_macros)]
        macro_rules! log {
          ( $level:ident, $escape($arg:tt)* ) => {
            $logger.$level(match $newlines {
              true => format!("{}\n", format!($escape($arg)*)),
              false => format!($escape($arg)*)
            });
          };
          () => {
            $logger.raw("".into());
          };
        }
      }
    }
  };
}

pub use create_logger_from_logger;

#[cfg(feature = "default-logger")]
pub mod default;
