use super::Mos6502;

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

use AddrMode::*;

impl Mos6502 {
    pub fn handle_addr_mode(&mut self, addr_mode: AddrMode) -> u8 {
        match addr_mode {
            Implied => {
                self.fetched = self.a;
            }
            Immediate => {
                self.addr_abs = self.pc;
                self.pc += 1;
            }
            ZeroPage => {
                self.addr_abs = self.read_byte(self.pc) as u16;
                self.pc += 1;
                self.addr_abs &= 0x00FF;
            }
            ZeroPageOffsetX => {
                self.addr_abs = self.read_byte(self.pc + self.x as u16) as u16;
                self.pc += 1;
                self.addr_abs &= 0x00FF;
            }
            ZeroPageOffsetY => {
                self.addr_abs = self.read_byte(self.pc + self.y as u16) as u16;
                self.pc += 1;
                self.addr_abs &= 0x00FF;
            }
            Absolute => {
                let addr = self.read_word(self.pc);
                self.pc += 2;

                self.addr_abs = addr;
            }
            AbsoluteOffsetX => {
                let (addr, hi, _) = self.read_word_and_bytes(self.pc);
                self.pc += 2;

                self.addr_abs = addr;
                self.addr_abs += self.x as u16;

                if self.addr_abs & 0xFF00 != ((hi as u16) << 8) {
                    return 1;
                }
            }
            AbsoluteOffsetY => {
                let (addr, hi, _) = self.read_word_and_bytes(self.pc);
                self.pc += 2;

                self.addr_abs = addr;
                self.addr_abs += self.y as u16;

                if self.addr_abs & 0xFF00 != ((hi as u16) << 8) {
                    return 1;
                }
            }
            Indirect => {
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
            IndirectOffsetX => {
                let supplied_address = self.read_byte(self.pc);
                self.pc += 1;

                self.addr_abs = self.read_word(supplied_address as u16 + self.x as u16);
            }
            IndirectOffsetY => {
                let supplied_address = self.read_byte(self.pc);
                self.pc += 1;

                let (addr, hi, _) = self.read_word_and_bytes(supplied_address as u16);
                self.addr_abs = addr;
                self.addr_abs += self.y as u16;

                if self.addr_abs & 0xFF00 != ((hi as u16) << 8) {
                    return 1;
                }
            }
            Relative => {
                self.addr_rel = self.read_byte(self.pc) as u16;
                self.pc += 1;
                if self.addr_rel & 0b10000000 != 0 {
                    self.addr_rel |= 0xFF00;
                }
            }
        }
        return 0;
    }
}
