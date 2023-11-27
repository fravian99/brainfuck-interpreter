#[derive(PartialEq, Eq, Debug)]
pub enum Instruction {
    // TODO Implement the other instructions
    IncrementPointer(u32),
    DecrementPointer(u32),
    IncrementValue(u8),
    DecrementValue(u8),
    None,
}

impl From<&char> for Instruction {
    fn from(value: &char) -> Self {
        // TODO Implement match cases for the other instructions
        match value {
            '>' => Instruction::IncrementPointer(1),
            '<' => Instruction::DecrementPointer(1),
            '+' => Instruction::IncrementValue(1),
            '-' => Instruction::DecrementValue(1),
            _ => Instruction::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Instruction;
    #[test]
    pub fn test_from_char() {
        assert_eq!(Instruction::from(&'>'), Instruction::IncrementPointer(1));
        assert_eq!(Instruction::from(&'<'), Instruction::DecrementPointer(1));
        assert_eq!(Instruction::from(&'+'), Instruction::IncrementValue(1));
        assert_eq!(Instruction::from(&'-'), Instruction::DecrementValue(1));
        assert_eq!(Instruction::from(&'?'), Instruction::None);
        // TODO Implement the other cases
        todo!();
    }
}
