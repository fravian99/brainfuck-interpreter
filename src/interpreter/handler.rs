use super::{instructions::Instruction, memory::Memory};

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
    pub fn execute(&mut self, code: &Vec<Instruction>) {
        for instruction in code {
            match instruction {
                Instruction::IncrementPointer(x) => self.memory.increment_pointer(*x),
                Instruction::DecrementPointer(x) => self.memory.decrement_pointer(*x),
                Instruction::IncrementValue(x) => self.memory.increment_value(*x),
                Instruction::DecrementValue(x) => self.memory.decrement_value(*x),
                Instruction::Output => {
                    print!("{}", self.memory.get_value());
                }
                Instruction::None => (),
                _ => (),
            }
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
}
