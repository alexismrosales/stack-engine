use crate::{
    engine::Engine,
    scanner::{tokenizer, trim_input},
};

mod engine;
mod instructions;
mod scanner;

fn main() {
    let input: String = scanner::read();
    let items = trim_input(&input);
    let tokenized_items = tokenizer(items);
    println!("=====================");
    println!("Tokenized_items: {:?}", tokenized_items);
    let mut e = Engine::new();
    let result = e.execute(tokenized_items);
    println!("=====================");
    println!("The result is: {result}");
}
