use std::collections::HashMap;

mod lexer;
mod parser;
mod token;

pub use parser::Value;

#[derive(Debug)]
pub struct Error {}

pub fn parse(input: &str) -> Result<HashMap<String, parser::Value>, Error> {
    let tokens = lexer::lex(input).map_err(|_| Error {})?;
    parser::parse_tokens(&tokens).map_err(|_| Error {})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let test = r#"
        me = "hello"
        # foo
        too = 1 # foo
        neg_two = -2 # foo
        _gaa = true
        shee = false
        float_test0 = 11.
        float_test1 = 1.1
        float_test2 = .11
        "#;

        let map = parse(test).expect("unable to parse in lib.rs tests");

        assert_eq!(map.len(), 8);

        match map.get("me") {
            Some(Value::String(s)) => assert_eq!(s, "hello"),
            _ => panic!("Key 'me' was not correct"),
        }

        match map.get("too") {
            Some(Value::Int(1)) => (),
            _ => panic!("Key 'too' was not correct"),
        }

        match map.get("neg_two") {
            Some(Value::Int(-2)) => (),
            _ => panic!("Key 'neg_two' was not correct"),
        }

        match map.get("_gaa") {
            Some(Value::Bool(true)) => (),
            _ => panic!("Key '_gaa' was not correct"),
        }

        match map.get("shee") {
            Some(Value::Bool(false)) => (),
            _ => panic!("Key 'shee' was not correct"),
        }

        match map.get("float_test0") {
            Some(Value::Float(11.0)) => (),
            _ => panic!("Key 'float_test0' was not correct"),
        }
        match map.get("float_test1") {
            Some(Value::Float(1.1)) => (),
            _ => panic!("Key 'float_test1' was not correct"),
        }
        match map.get("float_test2") {
            Some(Value::Float(0.11)) => (),
            _ => panic!("Key 'float_test2' was not correct"),
        }
    }
}
