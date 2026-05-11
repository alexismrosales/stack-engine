use crate::instructions::{Instruction, OpType};

enum MachineStatus {
    Running,
    Success,
    Error(String),
}

pub struct Engine {
    stack: Vec<f64>,
    pc: usize,
    status: MachineStatus,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            pc: 0,
            status: MachineStatus::Running,
        }
    }

    pub fn execute(&mut self, mut tokenized_items: Vec<Instruction>) -> f64 {
        while let Some(item) = tokenized_items.pop() {
            self.exec_instruction(item);
            // Show current status
            match &self.status {
                MachineStatus::Running => println!("[DEBUG][{}] Status: RUNNING", self.pc),
                MachineStatus::Error(e) => {
                    let em = format!("[ERROR][{}]: {}", self.pc, e);
                    panic!("Error {em}")
                }
                MachineStatus::Success => unreachable!(),
            }
        }
        // Once the tokens were all used, there is supposed to have only one value in the stack,
        // you can safely pop it
        match self.stack.pop() {
            Some(x) => {
                // Mark as success
                self.status = MachineStatus::Success;
                self.pc += 1;
                println!("[DEBUG][{}] Status: SUCCESS", self.pc);
                x
            }
            None => panic!("[ERROR] Error evaluating the last value in {}", self.pc),
        }
    }

    fn exec_instruction(&mut self, instruction: Instruction) {
        // Only saves the value and then just continue to the next token
        // On this point we know that there is only a character and to
        // evaluate the caracter we should have available two numbers,
        // in other case it panics
        let res = match instruction {
            Instruction::Push(v) => {
                self.stack.push(v);
                Ok(())
            }
            // Only saves the value and then just continue to the next token
            // On this point we know that there is only a character and to
            // evaluate the caracter we should have available two numbers,
            // in other case it panics
            Instruction::Operator(op) => match op {
                OpType::Add => self.execute_add(),
                OpType::Sub => self.execute_sub(),
                OpType::Mul => self.execute_mul(),
                OpType::Div => self.execute_div(),
            },
        };
        if let Err(e) = res {
            self.status = MachineStatus::Error(e);
        } else {
            self.end_instruction();
        }
    }

    fn pop_two(&mut self) -> Result<(f64, f64), String> {
        let n2 = self
            .stack
            .pop()
            .ok_or("Missing second operand (stack empty)".to_string())?;

        let n1 = self
            .stack
            .pop()
            .ok_or("Missing first operand (stack empty)".to_string())?;
        Ok((n1, n2))
    }

    fn execute_add(&mut self) -> Result<(), String> {
        let (n1, n2) = self.pop_two()?;
        self.stack.push(n1 + n2);
        Ok(())
    }

    fn execute_sub(&mut self) -> Result<(), String> {
        let (n1, n2) = self.pop_two()?;
        self.stack.push(n1 - n2);
        Ok(())
    }

    fn execute_mul(&mut self) -> Result<(), String> {
        let (n1, n2) = self.pop_two()?;
        self.stack.push(n1 * n2);
        Ok(())
    }

    fn execute_div(&mut self) -> Result<(), String> {
        let (n1, n2) = self.pop_two()?;
        if n2 == 0.00 {
            return Err("Division by zero".to_string());
        }
        self.stack.push(n1 / n2);
        Ok(())
    }

    fn end_instruction(&mut self) {
        self.pc += 1;
        self.status = MachineStatus::Running;
    }
}
