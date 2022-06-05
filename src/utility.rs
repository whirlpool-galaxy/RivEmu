/*
 * Copyright (C) 2022 Jonathan Schild - All Rights Reserved
 */

//! Utility functions to decode instructions.
//!
//! # Authors and Copyright
//! Copyright (C) 2022 Jonathan Schild - All Rights Reserved
//!  
//! - Jonathan Schild

/// Splits an instruction in into an an high, low and instruction prefix part.
///
/// -`inst`: Standard 32-bit instruction or 16-bit compressed instruction. Opcode is located at the least significant bits.
///
/// Returns instruction bits `(inst[6:5], inst[4:2], inst[1:0])`. Values are not sign extended.
pub fn opcode_div(inst: u32) -> (u32, u32, u32) {
    (
        select_bits_unsigned(inst, 5, 2),
        select_bits_unsigned(inst, 2, 3),
        select_bits_unsigned(inst, 0, 2),
    )
}

/// Decodes the `RD` field of a standard 32-bit instruction.
///
/// Returns `inst[11:7]`. Values are not sign extended.
pub fn rd(inst: u32) -> u8 {
    select_bits_unsigned(inst, 7, 5) as u8
}

/// Decodes the `RS1` field of a standard 32-bit instruction.
///
/// Returns `inst[19:15]`. Values are not sign extended.
pub fn rs1(inst: u32) -> u8 {
    select_bits_unsigned(inst, 15, 5) as u8
}

/// Decodes the `RS2` field of a standard 32-bit instruction.
///
/// Returns `inst[24:20]`. Values are not sign extended.
pub fn rs2(inst: u32) -> u8 {
    select_bits_unsigned(inst, 20, 5) as u8
}

/// Decodes the `func3` field of a standard 32-bit instruction.
///
/// Returns `inst[14:12]`. Values are not sign extended.
pub fn func3(inst: u32) -> u16 {
    select_bits_unsigned(inst, 12, 3) as u16
}

/// Decodes the `func7` field of a standard 32-bit instruction.
///
/// Returns `inst[31:25]`. Values are not sign extended.
pub fn func7(inst: u32) -> u16 {
    select_bits_unsigned(inst, 25, 7) as u16
}

/// Concatenates [func3] and [func7].
///
/// Returns `inst[func7].inst[func3]`. Values are not sign extended.
pub fn func10(inst: u32) -> u16 {
    (func7(inst) << 3 | func3(inst)) as u16
}

/// Decodes the `imm` field of a `I-Type` instruction.
/// Values are sign extended.
///
/// See [RISC-V Spec p. 130](https://github.com/riscv/riscv-isa-manual/releases/download/Ratified-IMAFDQC/riscv-spec-20191213.pdf#page.130)
pub fn imm_i(inst: u32) -> i32 {
    select_bits_signed(inst, 20, 12)
}

/// Decodes the `imm` field of a `S-Type` instruction.
/// Values are sign extended.
///
/// See [RISC-V Spec p. 130](https://github.com/riscv/riscv-isa-manual/releases/download/Ratified-IMAFDQC/riscv-spec-20191213.pdf#page.130)
pub fn imm_s(inst: u32) -> i32 {
    select_bits_signed(inst, 25, 7) << 5 | rd(inst) as i32
}

/// Decodes the `imm` field of a `B-Type` instruction.
/// Values are sign extended.
///
/// See [RISC-V Spec p. 130](https://github.com/riscv/riscv-isa-manual/releases/download/Ratified-IMAFDQC/riscv-spec-20191213.pdf#page.130)
pub fn imm_b(inst: u32) -> i32 {
    select_bits_signed(inst, 31, 1) << 12
        | (select_bits_unsigned(inst, 7, 1) << 11) as i32
        | (select_bits_unsigned(inst, 25, 6) << 5) as i32
        | (select_bits_unsigned(inst, 8, 4) << 1) as i32
}

/// Decodes the `imm` field of a `U-Type` instruction.
/// Values are sign extended.
///
/// See [RISC-V Spec p. 130](https://github.com/riscv/riscv-isa-manual/releases/download/Ratified-IMAFDQC/riscv-spec-20191213.pdf#page.130)
pub fn imm_u(inst: u32) -> i32 {
    select_bits_signed(inst, 12, 20) << 12
}

/// Decodes the `imm` field of a `J-Type` instruction.
/// Values are sign extended.
///
/// See [RISC-V Spec p. 130](https://github.com/riscv/riscv-isa-manual/releases/download/Ratified-IMAFDQC/riscv-spec-20191213.pdf#page.130)
pub fn imm_j(inst: u32) -> i32 {
    select_bits_signed(inst, 31, 1) << 20
        | (select_bits_unsigned(inst, 12, 8) << 12) as i32
        | (select_bits_unsigned(inst, 20, 1) << 11) as i32
        | (select_bits_unsigned(inst, 21, 10) << 1) as i32
}

/// Select bits from bit `low_bit` to `low_bit + length`.
/// Resulting integer is signed extended to 32 bit.
pub fn select_bits_signed(instruction: u32, low_bit: u32, length: u32) -> i32 {
    // move the msb to the left
    // move the lsb to the right
    // the right shift sign extends the result
    (instruction as i32) << (32 - low_bit - length) >> (32 - length)
}

/// Select bits from bit `low_bit` to `low_bit + length`.
/// Resulting integer is unsigned extended to 32 bit.
pub fn select_bits_unsigned(instruction: u32, low_bit: u32, length: u32) -> u32 {
    // move lsb of bits you want to select to the right (unsigned) = a
    // move 1 bit to the last position of the bits you want to select = b1
    // subtract 1 from b1 to get the proper bit mask = b2
    // use a bitwise-and on a and b2 to apply the bit mask
    (instruction >> low_bit) & ((1u32 << length) - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_div() {
        assert_eq!(opcode_div(0x55555563), (0b11, 0b000, 0b11))
    }

    #[test]
    fn test_test_rd() {
        assert_eq!(rd(0xFFFFF07F), 0)
    }

    #[test]
    fn test_rs1() {
        assert_eq!(rs1(0xFFF07FFF), 0)
    }

    #[test]
    fn test_rs2() {
        assert_eq!(rs2(0xFE0FFFFF), 0)
    }

    #[test]
    fn test_func3() {
        assert_eq!(func3(0xFFFF8FFF), 0)
    }

    #[test]
    fn test_func7() {
        assert_eq!(func7(0x01FFFFFF), 0)
    }

    #[test]
    fn test_func10() {
        assert_eq!(func10(0x01FF8FFF), 0)
    }

    #[test]
    fn test_imm_i() {
        assert_eq!(imm_i(0x800FFFFF), 0xFFFFF800u32 as i32)
    }

    #[test]
    fn test_imm_s() {}

    #[test]
    fn test_imm_b() {}

    #[test]
    fn test_imm_u() {}

    #[test]
    fn test_imm_j() {
        assert_eq!(imm_j(0xFF5FF06F), 0xFFFFFFF4u32 as i32);
    }

    #[test]
    fn test_select_bits_signed() {}

    #[test]
    fn test_select_bits_unsigned() {}
}
