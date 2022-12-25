pub mod tag;

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token {
  pub name: String,
  pub value: String
}

// Skips whitespace
pub(self) fn skip_whitespace(segment: String, idx: &mut i32) {
  while *idx < segment.len() as i32 &&  segment.chars().nth(*idx as usize).unwrap().is_whitespace() {
    *idx += 1;
  }
}

// Tokenizes a group of letters
pub(self) fn tokenize_letters(segment: String, idx: &mut i32) -> String {
  let mut letters = "".to_string();

  while *idx < segment.len() as i32 && segment.chars().nth(*idx as usize).unwrap().is_alphabetic() {
    letters.push(segment.chars().nth(*idx as usize).unwrap());
    *idx += 1;
  }

  return letters;
}
