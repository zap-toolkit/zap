use super::Token;

pub fn lex_tag(segment: String, idx: &mut i32) -> Vec<Token> {
  // Get the name of the tag
  *idx += 1;
  super::skip_whitespace(segment.clone(), &mut *idx);

  let mut tokens = vec![];

  // If the tag is a closing tag
  if segment.chars().nth(*idx as usize).unwrap() == '/' {
    *idx += 1;

    let tkn = Token { name: "$T_CLOSING_TAG".to_string(), value: segment.clone() };
    tokens.push(tkn);

    super::skip_whitespace(segment.clone(), &mut *idx);
    return tokens;
  }

  while segment.chars().nth(*idx as usize).unwrap() == '<' {
    *idx += 1;
  }

  let name = super::tokenize_letters(segment.clone(), &mut *idx);
  super::skip_whitespace(segment.clone(), &mut *idx);

  let tkn = Token { name: "$T_OPENING_TAG".to_string(), value: name };
  tokens.push(tkn);

  // TODO: tag content
  // TODO: proper closing tag logic

  // If the tag has attributes
  if segment.chars().nth(*idx as usize).unwrap() != '>' {
    while segment.chars().nth(*idx as usize).unwrap() != '>' {
      let attrib_name = super::tokenize_letters(segment.clone(), &mut *idx);
      super::skip_whitespace(segment.clone(), &mut *idx);

      // If there is not an equal sign after the attribute name
      if segment.chars().nth(*idx as usize).unwrap() != '=' {
        // TODO: more descriptive error messages
        eprintln!("Error: Missing equal sign.")
      }

      super::skip_whitespace(segment.clone(), &mut *idx);

      // If there is not a quotation mark after the equal sign
      *idx += 1;
      if segment.chars().nth(*idx as usize).unwrap() != '"' {
        println!("Error: Missing opening quotation marks after the equal sign.")
      }

      *idx += 1;

      super::skip_whitespace(segment.clone(), &mut *idx);
      let mut attrib_value = "".to_string();

      // Get every character until the end of the "string"
      while *idx < segment.len() as i32 && segment.chars().nth(*idx as usize).unwrap() != '"' {
        attrib_value.push(segment.chars().nth(*idx as usize).unwrap());
        *idx += 1;
      }

      super::skip_whitespace(segment.clone(), &mut *idx);

      // If there is not a quotation mark after the attribute value
      if segment.chars().nth(*idx as usize).unwrap() != '"' {
        println!("Error: Missing closing quotation marks after the attribute value.")
      }

      super::skip_whitespace(segment.clone(), &mut *idx);

      // Create tokens
      let tkn_attrib_name = Token { name: "$T_TAG_ATTRIB_NAME".to_string(), value: attrib_name.clone() };
      let tkn_attrib_value = Token { name: "$T_TAG_ATTRIB_VALUE".to_string(), value: attrib_value };

      tokens.push(tkn_attrib_name);
      tokens.push(tkn_attrib_value);

      *idx += 1;
      super::skip_whitespace(segment.clone(), &mut *idx);
    }
  } else {
    // If there is token content
    super::skip_whitespace(segment.clone(), &mut *idx);
    *idx += 1;
    super::skip_whitespace(segment.clone(), &mut *idx);

    // Get the content
    let mut content = "".to_string();

    while *idx < segment.len() as i32 && segment.chars().nth(*idx as usize).unwrap() != '<' {
      content.push(segment.chars().nth(*idx as usize).unwrap());
      *idx += 1;
    }

    super::skip_whitespace(segment.clone(), &mut *idx);
    tokens.push(Token { name: "$T_TAG_CONTENT".to_string(), value: content });

    // If there is a closing tag
    if segment.chars().nth(*idx as usize).unwrap() == '<' {
      *idx += 1;
      super::skip_whitespace(segment.clone(), &mut *idx);

      if segment.chars().nth(*idx as usize).unwrap() != '/' {
        println!("Error: Missing '/' character after tag content.");
      }

      *idx += 1;
      let closetag = super::tokenize_letters(segment.clone(), &mut *idx);

      // Append the tokens
      let closing = Token { name: "$T_CLOSING_TAG".to_string(), value: closetag };
      tokens.push(closing);

      *idx += 1;
      super::skip_whitespace(segment.clone(), &mut *idx);
    }
  }

  return tokens;
}
