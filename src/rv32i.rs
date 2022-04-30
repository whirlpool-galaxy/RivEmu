/*
 * Copyright (C) 2022 Jonathan Schild - All Rights Reserved
 */

use std::panic;

enum BaseInstruction {
    RType { rd: u8, rs1: u8, rs2: u8 },
    IType { rd: u8, rs1: u8, imm: i32 },
    SBType { rs1: u8, rs2: u8, imm: i32 },
    UJType { rd: u8, imm: i32 },
    Unknown { instruction: u32 },
}

trait RV32IBus {
    fn load_byte(&mut self, address: u32) -> u8;
    fn load_half_word(&mut self, address: u32) -> u16;
    fn load_word(&mut self, address: u32) -> u32;

    fn store_byte(&mut self, address: u32, data: u8);
    fn store_half_word(&mut self, address: u32, data: u16);
    fn store_word(&mut self, address: u32, data: u32);
}

trait RV32IInterface: RV32IBus {
    fn read_register(&self, register: u8) -> u32;
    fn write_register(&mut self, register: u8, data: u32);

    fn read_pc(&self) -> u32;
    fn write_pc(&mut self, pc: u32);
}

struct RV32ICPU {
    register: [u32; 32],
    pc: u32,
}

impl RV32IBus for RV32ICPU {
    fn load_byte(&mut self, address: u32) -> u8 {
        0
    }
    fn load_half_word(&mut self, address: u32) -> u16 {
        0
    }
    fn load_word(&mut self, address: u32) -> u32 {
        0
    }

    fn store_byte(&mut self, address: u32, data: u8) {}
    fn store_half_word(&mut self, address: u32, data: u16) {}
    fn store_word(&mut self, address: u32, data: u32) {}
}

impl RV32IInterface for RV32ICPU {
    fn read_register(&self, register: u8) -> u32 {
        if register > 31 {
            panic!(
                "Unknown register address: {:#010b}! Maybe an instruction decoding error. Only bits [0, 4] are valid.",
                register
            );
        }
        if register == 0 {
            0
        } else {
            self.register[register as usize]
        }
    }

    fn write_register(&mut self, register: u8, data: u32) {
        if register > 31 {
            panic!(
                "Unknown register address: {:#010b}! Maybe an instruction decoding error. Only bits [0, 4] are valid.",
                register
            );
        }
        self.register[register as usize] = data;
    }

    fn read_pc(&self) -> u32 {
        self.pc
    }

    fn write_pc(&mut self, pc: u32) {
        self.pc = pc;
    }
}

mod rv32i_decoder {
    use super::BaseInstruction;
    use utility::*;

    enum Operation {
        Compressed,
        Load,
        LoadFp,
        Custom0,
        MiscMem,
        OpImm,
        Auipc,
        OpImm32,
        Bit48_0,
        Store,
        StoreFp,
        Custom1,
        Amo,
        Op,
        Lui,
        Op32,
        Bit64,
        Madd,
        Msub,
        Nmsub,
        Nmadd,
        OpFp,
        Reserved0,
        Custom2,
        Bit48_1,
        Branch,
        Jalr,
        Reserved1,
        Jal,
        System,
        Reserved2,
        Custom3,
        Bit80,
    }

