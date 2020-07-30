mod token;
fn main() {
  let t = token::Token::asterisk(token::Loc::new(1, 10));
  println!("Hello, world! {:?}", t);
}
