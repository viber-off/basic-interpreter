#[derive(Debug)]
pub enum Token {
    Equal,
    Plus,
    Multiply,
    Minus,
    Divide,
    Modulo,

    LeftParen,
    RightParen,

    Number(i64),
    Float(f64),
}
