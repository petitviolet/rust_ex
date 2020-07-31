mod ast;
mod token;
fn main() {
  parse_interactive();
}

pub fn parse_interactive() -> () {
    use std::io::{stdin, stdout, BufRead, BufReader, Write};
    let prompt = |s: &str| {
        let stdout = stdout();
        let mut stdout = stdout.lock();
        stdout.write(s.as_bytes())?;
        stdout.flush()
    };
    let stdin = stdin();
    // let mut stdin = stdin.lock();
    let stdin = BufReader::new(stdin.lock());
    let mut lines = stdin.lines();
    loop {
        prompt("> ").unwrap();
        if let Some(Ok(line)) = lines.next() {
          match token::lex(&line) {
            Ok(tokens) => {
              println!("{:?}", tokens);
              match ast::Tokens::parse(tokens) {
                Ok(ast) => {
                  println!("{:?}", ast);
                },
                Err(err) => {
                  println!("ParseError: {:?}", err);
                }
              }
            },
            Err(err) => {
              println!("LexError: {:?}", err);
            }
          }
        } else {
            break;
        }
    }
}