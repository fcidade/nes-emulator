
#[allow(non_camel_case_types)]
pub enum Instruction {
    ADC_AddMemoryToAccWithCarry,
    AND_AndBitwiseWithAcc,
    ASL_ShiftLeftOneBit,
    BCC_BranchOnCarryClear,
    BCS_BranchOnCarrySet,
    BEQ_BranchOnResultZero,
    BIT_BitTestInMemoryWithAcc,
    BMI_BranchOnResultMinus,
    BNE_BranchOnResultNotZero,
    BPL_BranchOnResultPlus,
    BRK_ForceBreak,
    BVC_BranchOnOverflowClear,
    BVS_BranchOnOverflowSet,
    CLC_ClearCarryFlag,
    CLD_ClearDecimalMode,
    CLI_ClearInterruptDisableBit,
    CLV_ClearOverflowFlag,
    CMP_CompareMemoryAndAcc,
    CPX_CompareMemoryAndX,
    CPY_CompareMemoryAndY,
    DEC_DecrementMemoryByOne,
    DEX_DecrementXByOne,
    DEY_DecrementYByOne,
    EOR_ExclusiveORMemoryWithAcc,
    INC_IncrementMemoryByOne,
    INX_IncrementXByOne,
    INY_IncrementYByOne,
    JMP_JumpTo,
    JSR_JumpToSavingReturnAddr,
    LDA_LoadAccWithMemory,
    LDX_LoadXWithMemory,
    LDY_LoadYWithMemory,
    LSR_ShiftOneBitRight,
    NOP_NoOperation,
    ORA_ORMemoryWithAcc,
    PHA_PushAccOnStack,
    PHP_PushProcessorStatusOnStack,
    PLA_PullAccFromStack,
    PLP_PullProcessorStatusFromStack,
    ROL_RotateOneBitLeft,
    ROR_RotateOneBitRight,
    RTI_ReturnFromInterrupt,
    RTS_ReturnFromSubroutine,
    SBC_SubtractMemoryFromAccWithBorrow,
    SEC_SetCarryFlag,
    SED_SetDecimalMode,
    SEI_SetInterruptDisableStatus,
    STA_StoreAccInMemory,
    STX_StoreXInMemory,
    STY_StoreYInMemory,
    TAX_TransferAccToX,
    TAY_TransferAccToY,
    TSX_TransferStackPointerToX,
    TXA_TransferXToAcc,
    TXS_TransferXToStackRegister,
    TYA_TransferYToAcc,
    InvalidInstruction,
}

use Instruction::*;

use super::mos_6502::{Mos6502, Flag};

impl ToString for Instruction {
    fn to_string(&self) -> String {
        let str = match self {
            ADC_AddMemoryToAccWithCarry => "ADC",
            AND_AndBitwiseWithAcc => "AND",
            ASL_ShiftLeftOneBit => "ASL",
            BCC_BranchOnCarryClear => "BCC",
            BCS_BranchOnCarrySet => "BCS",
            BEQ_BranchOnResultZero => "BEQ",
            BIT_BitTestInMemoryWithAcc => "BIT",
            BMI_BranchOnResultMinus => "BMI",
            BNE_BranchOnResultNotZero => "BNE",
            BPL_BranchOnResultPlus => "BPL",
            BRK_ForceBreak => "BRK",
            BVC_BranchOnOverflowClear => "BVC",
            BVS_BranchOnOverflowSet => "BVS",
            CLC_ClearCarryFlag => "CLC",
            CLD_ClearDecimalMode => "CLD",
            CLI_ClearInterruptDisableBit => "CLI",
            CLV_ClearOverflowFlag => "CLV",
            CMP_CompareMemoryAndAcc => "CMP",
            CPX_CompareMemoryAndX => "CPX",
            CPY_CompareMemoryAndY => "CPY",
            DEC_DecrementMemoryByOne => "DEC",
            DEX_DecrementXByOne => "DEX",
            DEY_DecrementYByOne => "DEY",
            EOR_ExclusiveORMemoryWithAcc => "EOR",
            INC_IncrementMemoryByOne => "INC",
            INX_IncrementXByOne => "INX",
            INY_IncrementYByOne => "INY",
            JMP_JumpTo => "JMP",
            JSR_JumpToSavingReturnAddr => "JSR",
            LDA_LoadAccWithMemory => "LDA",
            LDX_LoadXWithMemory => "LDX",
            LDY_LoadYWithMemory => "LDY",
            LSR_ShiftOneBitRight => "LSR",
            NOP_NoOperation => "NOP",
            ORA_ORMemoryWithAcc => "ORA",
            PHA_PushAccOnStack => "PHA",
            PHP_PushProcessorStatusOnStack => "PHP",
            PLA_PullAccFromStack => "PLA",
            PLP_PullProcessorStatusFromStack => "PLP",
            ROL_RotateOneBitLeft => "ROL",
            ROR_RotateOneBitRight => "ROR",
            RTI_ReturnFromInterrupt => "RTI",
            RTS_ReturnFromSubroutine => "RTS",
            SBC_SubtractMemoryFromAccWithBorrow => "SBC",
            SEC_SetCarryFlag => "SEC",
            SED_SetDecimalMode => "SED",
            SEI_SetInterruptDisableStatus => "SEI",
            STA_StoreAccInMemory => "STA",
            STX_StoreXInMemory => "STX",
            STY_StoreYInMemory => "STY",
            TAX_TransferAccToX => "TAX",
            TAY_TransferAccToY => "TAY",
            TSX_TransferStackPointerToX => "TSX",
            TXA_TransferXToAcc => "TXA",
            TXS_TransferXToStackRegister => "TXS",
            TYA_TransferYToAcc => "TYA",
            InvalidInstruction => "INVALID INSTRUCTION!",
        };
        str.into()
    }
}

