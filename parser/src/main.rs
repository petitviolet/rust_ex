mod ast;
mod errors;
mod token;
fn main() {
    parse_interactive();
}

impl std::str::FromStr for ast::Ast {
    type Err = errors::CompileError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = token::lex(s)?;
        let ast = ast::parse(tokens)?;
        Ok(ast)
    }
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
            match line.parse::<ast::Ast>() {
                Ok(ast) => println!("{:?}", ast),
                Err(errors::CompileError::Lexer(lex_error)) => {
                    println!("Lex error({:?})", lex_error);
                    println!("{}", line);
                    let token::Loc { start, end } = lex_error.loc;
                    println!(
                        "{}{}{:?}",
                        String::from(" ").repeat(start),
                        String::from("^").repeat(end - start),
                        lex_error.value
                    );
                }
                Err(errors::CompileError::Parser(parse_error)) => {
                    let start;
                    let end;
                    match parse_error.token() {
                        Some(token) => {
                            start = token.loc.start;
                            end = token.loc.end;
                        }
                        None => {
                            start = line.len() - 1;
                            end = line.len();
                        }
                    }
                    println!("Parse error({:?})", parse_error);
                    println!("{}", line);
                    println!(
                        "{}{} {:?}",
                        String::from(" ").repeat(start),
                        String::from("^").repeat(end - start),
                        parse_error
                    );
                }
            }
        } else {
            break;
        }
    }
}
