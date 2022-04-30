/*
 * Copyright (C) 2022 Jonathan Schild - All Rights Reserved
 */

use super::BaseInstruction;
use super::RV32IInterface;
use crate::utility::*;

trait RV32IExecutor {
    fn get_executor(
        opcode: u8,
        func: u16,
        instruction: BaseInstruction,
    ) -> fn(cpu: &mut dyn RV32IInterface, instruction: BaseInstruction);
}

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
