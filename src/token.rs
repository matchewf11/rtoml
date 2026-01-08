#[derive(Debug, PartialEq)]
pub enum Token {
    String(String),
    Int(i32),
    Float(f32),
    Equal,
    Ident(String),
    NewLine,
    True,
    False,
}
pub fn is_identifier(c: char) -> bool {
    c.is_ascii_uppercase() || c.is_ascii_lowercase() || c == '_'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_identifier() {
        // passes
        for c in 'A'..='Z' {
            assert!(is_identifier(c), "Expected '{}' to be an identifier", c);
        }

        for c in 'a'..='z' {
            assert!(is_identifier(c), "Expected '{}' to be an identifier", c);
        }

        assert!(is_identifier('_'), "Expected '_' to be an identifier");

        for c in '0'..='9' {
            assert!(
                !is_identifier(c),
                "Expected '{}' NOT to be an identifier",
                c
            );
        }

        // fails
        let symbols = [
            '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '+', '=', ' ',
        ];
        for &c in &symbols {
            assert!(
                !is_identifier(c),
                "Expected '{}' NOT to be an identifier",
                c
            );
        }
    }
}
