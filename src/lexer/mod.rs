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

    fn handle_digits(&mut self, c: char) -> Token {
        let mut number = c.to_string();
        let mut has_dot = false;
        self.advance();

        while let Some(c) = self.next_char() {
            if c.is_digit(10) {
                number.push(c);
                self.advance();
            } else if c == '.' {
                if has_dot {
                    break;
                } else {
                    has_dot = true;
                    number.push(c);
                    self.advance();
                }
            } else {
                break;
            }
        }

        let token;

        if has_dot {
            token = match number.parse::<f64>() {
                Ok(value) => Token::Float(value),
                Err(_) => panic!("Failed to parse '{}' as f64", number),
            }
        } else {
            token = match number.parse::<i64>() {
                Ok(value) => Token::Number(value),
                Err(_) => panic!("Failed to parse '{}' as i64", number),
            }
        };

        token
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
                c if c.is_digit(10) => {
                    let token = self.handle_digits(c);
                    tokens.push(token);
                }
                c if c.is_whitespace() => {
                    self.advance();
                }
                c => {
                    println!("An unexpected character was found by the lexer '{}'.", c);
                }
            }
        }
        tokens
    }
}
