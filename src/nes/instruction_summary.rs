use super::{addr_modes::AddrMode, instructions::Instruction};
use AddrMode::*;
use Instruction::*;

pub struct InstructionSummary {
    pub addr_mode: AddrMode,
    pub instruction: Instruction,
    pub cycles: u8,
}

impl InstructionSummary {
    pub fn new(addr_mode: AddrMode, instruction: Instruction, cycles: u8) -> Self {
        Self {
            addr_mode,
            cycles,
            instruction,
        }
    }
}

impl From<u8> for InstructionSummary {
    fn from(opcode: u8) -> Self {
        match opcode {
            0x00 => Self::new(Implied, BRK_ForceBreak, 7),
            0x01 => Self::new(IndirectOffsetX, ORA_ORMemoryWithAcc, 6),
            0x05 => Self::new(ZeroPage, ORA_ORMemoryWithAcc, 3),
            0x06 => Self::new(ZeroPage, ASL_ShiftLeftOneBit, 5),
            0x08 => Self::new(Implied, PHP_PushProcessorStatusOnStack, 3),
            0x09 => Self::new(Immediate, ORA_ORMemoryWithAcc, 2),
            0x0A => Self::new(Implied, ASL_ShiftLeftOneBit, 2),
            0x0D => Self::new(Absolute, ORA_ORMemoryWithAcc, 4),
            0x0E => Self::new(Absolute, ASL_ShiftLeftOneBit, 6),
            _ => Self::new(Implied, InvalidInstruction, 2),
        }
    }
}
