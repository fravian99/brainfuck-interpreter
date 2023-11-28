use super::instructions::Instruction;

pub fn from_string_to_instructions(code: &str) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    for i in code.chars() {
        instructions.push(Instruction::from(&i));
    }
    instructions
}

#[cfg(test)]
mod tests {
    use super::from_string_to_instructions;
    use crate::interpreter::instructions::Instruction;

    #[test]
    fn test_from_string_to_instructions() {
        let code_string = "><+-";
        let expected_instructions = vec![
            Instruction::IncrementPointer(1),
            Instruction::DecrementPointer(1),
            Instruction::IncrementValue(1),
            Instruction::DecrementValue(1),
        ];
        let instructions = from_string_to_instructions(code_string);
        assert_eq!(expected_instructions, instructions);
    }
}
