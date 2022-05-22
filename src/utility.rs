/*
 * Copyright (C) 2022 Jonathan Schild - All Rights Reserved
 */

pub fn opcode(inst: u32) -> u8 {
    select_bits_unsigned(inst, 0, 7) as u8
}

pub fn opcode_div(inst: u32) -> (u32, u32, u32) {
    (
        select_bits_unsigned(inst, 5, 2),
        select_bits_unsigned(inst, 2, 3),
        select_bits_unsigned(inst, 0, 2),
    )
}

pub fn rd(inst: u32) -> u8 {
    select_bits_unsigned(inst, 7, 5) as u8
}

pub fn rs1(inst: u32) -> u8 {
    select_bits_unsigned(inst, 15, 5) as u8
}

pub fn rs2(inst: u32) -> u8 {
    select_bits_unsigned(inst, 12, 5) as u8
}

pub fn func3(inst: u32) -> u16 {
    select_bits_unsigned(inst, 12, 3) as u16
}

pub fn func7(inst: u32) -> u16 {
    select_bits_unsigned(inst, 25, 7) as u16
}

pub fn func10(inst: u32) -> u16 {
    (func7(inst) << 3 | func3(inst)) as u16
}

pub fn imm_i(inst: u32) -> i32 {
    select_bits_signed(inst, 20, 12)
}

pub fn imm_s(inst: u32) -> i32 {
    select_bits_signed(inst, 25, 7) << 5 | rd(inst) as i32
}

pub fn imm_b(inst: u32) -> i32 {
    select_bits_signed(inst, 31, 1) << 12
        | (select_bits_unsigned(inst, 7, 1) << 11) as i32
        | (select_bits_unsigned(inst, 25, 6) << 5) as i32
        | (select_bits_unsigned(inst, 8, 4) << 1) as i32
}

pub fn imm_u(inst: u32) -> i32 {
    select_bits_signed(inst, 12, 20) << 12
}

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

/// Returns the length of a bit field.
pub fn bit_length(low: u32, high: u32) -> u32 {
    high - low + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_imm_j() {
        assert_eq!(imm_j(0xFF5FF06F), 0xFFFFFFF4u32 as i32);
    }
}
