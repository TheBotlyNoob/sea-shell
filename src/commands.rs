use once_cell::sync::Lazy;

pub(crate) static COMMANDS: Lazy<&[Box<dyn crate::CommandHandler>]> = Lazy::new(|| &[]);