    impl Operation {
        fn decode(&self, instruction: u32) -> (Option<u16>, BaseInstruction) {
            let mut func = Option::None;
            let mut instr = BaseInstruction::Unknown { instruction };
            match self {
                Operation::Compressed => {}
                Operation::Load => {
                    func = Option::Some(func3(instruction));
                    instr = BaseInstruction::IType {
                        rd: rd(instruction),
                        rs1: rs1(instruction),
                        imm: imm_i(instruction),
                    }
                }
                Operation::LoadFp => {}
                Operation::Custom0 => {}
                Operation::MiscMem => {
                    // Unknown
                }
                Operation::OpImm => {
                    func = Option::Some(func3(instruction));
                    if func == Option::Some(0b001)
                        || func == Option::Some(0b001)
                        || func == Option::Some(0b001)
                    {
                        func = Option::Some(func10(instruction));
                        instr = BaseInstruction::RType {
                            rd: rd(instruction),
                            rs1: rs1(instruction),
                            rs2: rs2(instruction),
                        }
                    } else {
                        instr = BaseInstruction::IType {
                            rd: rd(instruction),
                            rs1: rs1(instruction),
                            imm: imm_i(instruction),
                        };
                    }
                }
                Operation::Auipc => {
                    instr = BaseInstruction::UJType {
                        rd: rd(instruction),
                        imm: imm_u(instruction),
                    }
                }
                Operation::OpImm32 => {}
                Operation::Bit48_0 => {}
                Operation::Store => {
                    func = Option::Some(func3(instruction));
                    instr = BaseInstruction::SBType {
                        rs1: rs1(instruction),
                        rs2: rs2(instruction),
                        imm: imm_s(instruction),
                    }
                }
                Operation::StoreFp => {}
                Operation::Custom1 => {}
                Operation::Amo => {}
                Operation::Op => {
                    func = Option::Some(func10(instruction));
                    instr = BaseInstruction::RType {
                        rd: rd(instruction),
                        rs1: rs1(instruction),
                        rs2: rs2(instruction),
                    }
                }
                Operation::Lui => {
                    instr = BaseInstruction::UJType {
                        rd: rd(instruction),
                        imm: imm_u(instruction),
                    }
                }
                Operation::Op32 => {}
                Operation::Bit64 => {}
                Operation::Madd => {}
                Operation::Msub => {}
                Operation::Nmsub => {}
                Operation::Nmadd => {}
                Operation::OpFp => {}
                Operation::Reserved0 => {}
                Operation::Custom2 => {}
                Operation::Bit48_1 => {}
                Operation::Branch => {
                    func = Option::Some(func3(instruction));
                    instr = BaseInstruction::SBType {
                        rs1: rs1(instruction),
                        rs2: rs2(instruction),
                        imm: imm_b(instruction),
                    }
                }
                Operation::Jalr => {
                    if func3(instruction) == 0 {
                        instr = BaseInstruction::IType {
                            rd: rd(instruction),
                            rs1: rs1(instruction),
                            imm: imm_u(instruction),
                        }
                    }
                }
                Operation::Reserved1 => {}
                Operation::Jal => {
                    instr = BaseInstruction::UJType {
                        rd: rd(instruction),
                        imm: imm_j(instruction),
                    }
                }
                Operation::System => {
                    // Unknown
                }
                Operation::Reserved2 => {}
                Operation::Custom3 => {}
                Operation::Bit80 => {}
            };
            (func, instr)
        }
    }

    /**
     * Table containing opcode types indexed by bit [`[6:5]`, `[4:2]`].
     */
    const OPCODE_DECODING_TABLE: [[Operation; 8]; 4] = [
        [
            Operation::Load,
            Operation::LoadFp,
            Operation::Custom0,
            Operation::MiscMem,
            Operation::OpImm,
            Operation::Auipc,
            Operation::OpImm32,
            Operation::Bit48_0,
        ],
        [
            Operation::Store,
            Operation::StoreFp,
            Operation::Custom1,
            Operation::Amo,
            Operation::Op,
            Operation::Lui,
            Operation::Op32,
            Operation::Bit64,
        ],
        [
            Operation::Madd,
            Operation::Msub,
            Operation::Nmsub,
            Operation::Nmadd,
            Operation::OpFp,
            Operation::Reserved0,
            Operation::Custom2,
            Operation::Bit48_1,
        ],
        [
            Operation::Branch,
            Operation::Jalr,
            Operation::Reserved1,
            Operation::Jal,
            Operation::System,
            Operation::Reserved2,
            Operation::Custom3,
            Operation::Bit80,
        ],
    ];

    mod utility {
        pub fn opcode(inst: u32) -> u32 {
            select_bits_unsigned(inst, 0, 7)
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
                | (select_bits_unsigned(inst, 19, 1) << 11) as i32
                | (select_bits_unsigned(inst, 20, 10) << 1) as i32
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
    }
}

mod rv32i_executor {

    use super::*;

    fn addi(cpu: &mut impl RV32IInterface, instruction: BaseInstruction) {
        match instruction {
            BaseInstruction::IType { rd, rs1, imm } => {
                let operand = cpu.read_register(rs1) as i32;
                cpu.write_register(rd, (imm.wrapping_add(operand)) as u32)
            }
            _ => {
                panic!("Invalid instruction type!")
            }
        }
    }

