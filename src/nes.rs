use std::cell::RefCell;

pub enum AddrMode {
    Implied,
    Immediate,
    ZeroPage,
    ZeroPageOffsetX,
    ZeroPageOffsetY,
    Absolute,
    AbsoluteOffsetX,
    AbsoluteOffsetY,
    Indirect,
    IndirectOffsetX,
    IndirectOffsetY,
    Relative,
}
pub enum Instruction {
    NOP,
}

pub struct InstructionSummary {
    addr_mode: AddrMode,
    instruction: Instruction,
    cycles: u8,
}

pub struct Mos6502 {
    pub pc: u16,
    pub stack_ptr: u16,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub bus: RefCell<Bus>,
    pub cycles: u8,
    pub fetched: u8,
    pub addr_abs: u16,
    pub addr_rel: u16,
}

impl Mos6502 {
    pub fn new(bus: RefCell<Bus>) -> Self {
        Self {
            pc: 0,
            stack_ptr: 0,
            a: 0,
            x: 0,
            y: 0,
            bus,
            cycles: 0,
            fetched: 0,
            addr_abs: 0,
            addr_rel: 0,
        }
    }

    pub fn clock(&mut self) {
        if self.cycles == 0 {
            let InstructionSummary {
                cycles,
                addr_mode,
                instruction,
            } = self.match_instruction();

            self.pc += 1;
            self.cycles = cycles;

            let addr_mode_additional_cycles = self.handle_addr_mode(addr_mode);
            let instruction_additional_cycles = self.handle_instruction(instruction);

            self.cycles += addr_mode_additional_cycles & instruction_additional_cycles;
        }

        self.cycles -= 1;
    }

    fn match_instruction(&self) -> InstructionSummary {
        let opcode = self.read_byte(self.pc);
        match opcode {
            _ => InstructionSummary {
                addr_mode: AddrMode::Immediate,
                instruction: Instruction::NOP,
                cycles: 1,
            },
        }
    }

    fn handle_addr_mode(&mut self, addr_mode: AddrMode) -> u8 {
        match addr_mode {
            AddrMode::Implied => {
                self.fetched = self.a;
            }
            AddrMode::Immediate => {
                self.addr_abs = self.pc;
                self.pc += 1;
            }
            AddrMode::ZeroPage => {
                self.addr_abs = self.read_byte(self.pc) as u16;
                self.pc += 1;
                self.addr_abs &= 0x00FF;
            }
            AddrMode::ZeroPageOffsetX => {
                self.addr_abs = self.read_byte(self.pc + self.x as u16) as u16;
                self.pc += 1;
                self.addr_abs &= 0x00FF;
            }
            AddrMode::ZeroPageOffsetY => {
                self.addr_abs = self.read_byte(self.pc + self.y as u16) as u16;
                self.pc += 1;
                self.addr_abs &= 0x00FF;
            }
            AddrMode::Absolute => {
                let addr = self.read_word(self.pc);
                self.pc += 2;

                self.addr_abs = addr;
            }
            AddrMode::AbsoluteOffsetX => {
                let (addr, hi, _) = self.read_word_and_bytes(self.pc);
                self.pc += 2;

                self.addr_abs = addr;
                self.addr_abs += self.x as u16;

                if self.addr_abs & 0xFF00 != ((hi as u16) << 8) {
                    return 1;
                }
            }
            AddrMode::AbsoluteOffsetY => {
                let (addr, hi, _) = self.read_word_and_bytes(self.pc);
                self.pc += 2;

                self.addr_abs = addr;
                self.addr_abs += self.y as u16;

                if self.addr_abs & 0xFF00 != ((hi as u16) << 8) {
                    return 1;
                }
            }
            AddrMode::Indirect => {
                let (memory_pointer, _, lo) = self.read_word_and_bytes(self.pc);
                self.pc += 2;

                let low_byte = if lo == 0x00FF {
                    // Simulate page boundary hardware bug
                    self.read_byte(memory_pointer & 0xFF00) as u16
                } else {
                    // Behave normally
                    self.read_byte(memory_pointer + 1) as u16
                };

                let high_byte = self.read_byte(memory_pointer) as u16;
                self.addr_abs = (low_byte << 8) | high_byte;
            }
            AddrMode::IndirectOffsetX => {
                let supplied_address = self.read_byte(self.pc);
                self.pc += 1;

                self.addr_abs = self.read_word(supplied_address as u16 + self.x as u16);
            }
            AddrMode::IndirectOffsetY => {
                let supplied_address = self.read_byte(self.pc);
                self.pc += 1;

                let (addr, hi, _) = self.read_word_and_bytes(supplied_address as u16);
                self.addr_abs = addr;
                self.addr_abs += self.y as u16;

                if self.addr_abs & 0xFF00 != ((hi as u16) << 8) {
                    return 1;
                }
            }
            AddrMode::Relative => {
                self.addr_rel = self.read_byte(self.pc) as u16;
                self.pc += 1;
                if self.addr_rel & 0b10000000 != 0 {
                    self.addr_rel |= 0xFF00;
                }
            }
        }
        return 0;
    }

    fn read_word_and_bytes(&mut self, addr: u16) -> (u16, u8, u8) {
        let low_byte = self.read_byte(self.pc);
        let high_byte = self.read_byte(self.pc + 1);
        (
            ((high_byte as u16) << 8) | low_byte as u16,
            high_byte,
            low_byte,
        )
    }

    fn read_word(&mut self, addr: u16) -> u16 {
        self.read_word_and_bytes(addr).0
    }

    fn read_byte(&self, addr: u16) -> u8 {
        self.bus.borrow().read(addr)
    }

    fn handle_instruction(&mut self, instruction: Instruction) -> u8 {
        match instruction {
            Instruction::NOP => 0,
        }
    }
}

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

    pub fn write(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize] = value;
    }
}
