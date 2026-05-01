#[derive(Debug)]
#[allow(dead_code)]
pub enum Instruction {
    Push(f64),
    Operator(OpType),
}

#[derive(Debug)]
pub enum OpType {
    Add,
    Sub,
    Mul,
    Div,
}
