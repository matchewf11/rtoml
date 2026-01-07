#[derive(Debug)]
pub enum Token {
    String(String),
    Int(i32),
    Equal,
    Ident(String),
    NewLine,
    True,
    False,
}
pub fn is_identifier(c: char) -> bool {
    c.is_ascii_uppercase() || c.is_ascii_lowercase() || c == '_'
}
