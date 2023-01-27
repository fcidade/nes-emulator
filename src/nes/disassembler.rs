use super::{addr_modes::AddrMode::*, instruction_summary::InstructionSummary, mos_6502::Mos6502};

pub fn disassemble(cpu: &Mos6502, start: u16, end: u16) -> Vec<String> {
    let mut pc = start;
    let mut disassembled: Vec<String> = vec![];
    while pc <= end {
        let sum = InstructionSummary::from(cpu.read_byte(pc));
        pc += 1;
        let (word, _, byte) = cpu.read_word_and_bytes(pc);

        let (params, bytes_to_skip) = match sum.addr_mode {
            Implied => ("".into(), 1),
            Immediate => (format!("#${:02X}", byte), 1),
            ZeroPage => (format!("${:02X}", byte), 1),
            ZeroPageOffsetX => (format!("${:02X}, X", byte), 1),
            ZeroPageOffsetY => (format!("${:02X}, Y", byte), 1),
            Absolute => (format!("${:04X}", word), 2),
            AbsoluteOffsetX => (format!("${:04X}, X", word), 1),
            AbsoluteOffsetY => (format!("${:04X}, Y", word), 1),
            Indirect => (format!("(${:04X})", word), 1),
            IndirectOffsetX => (format!("(${:02X}, X)", byte), 1),
            IndirectOffsetY => (format!("(${:02X}), Y", byte), 1),
            Relative => (format!("${:02X} [${:04X}]", byte, word), 1),
        };
        pc += bytes_to_skip;

        disassembled.push(format!("{} {}", sum.instruction.to_string(), params))
    }
    disassembled
}