    fn slti(cpu: &mut impl RV32IInterface, instruction: BaseInstruction) {
        match instruction {
            BaseInstruction::IType { rd, rs1, imm } => {
                let operand = cpu.read_register(rs1) as i32;
                if operand < imm {
                    cpu.write_register(rd, 1);
                } else {
                    cpu.write_register(rd, 0);
                }
            }
            _ => {
                panic!("Invalid instruction type!")
            }
        }
    }

    fn sltiu(cpu: &mut impl RV32IInterface, instruction: BaseInstruction) {
        match instruction {
            BaseInstruction::IType { rd, rs1, imm } => {
                let imm = imm as u32;
                let operand = cpu.read_register(rs1);
                if operand < imm {
                    cpu.write_register(rd, 1);
                } else {
                    cpu.write_register(rd, 0);
                }
            }
            _ => {
                panic!("Invalid instruction type!")
            }
        }
    }

    fn andi(cpu: &mut impl RV32IInterface, instruction: BaseInstruction) {
        match instruction {
            BaseInstruction::IType { rd, rs1, imm } => {
                let imm = imm as u32;
                let operand = cpu.read_register(rs1);
                cpu.write_register(rd, imm & operand);
            }
            _ => {
                panic!("Invalid instruction type!")
            }
        }
    }

    fn ori(cpu: &mut impl RV32IInterface, instruction: BaseInstruction) {
        match instruction {
            BaseInstruction::IType { rd, rs1, imm } => {
                let imm = imm as u32;
                let operand = cpu.read_register(rs1);
                cpu.write_register(rd, imm | operand);
            }
            _ => {
                panic!("Invalid instruction type!")
            }
        }
    }

    fn xori(cpu: &mut impl RV32IInterface, instruction: BaseInstruction) {
        match instruction {
            BaseInstruction::IType { rd, rs1, imm } => {
                let imm = imm as u32;
                let operand = cpu.read_register(rs1);
                cpu.write_register(rd, imm ^ operand);
            }
            _ => {
                panic!("Invalid instruction type!")
            }
        }
    }

    /**
     * WARNING: Expect RType instead of in RISC-V Spec specified IType!
     */
    fn slli(cpu: &mut impl RV32IInterface, instruction: BaseInstruction) {
        match instruction {
            // Is actually IType
            BaseInstruction::RType { rd, rs1, rs2 } => {
                let shamt = rs2 as u32;
                let operand = cpu.read_register(rs1);
                cpu.write_register(rd, operand << shamt);
            }
            _ => {
                panic!("Invalid instruction type!")
            }
        }
    }

    /**
     * WARNING: Expect RType instead of in RISC-V Spec specified IType!
     */
    fn srli(cpu: &mut impl RV32IInterface, instruction: BaseInstruction) {
        match instruction {
            // Is actually IType
            BaseInstruction::RType { rd, rs1, rs2 } => {
                let shamt = rs2 as u32;
                let operand = cpu.read_register(rs1);
                cpu.write_register(rd, operand >> shamt);
            }
            _ => {
                panic!("Invalid instruction type!")
            }
        }
    }

    /**
     * WARNING: Expect RType instead of in RISC-V Spec specified IType!
     */
    fn srai(cpu: &mut impl RV32IInterface, instruction: BaseInstruction) {
        match instruction {
            // Is actually IType
            BaseInstruction::RType { rd, rs1, rs2 } => {
                let shamt = rs2;
                let operand = cpu.read_register(rs1) as i32;
                cpu.write_register(rd, (operand >> shamt) as u32);
            }
            _ => {
                panic!("Invalid instruction type!")
            }
        }
    }

    fn lui(cpu: &mut impl RV32IInterface, instruction: BaseInstruction) {
        match instruction {
            BaseInstruction::UJType { rd, imm } => {
                cpu.write_register(rd, imm as u32);
            }
            _ => {
                panic!("Invalid instruction type!")
            }
        }
    }

