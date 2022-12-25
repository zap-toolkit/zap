mod lexer;

// Generates a parse tree from the provided HTML source.
pub fn gen(filename: &str) {
  let src = std::fs::read_to_string(filename)
    .expect(format!("Error: HTML source file \"{}\" does not exist.", filename).as_str());

  println!("{:?}", lexer::tag::lex_tag(src, 0));
}
