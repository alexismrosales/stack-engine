use std::io;

use crate::instructions::{Instruction, OpType};

pub fn read() -> String {
    let mut input = String::new();
    println!("Write an expression in Polish notation: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading line");
    input
}

pub fn trim_input(input: &str) -> Vec<String> {
    let trimmed_input: Vec<String> = input
        .split_whitespace()
        .map(|s| s.to_string()) // This is important to create new references in heap
        .collect();
    trimmed_input
}

pub fn tokenizer(items: Vec<String>) -> Vec<Instruction> {
    let mut tokens: Vec<Instruction> = Vec::new();
    //  Valid cases:
    //   10000 2000 +
    //   300 200 / 4 +
    //  Invalid cases:
    //   10000 hi
    //   100 2 +++
    // Scan available characters
    for i in items.iter() {
        // In case parse to f64 fails it just will assume first this might be an
        // operator, in any other case it panics
        let instruction: Instruction = if let Ok(n) = i.parse::<f64>() {
            Instruction::Push(n)
        } else if i.len() == 1 {
            // Safely unwrapped
            let sign = i.chars().next().unwrap();
            match sign {
                '+' => Instruction::Operator(OpType::Add),
                '-' => Instruction::Operator(OpType::Sub),
                '*' | 'x' => Instruction::Operator(OpType::Mul),
                '/' => Instruction::Operator(OpType::Div),
                _ => panic!("Invalid character: {sign}"),
            }
        } else {
            panic!("Invalid series of characters {i}");
        };
        tokens.push(instruction);
    }
    tokens
}
