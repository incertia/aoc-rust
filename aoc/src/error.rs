#[derive(Debug)]
pub enum AdventRuntimeError {
  Io(std::io::Error),
  Reqwest(reqwest::Error),
}

impl From<std::io::Error> for AdventRuntimeError {
  fn from(value: std::io::Error) -> Self {
    Self::Io(value)
  }
}

impl From<reqwest::Error> for AdventRuntimeError {
  fn from(value: reqwest::Error) -> Self {
    Self::Reqwest(value)
  }
}
