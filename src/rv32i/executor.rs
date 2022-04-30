/*
 * Copyright (C) 2022 Jonathan Schild - All Rights Reserved
 */

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