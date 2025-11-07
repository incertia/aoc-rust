pub const TEST_STRING: &[u8] = b"012345678901234567890123456789012345678901234567890123456789";

impl super::AocParser for Vec<u8> {
  fn parse(input: &[u8]) -> Self {
    input.to_owned()
  }
}