    fn auipc(cpu: &mut impl RV32IInterface, instruction: BaseInstruction) {
        match instruction {
            BaseInstruction::UJType { rd, imm } => {
                let pc = cpu.read_pc();
                cpu.write_register(rd, pc.wrapping_add(imm as u32));
            }
            _ => {
                panic!("Invalid instruction type!")
            }
        }
    }

    fn add(cpu: &mut impl RV32IInterface, instruction: BaseInstruction) {
        match instruction {
            BaseInstruction::RType { rd, rs1, rs2 } => {
                let lhs = cpu.read_register(rs1);
                let rhs = cpu.read_register(rs2);
                let result = lhs.wrapping_add(rhs);
                cpu.write_register(rd, result);
            }
            _ => {
                panic!("Invalid instruction type!")
            }
        }
    }

    fn slt(cpu: &mut impl RV32IInterface, instruction: BaseInstruction) {
        match instruction {
            BaseInstruction::RType { rd, rs1, rs2 } => {
                let lhs = cpu.read_register(rs1) as i32;
                let rhs = cpu.read_register(rs2) as i32;
                if lhs < rhs {
                    cpu.write_register(rd, 1);
                } else {
                    cpu.write_register(rd, 0)
                }
            }
            _ => {
                panic!("Invalid instruction type!")
            }
        }
    }

    fn sltu(cpu: &mut impl RV32IInterface, instruction: BaseInstruction) {
        match instruction {
            BaseInstruction::RType { rd, rs1, rs2 } => {
                let lhs = cpu.read_register(rs1);
                let rhs = cpu.read_register(rs2);
                if lhs < rhs {
                    cpu.write_register(rd, 1);
                } else {
                    cpu.write_register(rd, 0)
                }
            }
            _ => {
                panic!("Invalid instruction type!")
            }
        }
    }

    fn and(cpu: &mut impl RV32IInterface, instruction: BaseInstruction) {
        match instruction {
            BaseInstruction::RType { rd, rs1, rs2 } => {
                let lhs = cpu.read_register(rs1);
                let rhs = cpu.read_register(rs2);
                let result = lhs & rhs;
                cpu.write_register(rd, result);
            }
            _ => {
                panic!("Invalid instruction type!")
            }
        }
    }

    fn or(cpu: &mut impl RV32IInterface, instruction: BaseInstruction) {
        match instruction {
            BaseInstruction::RType { rd, rs1, rs2 } => {
                let lhs = cpu.read_register(rs1);
                let rhs = cpu.read_register(rs2);
                let result = lhs | rhs;
                cpu.write_register(rd, result);
            }
            _ => {
                panic!("Invalid instruction type!")
            }
        }
    }

    fn xor(cpu: &mut impl RV32IInterface, instruction: BaseInstruction) {
        match instruction {
            BaseInstruction::RType { rd, rs1, rs2 } => {
                let lhs = cpu.read_register(rs1);
                let rhs = cpu.read_register(rs2);
                let result = lhs ^ rhs;
                cpu.write_register(rd, result);
            }
            _ => {
                panic!("Invalid instruction type!")
            }
        }
    }

    fn sll(cpu: &mut impl RV32IInterface, instruction: BaseInstruction) {
        match instruction {
            BaseInstruction::RType { rd, rs1, rs2 } => {
                let lhs = cpu.read_register(rs1);
                let rhs = cpu.read_register(rs2);
                let result = lhs << rhs;
                cpu.write_register(rd, result);
            }
            _ => {
                panic!("Invalid instruction type!")
            }
        }
    }

    fn srl(cpu: &mut impl RV32IInterface, instruction: BaseInstruction) {
        match instruction {
            BaseInstruction::RType { rd, rs1, rs2 } => {
                let lhs = cpu.read_register(rs1);
                let rhs = cpu.read_register(rs2);
                let result = lhs >> rhs;
                cpu.write_register(rd, result);
            }
            _ => {
                panic!("Invalid instruction type!")
            }
        }
    }

    fn sub(cpu: &mut impl RV32IInterface, instruction: BaseInstruction) {
        match instruction {
            BaseInstruction::RType { rd, rs1, rs2 } => {
                let lhs = cpu.read_register(rs1);
                let rhs = cpu.read_register(rs2);
                let result = lhs.wrapping_sub(rhs);
                cpu.write_register(rd, result);
            }
            _ => {
                panic!("Invalid instruction type!")
            }
        }
    }

