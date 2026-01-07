use crate::token::Token;
use std::iter::Peekable;

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
struct IntParseErr {}

fn parse_int(itr: &mut Peekable<impl Iterator<Item = char>>) -> Result<i32, IntParseErr> {
    let mut result: i32 = 0;
    let negative = match itr.peek() {
        Some(&'-') => {
            itr.next(); // consume the negative sign
            true
        }
        _ => false,
    };

    let handle_neg = |n| n * (if negative { -1 } else { 1 });

    // make sure not empty / just negative sign
    if itr.peek().is_none() {
        return Err(IntParseErr {});
    }

    while let Some(&c) = itr.peek() {
        if c == '\n' || c == ' ' {
            return Ok(handle_neg(result));
        }

        let c = itr
            .next()
            .expect("Safe to unwrap, becuase we checked the peek value");

        if !c.is_ascii_digit() {
            return Err(IntParseErr {});
        }

        let digit = c.to_digit(10).expect("Returned if not digit") as i32;
        result = result * 10 + digit;
    }

    Ok(handle_neg(result))
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
            '.' => {
                // TODO: guranteed to be a float
                todo!();
            }
            '"' => tokens.push(Token::String(
                parse_string(&mut input_itr).map_err(|_| Error::UnableToParseString)?,
            )),
            c if c.is_ascii_digit() || c == '-' => {
                // TODO: check for floats (we know . is not the first char)
                // float should only have one '.' with optional number on both sides (check for - in front)
                // if no numbers then it would fail
                tokens.push(Token::Int(
                    parse_int(&mut input_itr).map_err(|_| Error::UnableToParseInt)?,
                ))
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

    #[test]
    fn test_parse_int() {
        let cases = vec![
            ("123", Ok(123), "simple positive"),
            ("0", Ok(0), "zero"),
            ("-42", Ok(-42), "negative number"),
            ("9999 ", Ok(9999), "number followed by space"),
            ("-7\n", Ok(-7), "negative number followed by newline"),
            (
                "42abc",
                Err(IntParseErr {}),
                "invalid characters after number",
            ),
            ("-", Err(IntParseErr {}), "negative sign only"),
            ("", Err(IntParseErr {}), "empty string"),
        ];

        for (input, expected, desc) in cases {
            let mut iter: Peekable<_> = input.chars().peekable();
            let result = parse_int(&mut iter);

            match expected {
                Ok(n) => assert_eq!(result.unwrap(), n, "Failed case: {}", desc),
                Err(_) => assert!(result.is_err(), "Failed case: {}", desc),
            }
        }
    }

    #[test]
    fn test_parse_ident() {
        todo!();
    }

    #[test]
    fn test_lex() {
        todo!();
    }
}
