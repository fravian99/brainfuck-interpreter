use super::{instructions::Instruction, memory::Memory};
use std::io;

/// Runs the instructions recived
pub struct Handler {
    memory: Memory,
}

impl Handler {
    pub fn new() -> Self {
        Self {
            memory: Memory::new(0),
        }
    }
    pub fn get_memory(&self) -> &Memory {
        &self.memory
    }
    pub fn clear_memory(&mut self){
        self.memory = Memory::default();
    }
    pub fn execute(&mut self, code: &Vec<Instruction>) {
        let mut instruction_pointer = 0;
        let mut loops: Vec<usize> = Vec::new();
        while instruction_pointer < code.len() {
            let instruction = &code[instruction_pointer];
            match instruction {
                Instruction::IncrementPointer(x) => self.memory.increment_pointer(*x),
                Instruction::DecrementPointer(x) => self.memory.decrement_pointer(*x),
                Instruction::IncrementValue(x) => self.memory.increment_value(*x),
                Instruction::DecrementValue(x) => self.memory.decrement_value(*x),
                Instruction::Output => {
                    print!("{} ", self.memory.get_value());
                }
                Instruction::Input => {
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Error");
                    self.memory.set_value(input.as_bytes()[0]);
                }
                Instruction::OpenLoop => {
                    if self.memory.get_value() != 0 {
                        loops.push(instruction_pointer);
                    } else {
                        loops.pop();
                    }
                }
                Instruction::CloseLoop => {
                    if self.memory.get_value() != 0 {
                        if let Some(&x) = loops.last() { instruction_pointer = x }
                    } else if loops.last().is_some() {
                        loops.pop();
                    }
                }
                Instruction::None => (),
            }
            instruction_pointer += 1
        }
    }
}
impl Default for Handler {
    fn default() -> Self {
        Self {
            memory: Memory::new(0),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::interpreter::instructions::Instruction;

    use super::Handler;

    #[test]
    fn test_execute_pointer() {
        let mut instructions: Vec<Instruction> = Vec::new();
        let initial = 2;
        let mut handler = Handler::default();

        instructions.push(Instruction::IncrementPointer(initial));
        handler.execute(&instructions);
        assert_eq!(handler.get_memory().get_pointer(), initial);

        instructions.push(Instruction::DecrementPointer(1));
        handler.execute(&instructions);
        assert_eq!(handler.get_memory().get_pointer(), initial * 2 - 1);
    }

    #[test]
    fn test_execute_value() {
        let mut instructions: Vec<Instruction> = Vec::new();
        let initial = 2;
        let mut handler = Handler::new();

        instructions.push(Instruction::IncrementValue(initial));
        handler.execute(&instructions);
        assert_eq!(handler.get_memory().get_value(), initial);

        instructions.push(Instruction::DecrementValue(1));
        handler.execute(&instructions);
        assert_eq!(handler.get_memory().get_value(), initial * 2 - 1);
    }

    #[test]
    fn test_execute_others() {
        let mut instructions: Vec<Instruction> = Vec::new();
        let mut handler = Handler::new();

        instructions.push(Instruction::None);
        handler.execute(&instructions);
        assert!(true);

        instructions.push(Instruction::Output);
        handler.execute(&instructions);
        assert!(true);
    }

    #[test]
    fn test_input() {
        let mut instructions: Vec<Instruction> = Vec::new();
        let mut handler = Handler::new();

        instructions.push(Instruction::Input);
        handler.execute(&instructions);
        assert!(true);
    }

    #[test]
    fn test_execute_loops() {
        let mut instructions: Vec<Instruction> = Vec::new();
        let mut handler = Handler::new();

        instructions.push(Instruction::IncrementValue(10));
        instructions.push(Instruction::OpenLoop);
        instructions.push(Instruction::DecrementValue(1));
        instructions.push(Instruction::CloseLoop);
        handler.execute(&instructions);
        assert_eq!(handler.get_memory().get_value(), 0);

    }
}
