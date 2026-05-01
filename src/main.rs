use crate::scanner::{tokenizer, trim_input};

mod instructions;
mod scanner;

fn main() {
    println!("Hello, world!");
    let input: String = scanner::read();
    let items = trim_input(&input);
    let tokenized_items = tokenizer(items);
    println!("tokenized_items: {:?}", tokenized_items);
}
