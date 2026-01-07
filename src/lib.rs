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
        assert_eq!(map.get("me"), Some(&Value::String("hello".to_string())));
        assert_eq!(map.get("too"), Some(&Value::Int(1)));
        assert_eq!(map.get("neg_two"), Some(&Value::Int(-2)));
        assert_eq!(map.get("_gaa"), Some(&Value::Bool(true)));
        assert_eq!(map.get("shee"), Some(&Value::Bool(false)));
        assert_eq!(map.get("float_test0"), Some(&Value::Float(11.0)));
        assert_eq!(map.get("float_test1"), Some(&Value::Float(1.1)));
        assert_eq!(map.get("float_test2"), Some(&Value::Float(0.11)));
    }
}
