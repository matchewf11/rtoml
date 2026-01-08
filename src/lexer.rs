use crate::token::Token;
use std::iter::Peekable;

enum ParseNumValue {
    Integer(i64),
    Float(f64),
}
struct NumParseErr {}

// test this
fn parse_num(itr: &mut impl Iterator<Item = char>) -> Result<ParseNumValue, NumParseErr> {
    todo!();
}


fn skip_to_newline(itr: &mut Peekable<impl Iterator<Item = char>>) {
    loop {
        if itr.peek().is_none() || itr.peek().unwrap() == &'\n' {
            return;
        }
        itr.next();
    }
}

#[derive(Debug)]
struct StringParseErr {}

fn parse_string(itr: &mut impl Iterator<Item = char>) -> Result<String, StringParseErr> {
    let first = itr.next().ok_or(StringParseErr {})?;
    if first != '"' {
        return Err(StringParseErr {});
    }

    let mut result = String::new();
    while let Some(c) = itr.next() {
        if c == '"' {
            return Ok(result);
        }
        result.push(c);
    }

    Err(StringParseErr {})
}

#[derive(Debug)]
struct IdentParseErr {}

fn parse_ident(itr: &mut Peekable<impl Iterator<Item = char>>) -> Result<String, IdentParseErr> {
    let mut result = String::new();

    while let Some(&c) = itr.peek() {
        if c == '\n' || c == ' ' {
            return Ok(result);
        }

        let c = itr
            .next()
            .expect("Safe to unwrap, becuase we checked the peek value");

        if !crate::token::is_identifier(c) {
            return Err(IdentParseErr {});
        }

        result.push(c);
    }

    Ok(result)
}

#[derive(Debug)]
pub enum Error {
    UnknownChar,
    UnableToParseString,
    UnableToParseInt,
    UnableToParseIdent,
}

pub fn lex(input: &str) -> Result<Vec<Token>, Error> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut input_itr = input.chars().peekable();

    loop {
        let c = match input_itr.peek() {
            None => {
                break;
            }
            Some(&c) => c,
        };

        match c {
            '#' => skip_to_newline(&mut input_itr),
            ' ' => {
                input_itr.next(); // skip spaces
            }
            '\n' => {
                tokens.push(Token::NewLine);
                input_itr.next();
            }
            '=' => {
                tokens.push(Token::Equal);
                input_itr.next();
            }
            '"' => tokens.push(Token::String(
                parse_string(&mut input_itr).map_err(|_| Error::UnableToParseString)?,
            )),
            c if c.is_ascii_digit() || c == '-' || c == '.' => {
                // TODO: check for floats (we know . could be the first char)
                // float should only have one '.' with optional number on both sides (check for - in front)
                // if no numbers then it would fail
                // tokens.push(Token::Int(
                //     parse_int(&mut input_itr).map_err(|_| Error::UnableToParseInt)?,
                // ))
            }
            c if crate::token::is_identifier(c) => {
                let rs = parse_ident(&mut input_itr).map_err(|_| Error::UnableToParseIdent)?;
                if rs == "true" {
                    tokens.push(Token::True);
                } else if rs == "false" {
                    tokens.push(Token::False);
                } else {
                    tokens.push(Token::Ident(rs));
                }
            }
            _ => {
                return Err(Error::UnknownChar);
            }
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skip_to_newline() {
        let cases = vec![
            ("hello world\nnext line", Some('\n'), "newline in middle"),
            ("", None, "empty string"),
            ("no newline here", None, "no newline"),
            ("\nstarts with newline", Some('\n'), "starts with newline"),
        ];

        for (input, expected, desc) in cases {
            let mut iter: Peekable<_> = input.chars().peekable();
            skip_to_newline(&mut iter);
            let peeked = iter.peek().copied();
            assert_eq!(peeked, expected, "Failed case: {}", desc);
        }
    }

    #[test]
    fn test_parse_string() {
        let cases = vec![
            ("\"hello\"", Ok("hello".to_string()), "simple string"),
            (
                "\"hello world!\"",
                Ok("hello world!".to_string()),
                "string with spaces",
            ),
            ("\"\"", Ok("".to_string()), "empty string"),
            (
                "\"unclosed",
                Err(StringParseErr {}),
                "missing closing quote",
            ),
            ("noquote", Err(StringParseErr {}), "missing opening quote"),
            ("", Err(StringParseErr {}), "empty iterator"),
        ];

        for (input, expected, desc) in cases {
            let mut iter = input.chars();
            let result = parse_string(&mut iter);

            match expected {
                Ok(ref s) => assert_eq!(result.unwrap(), *s, "Failed case: {}", desc),
                Err(_) => assert!(result.is_err(), "Failed case: {}", desc),
            }
        }
    }

    fn make_iter(s: &str) -> Peekable<std::str::Chars<'_>> {
        s.chars().peekable()
    }

    #[test]
    fn test_parse_ident() {
        let mut itr = make_iter("hello");
        let ident = parse_ident(&mut itr).unwrap();
        assert_eq!(ident, "hello");

        let mut itr = make_iter("hello world");
        let ident = parse_ident(&mut itr).unwrap();
        assert_eq!(ident, "hello");
        assert_eq!(itr.next(), Some(' '));

        let mut itr = make_iter("hello\nworld");
        let ident = parse_ident(&mut itr).unwrap();
        assert_eq!(ident, "hello");
        assert_eq!(itr.next(), Some('\n'));

        let mut itr = make_iter("_foo_bar");
        let ident = parse_ident(&mut itr).unwrap();
        assert_eq!(ident, "_foo_bar");

        let mut itr = make_iter("hello!");
        assert!(parse_ident(&mut itr).is_err());

        let mut itr = make_iter("");
        let ident = parse_ident(&mut itr).unwrap();
        assert_eq!(ident, "");

        let mut itr = make_iter(" foo");
        let ident = parse_ident(&mut itr).unwrap();
        assert_eq!(ident, "");
        assert_eq!(itr.next(), Some(' '));
    }

    #[test]
    fn test_lex() {
        let input = r#"
        # Comment line
        key = "value"
        number = 42
        negative = -7
        flag_true = true
        flag_false = false
        "#;

        let tokens = lex(input).expect("Lexing should succeed");

        let expected = vec![
            Token::NewLine,
            Token::NewLine,
            Token::Ident("key".to_string()),
            Token::Equal,
            Token::String("value".to_string()),
            Token::NewLine,
            Token::Ident("number".to_string()),
            Token::Equal,
            Token::Int(42),
            Token::NewLine,
            Token::Ident("negative".to_string()),
            Token::Equal,
            Token::Int(-7),
            Token::NewLine,
            Token::Ident("flag_true".to_string()),
            Token::Equal,
            Token::True,
            Token::NewLine,
            Token::Ident("flag_false".to_string()),
            Token::Equal,
            Token::False,
            Token::NewLine,
        ];

        assert_eq!(tokens, expected);

        let err_input = "key @ value";
        let err = lex(err_input).unwrap_err();
        assert!(matches!(err, Error::UnknownChar));

        let empty_tokens = lex("").expect("Lexing empty string should succeed");
        assert!(empty_tokens.is_empty());
    }
}