    fn sra(cpu: &mut impl RV32IInterface, instruction: BaseInstruction) {
        match instruction {
            BaseInstruction::RType { rd, rs1, rs2 } => {
                let lhs = cpu.read_register(rs1) as i32;
                let rhs = cpu.read_register(rs2);
                let result = lhs >> rhs;
                cpu.write_register(rd, result as u32);
            }
            _ => {
                panic!("Invalid instruction type!")
            }
        }
    }

    fn jal(cpu: &mut impl RV32IInterface, instruction: BaseInstruction) {
        match instruction {
            BaseInstruction::UJType { rd, imm } => {
                let pc = cpu.read_pc();
                cpu.write_register(rd, pc.wrapping_add(4));
                cpu.write_pc(pc.wrapping_add(imm as u32));
            }
            _ => {
                panic!("Invalid instruction type!")
            }
        }
    }

    fn jalr(cpu: &mut impl RV32IInterface, instruction: BaseInstruction) {
        match instruction {
            BaseInstruction::IType { rd, rs1, imm } => {
                let pc = cpu.read_pc();
                cpu.write_register(rd, pc.wrapping_add(4));
                let new_pc = cpu.read_register(rs1);
                cpu.write_pc(new_pc.wrapping_add(imm as u32) & 0xFFFFFFFE);
            }
            _ => {
                panic!("Invalid instruction type!")
            }
        }
    }

    fn beq(cpu: &mut impl RV32IInterface, instruction: BaseInstruction) {
        match instruction {
            BaseInstruction::SBType { rs1, rs2, imm } => {
                let lhs = cpu.read_register(rs1);
                let rhs = cpu.read_register(rs2);
                if lhs == rhs {
                    let pc = cpu.read_pc();
                    cpu.write_pc(pc.wrapping_add(imm as u32));
                }
            }
            _ => {
                panic!("Invalid instruction type!")
            }
        }
    }

    fn bne(cpu: &mut impl RV32IInterface, instruction: BaseInstruction) {
        match instruction {
            BaseInstruction::SBType { rs1, rs2, imm } => {
                let lhs = cpu.read_register(rs1);
                let rhs = cpu.read_register(rs2);
                if lhs != rhs {
                    let pc = cpu.read_pc();
                    cpu.write_pc(pc.wrapping_add(imm as u32));
                }
            }
            _ => {
                panic!("Invalid instruction type!")
            }
        }
    }

    fn blt(cpu: &mut impl RV32IInterface, instruction: BaseInstruction) {
        match instruction {
            BaseInstruction::SBType { rs1, rs2, imm } => {
                let lhs = cpu.read_register(rs1) as i32;
                let rhs = cpu.read_register(rs2) as i32;
                if lhs < rhs {
                    let pc = cpu.read_pc();
                    cpu.write_pc(pc.wrapping_add(imm as u32));
                }
            }
            _ => {
                panic!("Invalid instruction type!")
            }
        }
    }

    fn bltu(cpu: &mut impl RV32IInterface, instruction: BaseInstruction) {
        match instruction {
            BaseInstruction::SBType { rs1, rs2, imm } => {
                let lhs = cpu.read_register(rs1);
                let rhs = cpu.read_register(rs2);
                if lhs < rhs {
                    let pc = cpu.read_pc();
                    cpu.write_pc(pc.wrapping_add(imm as u32));
                }
            }
            _ => {
                panic!("Invalid instruction type!")
            }
        }
    }

    fn bge(cpu: &mut impl RV32IInterface, instruction: BaseInstruction) {
        match instruction {
            BaseInstruction::SBType { rs1, rs2, imm } => {
                let lhs = cpu.read_register(rs1) as i32;
                let rhs = cpu.read_register(rs2) as i32;
                if lhs >= rhs {
                    let pc = cpu.read_pc();
                    cpu.write_pc(pc.wrapping_add(imm as u32));
                }
            }
            _ => {
                panic!("Invalid instruction type!")
            }
        }
    }

