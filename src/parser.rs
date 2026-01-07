use std::collections::HashMap;

use crate::token::Token;

#[derive(Debug)]
pub enum Value {
    String(String),
    Bool(bool),
    Int(u32),
}

#[derive(Debug)]
pub struct Error {}

pub fn parse_tokens(list: &[Token]) -> Result<HashMap<String, Value>, Error> {
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
                    _ => return Err(Error {}),
                }

                match list_iter.next() {
                    Some(Token::Int(n)) => {
                        map.insert(id_string.clone(), Value::Int(*n));
                    }
                    Some(Token::String(s)) => {
                        map.insert(id_string.clone(), Value::String(s.clone()));
                    }
                    Some(Token::True) => {
                        map.insert(id_string.clone(), Value::Bool(true));
                    }
                    Some(Token::False) => {
                        map.insert(id_string.clone(), Value::Bool(false));
                    }
                    _ => return Err(Error {}),
                }

                match list_iter.next() {
                    Some(Token::NewLine) | None => (),
                    _ => return Err(Error {}),
                }
            }
            _ => return Err(Error {}),
        }
    }

    Ok(map)
}
