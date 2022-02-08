pub fn tokenize_command(input: impl AsRef<str>) -> Vec<String> {
  let commands = input.as_ref().split(';').map(String::from).collect();

  commands
}