impl Mos6502 {
    pub fn handle_instruction(&mut self, instruction: Instruction) -> u8 {
        match instruction {
            ADC_AddMemoryToAccWithCarry => {}
            AND_AndBitwiseWithAcc => {
                self.fetch();
                self.a &= self.fetched;
                self.set_flag(Flag::Zero, self.a == 0);
                self.set_flag(Flag::Negative, self.a & 0b10000000 == 1);
                return 1;
            }
            ASL_ShiftLeftOneBit => {}
            BCC_BranchOnCarryClear => {}
            BCS_BranchOnCarrySet => {
                if self.get_flag(Flag::Carry) {
                    self.cycles += 1;
                    self.addr_abs = self.pc + self.addr_rel;

                    if self.addr_abs & 0xFF00 != self.pc & 0xFF00 {
                        self.cycles += 1;
                    }
                    self.pc = self.addr_abs
                }
            }
            BEQ_BranchOnResultZero => {}
            BIT_BitTestInMemoryWithAcc => {}
            BMI_BranchOnResultMinus => {}
            BNE_BranchOnResultNotZero => {}
            BPL_BranchOnResultPlus => {}
            BRK_ForceBreak => {}
            BVC_BranchOnOverflowClear => {}
            BVS_BranchOnOverflowSet => {}
            CLC_ClearCarryFlag => self.set_flag(Flag::Carry, false),
            CLD_ClearDecimalMode => self.set_flag(Flag::DecimalMode, false),
            CLI_ClearInterruptDisableBit => self.set_flag(Flag::DisableInterrupts, false),
            CLV_ClearOverflowFlag => {}
            CMP_CompareMemoryAndAcc => {}
            CPX_CompareMemoryAndX => {}
            CPY_CompareMemoryAndY => {}
            DEC_DecrementMemoryByOne => {}
            DEX_DecrementXByOne => {}
            DEY_DecrementYByOne => {}
            EOR_ExclusiveORMemoryWithAcc => {}
            INC_IncrementMemoryByOne => {}
            INX_IncrementXByOne => {}
            INY_IncrementYByOne => {}
            JMP_JumpTo => {}
            JSR_JumpToSavingReturnAddr => {}
            LDA_LoadAccWithMemory => {}
            LDX_LoadXWithMemory => {}
            LDY_LoadYWithMemory => {}
            LSR_ShiftOneBitRight => {}
            NOP_NoOperation => {}
            ORA_ORMemoryWithAcc => {}
            PHA_PushAccOnStack => {}
            PHP_PushProcessorStatusOnStack => {}
            PLA_PullAccFromStack => {}
            PLP_PullProcessorStatusFromStack => {}
            ROL_RotateOneBitLeft => {}
            ROR_RotateOneBitRight => {}
            RTI_ReturnFromInterrupt => {}
            RTS_ReturnFromSubroutine => {}
            SBC_SubtractMemoryFromAccWithBorrow => {}
            SEC_SetCarryFlag => self.set_flag(Flag::Carry, true),
            SED_SetDecimalMode => self.set_flag(Flag::DecimalMode, true),
            SEI_SetInterruptDisableStatus => self.set_flag(Flag::DisableInterrupts, true),
            STA_StoreAccInMemory => {}
            STX_StoreXInMemory => {}
            STY_StoreYInMemory => {}
            TAX_TransferAccToX => {}
            TAY_TransferAccToY => {}
            TSX_TransferStackPointerToX => {}
            TXA_TransferXToAcc => {}
            TXS_TransferXToStackRegister => {}
            TYA_TransferYToAcc => {}
            InvalidInstruction => println!("Invalid instruction!"),
        }
        return 0
    }
}
