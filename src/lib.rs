use std::collections::HashMap;

mod lexer;
mod parser;
mod token;

pub use parser::Value;

#[derive(Debug)]
pub struct Error {}

pub fn parse(input: &str) -> Result<HashMap<String, parser::Value>, Error> {
    let tokens = lexer::lex(input).map_err(|_| Error {})?;
    Ok(parser::parse_tokens(&tokens).map_err(|_| Error {})?)
}
