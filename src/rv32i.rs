/*
 * Copyright (C) 2022 Jonathan Schild - All Rights Reserved
 */

use std::panic;

enum BaseInstruction {
    RType { rd: u8, rs1: u8, rs2: u8 },
    IType { rd: u8, rs1: u8, imm: i32 },
    SBType { rs1: u8, rs2: u8, imm: i32 },
    UJType { rd: u8, imm: i32 },
}

trait RV32IBus {}

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

impl RV32IBus for RV32ICPU {}

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

fn addi(cpu: &mut impl RV32IInterface, instruction: BaseInstruction) {
    match instruction {
        BaseInstruction::IType { rd, rs1, imm } => {
            let operand = cpu.read_register(rs1) as i32;
            cpu.write_register(rd, (imm.wrapping_add(operand)) as u32)
        }
        _ => return,
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
        _ => return,
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
        _ => return,
    }
}

fn andi(cpu: &mut impl RV32IInterface, instruction: BaseInstruction) {
    match instruction {
        BaseInstruction::IType { rd, rs1, imm } => {
            let imm = imm as u32;
            let operand = cpu.read_register(rs1);
            cpu.write_register(rd, imm & operand);
        }
        _ => return,
    }
}

fn ori(cpu: &mut impl RV32IInterface, instruction: BaseInstruction) {
    match instruction {
        BaseInstruction::IType { rd, rs1, imm } => {
            let imm = imm as u32;
            let operand = cpu.read_register(rs1);
            cpu.write_register(rd, imm | operand);
        }
        _ => return,
    }
}

fn xori(cpu: &mut impl RV32IInterface, instruction: BaseInstruction) {
    match instruction {
        BaseInstruction::IType { rd, rs1, imm } => {
            let imm = imm as u32;
            let operand = cpu.read_register(rs1);
            cpu.write_register(rd, imm ^ operand);
        }
        _ => return,
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
        _ => return,
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
        _ => return,
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
        _ => return,
    }
}

fn lui(cpu: &mut impl RV32IInterface, instruction: BaseInstruction) {
    match instruction {
        BaseInstruction::UJType { rd, imm } => {
            cpu.write_register(rd, imm as u32);
        }
        _ => return,
    }
}

fn auipc(cpu: &mut impl RV32IInterface, instruction: BaseInstruction) {
    match instruction {
        BaseInstruction::UJType { rd, imm } => {
            let pc = cpu.read_pc();
            cpu.write_register(rd, pc.wrapping_add(imm as u32));
        }
        _ => return,
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
        _ => return,
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
        _ => return,
    }
}

fn sltu(cpu: &mut impl RV32IInterface, instruction: BaseInstruction) {
    match instruction {
        BaseInstruction::RType { rd, rs1, rs2 } => {
            let lhs = cpu.read_register(rs1);
            let rhs = cpu.read_register(rs2);
            let result = lhs.wrapping_add(rhs);
            if lhs < rhs {
                cpu.write_register(rd, 1);
            } else {
                cpu.write_register(rd, 0)
            }
        }
        _ => return,
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
        _ => return,
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
        _ => return,
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
        _ => return,
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
        _ => return,
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
        _ => return,
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
        _ => return,
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
        _ => return,
    }
}
