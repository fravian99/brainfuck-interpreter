use std::fmt::Display;

/// # Memory
/// Struct that contains a vector of bytes and a pointer.
/// ## Contains
/// `blocks` contains the bytes stored.
///
/// `pointer` containst the position of the value in `blocks`.
///
pub struct Memory {
    blocks: Vec<u8>,
    pointer: u32,
}
impl Memory {
    /// Returns an instance of the struct [Memory].
    ///
    /// `pointer` is the initial pointer of the memory.
    pub fn new(pointer: u32) -> Self {
        let blocks = vec![0; u32::MAX as usize + 1];
        Self { blocks, pointer }
    }

    ///Returns the actual pointer.
    pub fn get_pointer(&self) -> u32 {
        self.pointer
    }
    /// Returns a reference to the blocks vector.
    pub fn get_blocks(&self) -> &Vec<u8> {
        &self.blocks
    }
    /// Returns the actual value pointed by the pointer.
    pub fn get_value(&self) -> u8 {
        self.blocks[self.pointer as usize]
    }
    pub fn increment_pointer(&mut self, increment: u32) {
        self.pointer += increment
    }
    pub fn decrement_pointer(&mut self, decrement: u32) {
        self.pointer -= decrement
    }
    pub fn increment_value(&mut self, increment: u8) {
        self.blocks[self.pointer as usize] += increment;
    }
    pub fn decrement_value(&mut self, decrement: u8) {
        self.blocks[self.pointer as usize] -= decrement;
    }
}

impl Display for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Pointer: {}\n Memory value:\n {:?}",
            self.pointer,
            self.get_value()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Memory;

    #[test]
    pub fn test_new() {
        let memory = Memory::new(0);
        let pointer = memory.get_pointer();
        let value = memory.get_value();
        assert_eq!(pointer, 0);
        assert_eq!(value, 0);
        println!("{}", memory);
        println!("{}", memory);

        let memory = Memory::new(u32::MAX);
        let pointer = memory.get_pointer();
        let value = memory.get_value();
        assert_eq!(pointer, u32::max_value());
        assert_eq!(value, 0);
        println!("{}", memory);
        println!("{}", memory);
    }

    #[test]
    pub fn test_increment_pointer() {
        let mut memory = Memory::new(0);
        let pointer = 0;
        let increment = 1;

        memory.increment_pointer(increment);
        assert_eq!(memory.get_pointer(), increment);

        memory.increment_pointer(increment);
        assert_eq!(memory.get_pointer(), increment * 2);

        let mut memory = Memory::new(pointer);
        //let max_range = 100000000;
        let max_range = 10;
        for i in 0..max_range {
            memory.increment_pointer(increment);
            assert_eq!(memory.get_pointer(), pointer + (i + 1) * increment);
        }
    }

    #[test]
    pub fn test_decrement_pointer() {
        let pointer = 10;
        let mut memory = Memory::new(pointer);
        let decrement = 1;
        for i in 9..0 {
            memory.decrement_pointer(decrement);
            assert_eq!(
                memory.get_pointer(),
                i,
                "Testing decrementing pointer {}, with decrement {}",
                memory.get_pointer(),
                decrement
            );
        }
        let pointer = u32::MAX;
        let mut memory = Memory::new(pointer);
        let decrement = pointer;
        memory.decrement_pointer(u32::MAX);
        assert_eq!(
            memory.get_pointer(),
            0,
            "Testing decrementing pointer {}, with decrement {}",
            memory.get_pointer(),
            decrement
        );
    }

    #[test]
    pub fn test_increment_value() {
        let pointer = 0;
        let increment = 1;
        let mut memory = Memory::new(pointer);
        let max_range = 5;
        for _i in 0..max_range {
            for j in 0..u8::MAX {
                memory.increment_value(increment);
                let storage_val = memory.get_blocks()[memory.get_pointer() as usize];
                assert_eq!(storage_val, j + 1);
            }
            memory.increment_pointer(increment as u32);
        }
        let mut memory = Memory::new(pointer);
        let increment = u8::MAX;
        for _i in 0..max_range {
            memory.increment_value(increment);
            let storage_val = memory.get_blocks()[memory.get_pointer() as usize];
            assert_eq!(storage_val, increment);
            memory.increment_pointer(increment as u32);
        }
    }

    #[test]
    pub fn test_decrement_value() {
        let pointer = 0;
        let mut memory = Memory::new(pointer);

        memory.increment_value(2);
        memory.decrement_value(1);
        assert_eq!(memory.get_value(), 1);

        memory.increment_pointer(1); //New value
        memory.increment_value(u8::MAX);
        memory.decrement_value(u8::MAX);
        assert_eq!(memory.get_value(), 0);
    }
}
