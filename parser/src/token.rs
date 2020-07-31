#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Loc {
    start: usize,
    end: usize,
}

impl Loc {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
    pub fn merge(&self, other: &Loc) -> Self {
        use std::cmp::{max, min};
        Loc {
            start: min(self.start, other.start),
            end: max(self.end, other.end),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Annotation<T> {
    pub value: T,
    pub loc: Loc,
}
impl<T> Annotation<T> {
    pub fn new(value: T, loc: Loc) -> Self {
        Self { value, loc }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Number(u64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    LParen,
    RParen,
}
pub type Token = Annotation<TokenKind>;

macro_rules! annotation_factory {
    ($name: ident, $token: expr) => {
        pub fn $name(loc: Loc) -> Self {
            Self::new($token, loc)
        }
    };
}
impl Token {
    pub fn number(n: u64, loc: Loc) -> Self {
        Self::new(TokenKind::Number(n), loc)
    }
    annotation_factory!(plus, TokenKind::Plus);
    annotation_factory!(minus, TokenKind::Minus);
    annotation_factory!(asterisk, TokenKind::Asterisk);
    annotation_factory!(slash, TokenKind::Slash);
    annotation_factory!(lparen, TokenKind::LParen);
    annotation_factory!(rparen, TokenKind::RParen);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LexErrorKind {
    InvalidChar(char),
    EOF,
}
pub type LexError = Annotation<LexErrorKind>;

impl LexError {
    pub fn invalid_char(c: char, loc: Loc) -> Self {
        Self::new(LexErrorKind::InvalidChar(c), loc)
    }
    annotation_factory!(eof, LexErrorKind::EOF);
}

mod lex {
    use super::{LexError, Loc, Token};

    pub type LexResult<T> = Result<(T, usize), LexError>;

    macro_rules! lexer_fn {
        ($name: ident, $expected: expr) => {
            pub fn $name(input_byte: &[u8], position: usize) -> LexResult<Token> {
                consume_byte(input_byte, position, $expected).map(|(_, end)| {
                    return (Token::$name(Loc::new(position, end)), end);
                })
            }
        };
    }

    fn consume_byte(input_bytes: &[u8], position: usize, expected: u8) -> LexResult<u8> {
        if input_bytes.len() <= position {
            Err(LexError::eof(Loc::new(position, position)))
        } else if input_bytes[position] != expected {
            Err(LexError::invalid_char(
                input_bytes[position] as char,
                Loc::new(position, position),
            ))
        } else {
            Ok((expected, position + 1))
        }
    }
    lexer_fn!(plus, b'+');
    lexer_fn!(minus, b'-');
    lexer_fn!(asterisk, b'*');
    lexer_fn!(slash, b'/');
    lexer_fn!(lparen, b'(');
    lexer_fn!(rparen, b')');

    fn consume_bytes_until(
        input_bytes: &[u8],
        mut position: usize,
        mut f: impl FnMut(u8) -> bool,
    ) -> usize {
        while position < input_bytes.len() && f(input_bytes[position]) {
            position += 1;
        }
        position
    }

    pub fn number(input_bytes: &[u8], position: usize) -> LexResult<Token> {
        let start = position;
        let end = consume_bytes_until(input_bytes, start, |byte| b"1234567890".contains(&byte));
        let n = std::str::from_utf8(&input_bytes[start..end])
            .unwrap()
            .parse()
            .unwrap();
        Ok((Token::number(n, Loc::new(start, end)), end))
    }

    pub fn ignore_spaces(input_bytes: &[u8], position: usize) -> LexResult<()> {
        let end = consume_bytes_until(input_bytes, position, |byte| b" \n\t".contains(&byte));
        Ok(((), end))
    }
}

pub fn lex(input: &str) -> Result<Vec<Token>, LexError> {
    let mut tokens: Vec<Token> = Vec::new();
    let input_bytes = input.as_bytes();
    let mut position = 0;
    macro_rules! lex_a_token {
        ($lexer:expr) => {{
            let (token, new_position) = $lexer?;
            tokens.push(token);
            position = new_position;
        }};
    }

    while position < input_bytes.len() {
        match input_bytes[position] {
            b'0'..=b'9' => lex_a_token!(lex::number(input_bytes, position)),
            b'+' => lex_a_token!(lex::plus(input_bytes, position)),
            b'-' => lex_a_token!(lex::minus(input_bytes, position)),
            b'*' => lex_a_token!(lex::asterisk(input_bytes, position)),
            b'/' => lex_a_token!(lex::slash(input_bytes, position)),
            b'(' => lex_a_token!(lex::lparen(input_bytes, position)),
            b')' => lex_a_token!(lex::rparen(input_bytes, position)),
            b' ' | b'\n' | b'\t' => {
                let ((), new_position) = lex::ignore_spaces(input_bytes, position)?;
                position = new_position;
            }
            b => {
                return Err(LexError::invalid_char(
                    b as char,
                    Loc::new(position, position + 1),
                ))
            }
        }
    }

    Ok(tokens)
}

pub fn lex_interactive() -> () {
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
            let token = lex(&line);
            println!("{:?}", token);
        } else {
            break;
        }
    }
}

#[test]
fn test_lexer() {
    assert_eq!(
        lex("1 + 2 + 3 - -10"),
        Ok(vec![
            Token::number(1, Loc::new(0, 1)),
            Token::plus(Loc::new(2, 3)),
            Token::number(2, Loc::new(4, 5)),
            Token::plus(Loc::new(6, 7)),
            Token::number(3, Loc::new(8, 9)),
            Token::minus(Loc::new(10, 11)),
            Token::minus(Loc::new(12, 13)),
            Token::number(10, Loc::new(13, 15)),
        ])
    )
}
