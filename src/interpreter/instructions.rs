#[derive(PartialEq, Eq, Debug)]
pub enum Instruction {
    IncrementPointer(u32),
    DecrementPointer(u32),
    IncrementValue(u8),
    DecrementValue(u8),
    Output,
    Input,
    OpenLoop,
    CloseLoop,
    None,
}

impl From<char> for Instruction {
    fn from(value: char) -> Self {
        match value {
            '>' => Instruction::IncrementPointer(1),
            '<' => Instruction::DecrementPointer(1),
            '+' => Instruction::IncrementValue(1),
            '-' => Instruction::DecrementValue(1),
            '.' => Instruction::Output,
            ',' => Instruction::Input,
            '[' => Instruction::OpenLoop,
            ']' => Instruction::CloseLoop,
            _ => Instruction::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Instruction;
    #[test]
    pub fn test_from_char() {
        dbg!(Instruction::IncrementPointer(1));
        assert_eq!(Instruction::from('>'), Instruction::IncrementPointer(1));
        assert_eq!(Instruction::from('<'), Instruction::DecrementPointer(1));
        assert_eq!(Instruction::from('+'), Instruction::IncrementValue(1));
        assert_eq!(Instruction::from('-'), Instruction::DecrementValue(1));
        assert_eq!(Instruction::from('.'), Instruction::Output);
        assert_eq!(Instruction::from(','), Instruction::Input);
        assert_eq!(Instruction::from('['), Instruction::OpenLoop);
        assert_eq!(Instruction::from(']'), Instruction::CloseLoop);
        assert_eq!(Instruction::from('?'), Instruction::None);
    }
}
