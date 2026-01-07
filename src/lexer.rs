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

enum StringParseErr {
    NoMoreChars,
    NoStartQuotation,
    NoEndQuotation,
}

fn parse_string(itr: &mut impl Iterator<Item = char>) -> Result<String, StringParseErr> {
    let first = itr.next().ok_or(StringParseErr::NoMoreChars)?;
    if first != '"' {
        return Err(StringParseErr::NoStartQuotation);
    }

    let mut result = String::new();
    while let Some(c) = itr.next() {
        if c == '"' {
            return Ok(result);
        }
        result.push(c);
    }

    Err(StringParseErr::NoEndQuotation)
}

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
