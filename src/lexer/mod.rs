use token::Token;

pub mod token;

pub struct Lexer {
    input: String,
    position: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer { input, position: 0 }
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn next_char(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        while let Some(c) = self.next_char() {
            match c {
                '+' => {
                    self.advance();
                    tokens.push(Token::Plus)
                }
                '-' => {
                    self.advance();
                    tokens.push(Token::Minus)
                }
                '*' => {
                    self.advance();
                    tokens.push(Token::Multiply)
                }
                '/' => {
                    self.advance();
                    tokens.push(Token::Divide)
                }
                '%' => {
                    self.advance();
                    tokens.push(Token::Modulo)
                }
                '=' => {
                    self.advance();
                    tokens.push(Token::Equal)
                }
                c => {
                    println!("An unexpected character was found by the lexer '{}'.", c);
                }
            }
        }
        tokens
    }
}
