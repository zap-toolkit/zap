mod tag;

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

  while *idx < segment.len() as i32 && (segment.chars().nth(*idx as usize).unwrap().is_alphanumeric()
    || segment.chars().nth(*idx as usize).unwrap() == '-') {
      letters.push(segment.chars().nth(*idx as usize).unwrap());
      *idx += 1;
  }

  return letters;
}

pub fn lex(src: String) -> Vec<Token> {
  let mut tkns: Vec<Token> = vec![];
  let mut idx = 0;
  let mut count = 0;

  skip_whitespace(src.clone(), &mut idx);

  while count < 7 {
    let tag = tag::lex_tag(src.clone(), &mut idx);

    if tag.len() == 0 {
      break;
    }

    for tkn in tag {
      tkns.push(tkn);
    }

    skip_whitespace(src.clone(), &mut idx);
    count += 1;
  }

  return tkns;
}
