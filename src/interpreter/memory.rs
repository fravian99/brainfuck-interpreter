pub struct Memory {
    blocks: Vec<u8>,
    pointer: u32,
}
impl Memory {
    pub fn new(pointer: u32) -> Self {
        let blocks = vec![0; u32::MAX as usize + 1];
        Self { blocks, pointer }
    }
    pub fn get_pointer(&self) -> u32 {
        self.pointer
    }
    pub fn get_value(&self) -> u8 {
        self.blocks[self.pointer as usize]
    }
}
