

use std::cell::RefCell;

use super::{bus::Bus, instruction_summary::InstructionSummary, addr_modes::AddrMode};

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

            let instruction = InstructionSummary::from(self.opcode);

            self.pc += 1;
            self.cycles = instruction.cycles;

            let addr_mode_additional_cycles = self.handle_addr_mode(instruction.addr_mode);
            let instruction_additional_cycles = self.handle_instruction(instruction.instruction);

            self.cycles += addr_mode_additional_cycles & instruction_additional_cycles;
        }

        self.cycles -= 1;
    }

    pub fn read_word_and_bytes(&mut self, addr: u16) -> (u16, u8, u8) {
        let low_byte = self.read_byte(addr);
        let high_byte = self.read_byte(addr + 1);
        (
            ((high_byte as u16) << 8) | low_byte as u16,
            high_byte,
            low_byte,
        )
    }

    pub fn read_word(&mut self, addr: u16) -> u16 {
        self.read_word_and_bytes(addr).0
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        self.bus.borrow().read(addr)
    }

    pub fn fetch(&mut self) -> u8 {
        match InstructionSummary::from(self.opcode).addr_mode {
            AddrMode::Implied => {}
            _ => {
                self.fetched = self.read_byte(self.addr_abs);
            }
        };
        self.fetched
    }

    pub fn get_flag(&self, flag: Flag) -> bool {
        let bit_mask = self.get_status_bit_mask(flag);
        self.status_flags & bit_mask == 1
    }

    pub fn set_flag(&mut self, flag: Flag, val: bool) {
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
