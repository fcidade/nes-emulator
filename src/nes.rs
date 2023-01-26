pub mod bus;
pub mod instructions;

use std::cell::RefCell;

use self::{bus::Bus, instructions::{Instruction}};

pub enum Flag {
    Carry,
    Zero,
    DisableInterrupts,
    DecimalMode,
    Break,
    Unused,
    Overflow,
    Negative,
}

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

pub struct InstructionSummary {
    addr_mode: AddrMode,
    instruction: Instruction,
    cycles: u8,
}

pub struct Mos6502 {
    pub pc: u16,
    pub status_flags: u8,
    pub stack_ptr: u16,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub bus: RefCell<Bus>,
    pub cycles: u8,
    pub fetched: u8,
    pub addr_abs: u16,
    pub addr_rel: u16,
    pub opcode: u8,
}

impl Mos6502 {
    pub fn new(bus: RefCell<Bus>) -> Self {
        Self {
            pc: 0,
            stack_ptr: 0,
            status_flags: 0,
            a: 0,
            x: 0,
            y: 0,
            bus,
            cycles: 0,
            fetched: 0,
            addr_abs: 0,
            addr_rel: 0,
            opcode: 0,
        }
    }

    pub fn clock(&mut self) {
        if self.cycles == 0 {
            self.opcode = self.read_byte(self.pc);

            let InstructionSummary {
                cycles,
                addr_mode,
                instruction,
            } = self.lookup_opcode(self.opcode);

            self.pc += 1;
            self.cycles = cycles;

            let addr_mode_additional_cycles = self.handle_addr_mode(addr_mode);
            let instruction_additional_cycles = self.handle_instruction(instruction);

            self.cycles += addr_mode_additional_cycles & instruction_additional_cycles;
        }

        self.cycles -= 1;
    }

    fn lookup_opcode(&self, opcode: u8) -> InstructionSummary {
        let instruction = match opcode {
            _ => InstructionSummary {
                addr_mode: AddrMode::Immediate,
                instruction: Instruction::NOP_NoOperation,
                cycles: 1,
            },
        };
        instruction
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
        let low_byte = self.read_byte(addr);
        let high_byte = self.read_byte(addr + 1);
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

    fn fetch(&mut self) -> u8 {
        match self.lookup_opcode(self.opcode).addr_mode {
            AddrMode::Implied => {}
            _ => {
                self.fetched = self.read_byte(self.addr_abs);
            }
        };
        self.fetched
    }

    fn get_flag(&self, flag: Flag) -> bool {
        let bit_mask = self.get_status_bit_mask(flag);
        self.status_flags & bit_mask == 1
    }

    fn set_flag(&mut self, flag: Flag, val: bool) {
        let bit_mask = self.get_status_bit_mask(flag);
        let val = val as u8;
        if val != 0 {
            self.status_flags |= val << bit_mask;
        } else {
            self.status_flags &= !(val << bit_mask);
        }
    }

    fn get_status_bit_mask(&self, flag: Flag) -> u8 {
        match flag {
            Flag::Carry => 0b00000001,
            Flag::Zero => 0b00000010,
            Flag::DisableInterrupts => 0b00000100,
            Flag::DecimalMode => 0b00001000,
            Flag::Break => 0b00010000,
            Flag::Unused => 0b00100000,
            Flag::Overflow => 0b01000000,
            Flag::Negative => 0b10000000,
        }
    }
}
