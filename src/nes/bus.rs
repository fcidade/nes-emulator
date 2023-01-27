pub struct Bus {
    memory: [u8; 64 * 1024],
}

impl Bus {
    pub fn new() -> Self {
        Self {
            memory: [0; 64 * 1024],
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn read_bulk(&self, addr: u16, size: u16) -> Vec<u8> {
        self.memory[(addr as usize)..((addr + size) as usize)].to_vec()
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize] = value;
    }
}