    fn bgeu(cpu: &mut impl RV32IInterface, instruction: BaseInstruction) {
        match instruction {
            BaseInstruction::SBType { rs1, rs2, imm } => {
                let lhs = cpu.read_register(rs1);
                let rhs = cpu.read_register(rs2);
                if lhs >= rhs {
                    let pc = cpu.read_pc();
                    cpu.write_pc(pc.wrapping_add(imm as u32));
                }
            }
            _ => {
                panic!("Invalid instruction type!")
            }
        }
    }

    fn lb(cpu: &mut impl RV32IInterface, instruction: BaseInstruction) {
        match instruction {
            BaseInstruction::IType { rd, rs1, imm } => {
                let base = cpu.read_register(rs1);
                let address = base.wrapping_add(imm as u32);
                let data = ((cpu.load_byte(address) as i8) as i32) as u32;

                cpu.write_register(rd, data)
            }
            _ => {
                panic!("Invalid instruction type!")
            }
        }
    }

    fn lh(cpu: &mut impl RV32IInterface, instruction: BaseInstruction) {
        match instruction {
            BaseInstruction::IType { rd, rs1, imm } => {
                let base = cpu.read_register(rs1);
                let address = base.wrapping_add(imm as u32);
                let data = ((cpu.load_half_word(address) as i16) as i32) as u32;

                cpu.write_register(rd, data)
            }
            _ => {
                panic!("Invalid instruction type!")
            }
        }
    }

    fn lw(cpu: &mut impl RV32IInterface, instruction: BaseInstruction) {
        match instruction {
            BaseInstruction::IType { rd, rs1, imm } => {
                let base = cpu.read_register(rs1);
                let address = base.wrapping_add(imm as u32);
                let data = cpu.load_word(address);

                cpu.write_register(rd, data)
            }
            _ => {
                panic!("Invalid instruction type!")
            }
        }
    }

    fn lbu(cpu: &mut impl RV32IInterface, instruction: BaseInstruction) {
        match instruction {
            BaseInstruction::IType { rd, rs1, imm } => {
                let base = cpu.read_register(rs1);
                let address = base.wrapping_add(imm as u32);
                let data = cpu.load_byte(address) as u32;

                cpu.write_register(rd, data)
            }
            _ => {
                panic!("Invalid instruction type!")
            }
        }
    }

    fn lhu(cpu: &mut impl RV32IInterface, instruction: BaseInstruction) {
        match instruction {
            BaseInstruction::IType { rd, rs1, imm } => {
                let base = cpu.read_register(rs1);
                let address = base.wrapping_add(imm as u32);
                let data = cpu.load_half_word(address) as u32;

                cpu.write_register(rd, data)
            }
            _ => {
                panic!("Invalid instruction type!")
            }
        }
    }

    fn sb(cpu: &mut impl RV32IInterface, instruction: BaseInstruction) {
        match instruction {
            BaseInstruction::SBType { rs1, rs2, imm } => {
                let base = cpu.read_register(rs1);
                let address = base.wrapping_add(imm as u32);
                let data = cpu.read_register(rs2);

                cpu.store_byte(address, data as u8);
            }
            _ => {
                panic!("Invalid instruction type!")
            }
        }
    }

    fn sh(cpu: &mut impl RV32IInterface, instruction: BaseInstruction) {
        match instruction {
            BaseInstruction::SBType { rs1, rs2, imm } => {
                let base = cpu.read_register(rs1);
                let address = base.wrapping_add(imm as u32);
                let data = cpu.read_register(rs2);

                cpu.store_half_word(address, data as u16);
            }
            _ => {
                panic!("Invalid instruction type!")
            }
        }
    }

    fn sw(cpu: &mut impl RV32IInterface, instruction: BaseInstruction) {
        match instruction {
            BaseInstruction::SBType { rs1, rs2, imm } => {
                let base = cpu.read_register(rs1);
                let address = base.wrapping_add(imm as u32);
                let data = cpu.read_register(rs2);

                cpu.store_word(address, data);
            }
            _ => {
                panic!("Invalid instruction type!")
            }
        }
    }

    fn fence(cpu: &mut impl RV32IInterface, instruction: BaseInstruction) {}

    fn ecall(cpu: &mut impl RV32IInterface, instruction: BaseInstruction) {}

    fn ebreak(cpu: &mut impl RV32IInterface, instruction: BaseInstruction) {}
}
