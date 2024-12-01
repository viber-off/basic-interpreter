use lexer::Lexer;

mod lexer;

fn main() {
    let source_code = String::from("+=*");

    let mut lexer = Lexer::new(source_code);

    let tokens = lexer.lex();

    for token in tokens {
        println!("Token: {:?}", token)
    }
}
