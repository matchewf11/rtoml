
use std::{collections::HashMap, iter::Peekable};

#[derive(Debug)]
pub enum Token {
    String(String),
    Int(u32),
    Equal,
    Ident(String),
    NewLine,
    True,
    False,
}

fn is_identifier(c: char) -> bool {
    c.is_ascii_uppercase() || c.is_ascii_lowercase() || c == '_'
}

struct IdentParseErr {}
fn parse_ident_token(
    itr: &mut Peekable<impl Iterator<Item = char>>,
) -> Result<String, IdentParseErr> {
    let mut result = String::new();

    while let Some(&c) = itr.peek() {
        if c == '\n' || c == ' ' {
            return Ok(result);
        }

        let c = itr
            .next()
            .expect("Safe to unwrap, becuase we checked the peek value");

        if !is_identifier(c) {
            return Err(IdentParseErr {});
        }

        result.push(c);
    }

    Ok(result)
}

fn consume_comment_until_newline(itr: &mut Peekable<impl Iterator<Item = char>>) {
    loop {
        if itr.peek().is_none() || itr.peek().unwrap() == &'\n' {
            return;
        }
        itr.next();
    }
}

struct IntParseErr {}
fn parse_int_token(itr: &mut Peekable<impl Iterator<Item = char>>) -> Result<u32, IntParseErr> {
    let mut result: u32 = 0;

    while let Some(&c) = itr.peek() {
        if c == '\n' || c == ' ' {
            return Ok(result);
        }

        let c = itr
            .next()
            .expect("Safe to unwrap, becuase we checked the peek value");

        if !c.is_ascii_digit() {
            return Err(IntParseErr {});
        }

        result *= 10;
        result += c.to_digit(10).expect("Returned if not digit");
    }

    Ok(result)
}

enum StringParseErr {
    NoMoreChars,
    NoStartQuotation,
    NoEndQuotation,
}

fn parse_string_token(itr: &mut impl Iterator<Item = char>) -> Result<String, StringParseErr> {
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

pub enum ParseErr {
    UnknownChar(char),
    UnableToParseString,
    UnableToParseInt,
    UnableToParseIdent,
}

pub fn parse(input: &str) -> Result<Vec<Token>, ParseErr> {
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
            '#' => consume_comment_until_newline(&mut input_itr),
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
                parse_string_token(&mut input_itr).map_err(|_| ParseErr::UnableToParseString)?,
            )),
            c if c.is_ascii_digit() => tokens.push(Token::Int(
                parse_int_token(&mut input_itr).map_err(|_| ParseErr::UnableToParseInt)?,
            )),
            c if is_identifier(c) => {
                let rs =
                    parse_ident_token(&mut input_itr).map_err(|_| ParseErr::UnableToParseIdent)?;
                if rs == "true" {
                    tokens.push(Token::True);
                } else if rs == "false" {
                    tokens.push(Token::False);
                } else {
                    tokens.push(Token::Ident(rs));
                }
            }
            c => {
                return Err(ParseErr::UnknownChar(c));
            }
        }
    }

    Ok(tokens)
}

#[derive(Debug)]
pub enum TomlValues {
    String(String),
    Bool(bool),
    Int(u32),
}

#[derive(Debug)]
pub struct TokenMapErr {}

pub fn token_list_to_map(list: &[Token]) -> Result<HashMap<String, TomlValues>, TokenMapErr> {
    let mut map = HashMap::new();

    let mut list_iter = list.iter();

    while let Some(next_token) = list_iter.next() {
        match next_token {
            Token::NewLine => {
                continue;
            }
            Token::Ident(id_string) => {
                match list_iter.next() {
                    Some(Token::Equal) => (),
                    _ => return Err(TokenMapErr {}),
                }

                match list_iter.next() {
                    Some(Token::Int(n)) => {
                        map.insert(id_string.clone(), TomlValues::Int(*n));
                    }
                    Some(Token::String(s)) => {
                        map.insert(id_string.clone(), TomlValues::String(s.clone()));
                    }
                    Some(Token::True) => {
                        map.insert(id_string.clone(), TomlValues::Bool(true));
                    }
                    Some(Token::False) => {
                        map.insert(id_string.clone(), TomlValues::Bool(false));
                    }
                    _ => return Err(TokenMapErr {}),
                }

                match list_iter.next() {
                    Some(Token::NewLine) | None => (),
                    _ => return Err(TokenMapErr {}),
                }
            }
            _ => return Err(TokenMapErr {}),
        }
    }

    Ok(map)
}

