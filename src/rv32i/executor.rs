/*
 * Copyright (C) 2022 Jonathan Schild - All Rights Reserved
 */

use super::*;

pub fn unknown(cpu: &mut dyn RV32IInterface, instruction: BaseInstruction) {
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
            let mut pc = cpu.read_pc();
            cpu.write_register(rd, pc);
            pc = pc.wrapping_sub(4);
            let new_pc = cpu.read_register(rs1);
            cpu.write_pc(new_pc.wrapping_add(imm as u32) & 0xFFFFFFFE);
            println!("rs1: {:08x}, imm: {:08x}, pc: {:08x}, new_pc: {:08x}, written_pc: {:08x}", rs1, imm, pc, new_pc, cpu.read_pc());
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

pub fn fence(cpu: &mut dyn RV32IInterface, instruction: BaseInstruction) {}

pub fn ecall(cpu: &mut dyn RV32IInterface, instruction: BaseInstruction) {}

pub fn ebreak(cpu: &mut dyn RV32IInterface, instruction: BaseInstruction) {}

#[cfg(test)]
mod tests {
    use super::*;

    struct Dummy {
        pub register: [u32; 32],
        pub pc: u32,
        pub bus: DummyBusStatus,
    }

    struct DummyBusStatus {
        pub last_write: bool,
        pub last_byte: bool,
        pub last_half_word: bool,
        pub last_word: bool,
        pub last_addr: u32,
        pub last_data: u32,
    }

    impl DummyBusStatus {
        pub fn new() -> DummyBusStatus {
            DummyBusStatus {
                last_write: false,
                last_byte: false,
                last_half_word: false,
                last_word: false,
                last_addr: 0,
                last_data: 0,
            }
        }

        pub fn reset(&mut self) {
            self.last_write = false;
            self.last_byte = false;
            self.last_half_word = false;
            self.last_word = false;
            self.last_addr = 0;
            self.last_data = 0;
        }
    }

    impl Dummy {
        pub fn new() -> Dummy {
            Dummy {
                register: [0; 32],
                pc: 0,
                bus: DummyBusStatus::new(),
            }
        }
    }

    impl RV32IBus for Dummy {
        fn load_byte(&mut self, address: u32) -> u8 {
            self.bus.last_addr = address;
            self.bus.last_byte = true;
            self.bus.last_data as u8
        }

        fn load_half_word(&mut self, address: u32) -> u16 {
            self.bus.last_addr = address;
            self.bus.last_half_word = true;
            self.bus.last_data as u16
        }

        fn load_word(&mut self, address: u32) -> u32 {
            self.bus.last_addr = address;
            self.bus.last_word = true;
            self.bus.last_data
        }

        fn store_byte(&mut self, address: u32, data: u8) {
            self.bus.last_addr = address;
            self.bus.last_data = data as u32;
            self.bus.last_byte = true;
            self.bus.last_write = true;
        }

        fn store_half_word(&mut self, address: u32, data: u16) {
            self.bus.last_addr = address;
            self.bus.last_data = data as u32;
            self.bus.last_half_word = true;
            self.bus.last_write = true;
        }

        fn store_word(&mut self, address: u32, data: u32) {
            self.bus.last_addr = address;
            self.bus.last_data = data as u32;
            self.bus.last_word = true;
            self.bus.last_write = true;
        }
    }

    impl RV32IInterface for Dummy {
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

    #[test]
    fn unknown() {}

    #[test]
    fn test_addi() {
        let mut dummy = Dummy::new();

        dummy.register[0x01] = 0xFFFFFFFF; // -1

        let a = BaseInstruction::IType {
            rd: 0x02,
            rs1: 0x01,
            imm: -0x800, // -2048
        };

        addi(&mut dummy, a);

        assert_eq!(dummy.register[0x02], 0xFFFFF7FF); // -2049

        let b = BaseInstruction::IType {
            rd: 0x02,
            rs1: 0x01,
            imm: 0x001,
        };

        addi(&mut dummy, b);

        assert_eq!(dummy.register[0x02], 0x0);

        let c = BaseInstruction::IType {
            rd: 0x02,
            rs1: 0x01,
            imm: -0x1,
        };

        addi(&mut dummy, c);

        assert_eq!(dummy.register[0x02], 0xFFFFFFFE);

        dummy.register[0x1F] = 10;

        let d = BaseInstruction::IType {
            rd: 0x01,
            rs1: 0x1F,
            imm: -3,
        };

        addi(&mut dummy, d);

        assert_eq!(dummy.register[0x01], 7);
    }
}
