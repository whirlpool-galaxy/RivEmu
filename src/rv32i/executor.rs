/*
 * Copyright (C) 2022 Jonathan Schild - All Rights Reserved
 */

use super::*;

pub fn unknown(_cpu: &mut dyn RV32IInterface, instruction: BaseInstruction) {
    match instruction {
        BaseInstruction::Unknown { instruction } => {
            panic!("Unknown instruction: 0x{:08x}", instruction)
        }
        _ => panic!("Invalid instruction type!"),
    };
}

pub fn addi(cpu: &mut dyn RV32IInterface, instruction: BaseInstruction) {
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

pub fn slti(cpu: &mut dyn RV32IInterface, instruction: BaseInstruction) {
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

pub fn sltiu(cpu: &mut dyn RV32IInterface, instruction: BaseInstruction) {
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

pub fn andi(cpu: &mut dyn RV32IInterface, instruction: BaseInstruction) {
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

pub fn ori(cpu: &mut dyn RV32IInterface, instruction: BaseInstruction) {
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

pub fn xori(cpu: &mut dyn RV32IInterface, instruction: BaseInstruction) {
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
 * WARNING: Expects RType instead of in RISC-V Spec specified IType!
 */
pub fn slli(cpu: &mut dyn RV32IInterface, instruction: BaseInstruction) {
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
 * WARNING: Expects RType instead of in RISC-V Spec specified IType!
 */
pub fn srli(cpu: &mut dyn RV32IInterface, instruction: BaseInstruction) {
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
 * WARNING: Expects RType instead of in RISC-V Spec specified IType!
 */
pub fn srai(cpu: &mut dyn RV32IInterface, instruction: BaseInstruction) {
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

pub fn lui(cpu: &mut dyn RV32IInterface, instruction: BaseInstruction) {
    match instruction {
        BaseInstruction::UJType { rd, imm } => {
            cpu.write_register(rd, imm as u32);
        }
        _ => {
            panic!("Invalid instruction type!")
        }
    }
}

pub fn auipc(cpu: &mut dyn RV32IInterface, instruction: BaseInstruction) {
    match instruction {
        BaseInstruction::UJType { rd, imm } => {
            let pc = cpu.read_pc().wrapping_sub(4);
            cpu.write_register(rd, pc.wrapping_add(imm as u32));
        }
        _ => {
            panic!("Invalid instruction type!")
        }
    }
}

pub fn add(cpu: &mut dyn RV32IInterface, instruction: BaseInstruction) {
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

pub fn slt(cpu: &mut dyn RV32IInterface, instruction: BaseInstruction) {
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

pub fn sltu(cpu: &mut dyn RV32IInterface, instruction: BaseInstruction) {
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

pub fn and(cpu: &mut dyn RV32IInterface, instruction: BaseInstruction) {
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

pub fn or(cpu: &mut dyn RV32IInterface, instruction: BaseInstruction) {
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

pub fn xor(cpu: &mut dyn RV32IInterface, instruction: BaseInstruction) {
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

pub fn sll(cpu: &mut dyn RV32IInterface, instruction: BaseInstruction) {
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

pub fn srl(cpu: &mut dyn RV32IInterface, instruction: BaseInstruction) {
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

pub fn sub(cpu: &mut dyn RV32IInterface, instruction: BaseInstruction) {
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

pub fn sra(cpu: &mut dyn RV32IInterface, instruction: BaseInstruction) {
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

pub fn jal(cpu: &mut dyn RV32IInterface, instruction: BaseInstruction) {
    match instruction {
        BaseInstruction::UJType { rd, imm } => {
            let mut pc = cpu.read_pc();
            cpu.write_register(rd, pc);
            pc = pc.wrapping_sub(4);
            cpu.write_pc(pc.wrapping_add(imm as u32));
        }
        _ => {
            panic!("Invalid instruction type!")
        }
    }
}

pub fn jalr(cpu: &mut dyn RV32IInterface, instruction: BaseInstruction) {
    match instruction {
        BaseInstruction::IType { rd, rs1, imm } => {
            let pc = cpu.read_pc();
            cpu.write_register(rd, pc);
            // pc = pc.wrapping_sub(4);
            let new_pc = cpu.read_register(rs1);
            cpu.write_pc(new_pc.wrapping_add(imm as u32) & 0xFFFFFFFE);
        }
        _ => {
            panic!("Invalid instruction type!")
        }
    }
}

pub fn beq(cpu: &mut dyn RV32IInterface, instruction: BaseInstruction) {
    match instruction {
        BaseInstruction::SBType { rs1, rs2, imm } => {
            let lhs = cpu.read_register(rs1);
            let rhs = cpu.read_register(rs2);
            if lhs == rhs {
                let pc = cpu.read_pc().wrapping_sub(4);
                cpu.write_pc(pc.wrapping_add(imm as u32));
            }
        }
        _ => {
            panic!("Invalid instruction type!")
        }
    }
}

pub fn bne(cpu: &mut dyn RV32IInterface, instruction: BaseInstruction) {
    match instruction {
        BaseInstruction::SBType { rs1, rs2, imm } => {
            let lhs = cpu.read_register(rs1);
            let rhs = cpu.read_register(rs2);
            if lhs != rhs {
                let pc = cpu.read_pc().wrapping_sub(4);
                cpu.write_pc(pc.wrapping_add(imm as u32));
            }
        }
        _ => {
            panic!("Invalid instruction type!")
        }
    }
}

pub fn blt(cpu: &mut dyn RV32IInterface, instruction: BaseInstruction) {
    match instruction {
        BaseInstruction::SBType { rs1, rs2, imm } => {
            let lhs = cpu.read_register(rs1) as i32;
            let rhs = cpu.read_register(rs2) as i32;
            if lhs < rhs {
                let pc = cpu.read_pc().wrapping_sub(4);
                cpu.write_pc(pc.wrapping_add(imm as u32));
            }
        }
        _ => {
            panic!("Invalid instruction type!")
        }
    }
}

pub fn bltu(cpu: &mut dyn RV32IInterface, instruction: BaseInstruction) {
    match instruction {
        BaseInstruction::SBType { rs1, rs2, imm } => {
            let lhs = cpu.read_register(rs1);
            let rhs = cpu.read_register(rs2);
            if lhs < rhs {
                let pc = cpu.read_pc().wrapping_sub(4);
                cpu.write_pc(pc.wrapping_add(imm as u32));
            }
        }
        _ => {
            panic!("Invalid instruction type!")
        }
    }
}

pub fn bge(cpu: &mut dyn RV32IInterface, instruction: BaseInstruction) {
    match instruction {
        BaseInstruction::SBType { rs1, rs2, imm } => {
            let lhs = cpu.read_register(rs1) as i32;
            let rhs = cpu.read_register(rs2) as i32;
            if lhs >= rhs {
                let pc = cpu.read_pc().wrapping_sub(4);
                cpu.write_pc(pc.wrapping_add(imm as u32));
            }
        }
        _ => {
            panic!("Invalid instruction type!")
        }
    }
}

pub fn bgeu(cpu: &mut dyn RV32IInterface, instruction: BaseInstruction) {
    match instruction {
        BaseInstruction::SBType { rs1, rs2, imm } => {
            let lhs = cpu.read_register(rs1);
            let rhs = cpu.read_register(rs2);
            if lhs >= rhs {
                let pc = cpu.read_pc().wrapping_sub(4);
                cpu.write_pc(pc.wrapping_add(imm as u32));
            }
        }
        _ => {
            panic!("Invalid instruction type!")
        }
    }
}

pub fn lb(cpu: &mut dyn RV32IInterface, instruction: BaseInstruction) {
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

pub fn lh(cpu: &mut dyn RV32IInterface, instruction: BaseInstruction) {
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

pub fn lw(cpu: &mut dyn RV32IInterface, instruction: BaseInstruction) {
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

pub fn lbu(cpu: &mut dyn RV32IInterface, instruction: BaseInstruction) {
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

pub fn lhu(cpu: &mut dyn RV32IInterface, instruction: BaseInstruction) {
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

pub fn sb(cpu: &mut dyn RV32IInterface, instruction: BaseInstruction) {
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

pub fn sh(cpu: &mut dyn RV32IInterface, instruction: BaseInstruction) {
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

pub fn sw(cpu: &mut dyn RV32IInterface, instruction: BaseInstruction) {
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

// pub fn fence(cpu: &mut dyn RV32IInterface, instruction: BaseInstruction) {}

pub fn eenv(cpu: &mut dyn RV32IInterface, instruction: BaseInstruction) {
    match instruction {
        BaseInstruction::IType { imm, .. } => {
            let (_r, b, c) = cpu.get_std_irns();
            if imm == 1 {
                cpu.interrupt(b);
            } else if imm == 0 {
                cpu.interrupt(c);
            }
        }
        _ => {
            panic!("Invalid instruction type!")
        }
    }
}
