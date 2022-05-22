/*
 * Copyright (C) 2022 Jonathan Schild - All Rights Reserved
 */

use super::executor::*;
use super::BaseInstruction;
use super::RV32IInterface;
use crate::utility::*;

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

    fn get_executor(
        &self,
        func: Option<u16>,
    ) -> fn(cpu: &mut dyn RV32IInterface, instruction: BaseInstruction) {
        let mut ret = unknown as fn(cpu: &mut dyn RV32IInterface, instruction: BaseInstruction);
        match self {
            Operation::Compressed => {}
            Operation::Load => {
                let func = match func {
                    Some(x) => x,
                    None => panic!("Invalid function code!"),
                };
                match func {
                    0b000 => ret = lb,
                    0b001 => ret = lh,
                    0b010 => ret = lw,
                    0b100 => ret = lbu,
                    0b101 => ret = lhu,
                    _ => panic!("Invalid function code!"),
                };
            }
            Operation::LoadFp => {}
            Operation::Custom0 => {}
            Operation::MiscMem => {
                // Unknown
            }
            Operation::OpImm => {
                let func = match func {
                    Some(x) => x,
                    None => panic!("Invalid function code!"),
                };
                match func {
                    0b000 => ret = addi,
                    0b010 => ret = slti,
                    0b011 => ret = sltiu,
                    0b100 => ret = xori,
                    0b110 => ret = ori,
                    0b111 => ret = andi,
                    0b0000000001 => ret = slli,
                    0b0000000101 => ret = srli,
                    0b0100000101 => ret = srai,
                    _ => panic!("Invalid function code!"),
                };
            }
            Operation::Auipc => match func {
                Some(_) => panic!("Invalid function code!"),
                None => ret = auipc,
            },
            Operation::OpImm32 => {}
            Operation::Bit48_0 => {}
            Operation::Store => {
                let func = match func {
                    Some(x) => x,
                    None => panic!("Invalid function code!"),
                };
                match func {
                    0b000 => ret = sb,
                    0b001 => ret = sh,
                    0b010 => ret = sw,
                    _ => panic!("Invalid function code!"),
                };
            }
            Operation::StoreFp => {}
            Operation::Custom1 => {}
            Operation::Amo => {}
            Operation::Op => {
                let func = match func {
                    Some(x) => x,
                    None => panic!("Invalid function code!"),
                };
                match func {
                    0b0000000000 => ret = add,
                    0b0100000000 => ret = sub,
                    0b0000000001 => ret = sll,
                    0b0000000010 => ret = slt,
                    0b0000000011 => ret = sltu,
                    0b0000000100 => ret = xor,
                    0b0000000101 => ret = srl,
                    0b0100000101 => ret = sra,
                    0b0000000110 => ret = or,
                    0b0000000111 => ret = and,
                    _ => panic!("Invalid function code!"),
                };
            }
            Operation::Lui => match func {
                Some(_) => panic!("Invalid function code!"),
                None => ret = lui,
            },
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
                let func = match func {
                    Some(x) => x,
                    None => panic!("Invalid function code!"),
                };
                match func {
                    0b000 => ret = beq,
                    0b001 => ret = bne,
                    0b100 => ret = blt,
                    0b101 => ret = bge,
                    0b110 => ret = bltu,
                    0b111 => ret = bgeu,
                    _ => panic!("Invalid function code!"),
                };
            }
            Operation::Jalr => {
                let func = match func {
                    Some(x) => x,
                    None => panic!("Invalid function code!"),
                };
                match func {
                    0b000 => ret = jalr,
                    _ => panic!("Invalid function code!"),
                };
            }
            Operation::Reserved1 => {}
            Operation::Jal => match func {
                Some(_) => panic!("Invalid function code!"),
                None => ret = jal,
            },
            Operation::System => {
                // Unknown
            }
            Operation::Reserved2 => {}
            Operation::Custom3 => {}
            Operation::Bit80 => {}
        };
        ret
    }
}

pub fn decode(
    instruction: u32,
) -> (
    BaseInstruction,
    fn(cpu: &mut dyn RV32IInterface, instruction: BaseInstruction),
) {
    let (op_hi, op_low, op_comp) = opcode_div(instruction);
    if op_comp == 0x11 {
        let operation = &OPCODE_DECODING_TABLE[op_hi as usize][op_low as usize];
        let (func, instr) = operation.decode(instruction);
        (instr, operation.get_executor(func))
    } else {
        (BaseInstruction::Unknown { instruction }, unknown)
    }
}
