mod errors;
mod token;
mod tokenizer;
mod value;

use tokenizer::Tokenizer;

fn main() {
    let mut lexer = Tokenizer::new("Hello, world!");
    let t = lexer.next_token().unwrap();
    println!("{:?}", t);
    let hello = "Здравствуйте";
    for c in hello.chars() {
        println!("{c}");
    }
}
