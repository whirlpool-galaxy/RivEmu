/*
 * Copyright (C) 2022 Jonathan Schild - All Rights Reserved
 */

mod decoder;
mod executor;

use std::cell::RefCell;
use std::panic;
use std::rc::Rc;

pub enum BaseInstruction {
    RType { rd: u8, rs1: u8, rs2: u8 },
    IType { rd: u8, rs1: u8, imm: i32 },
    SBType { rs1: u8, rs2: u8, imm: i32 },
    UJType { rd: u8, imm: i32 },
    Unknown { instruction: u32 },
}

pub trait RV32IBus {
    fn load_byte(&mut self, address: u32) -> u8;
    fn load_half_word(&mut self, address: u32) -> u16;
    fn load_word(&mut self, address: u32) -> u32;

    fn store_byte(&mut self, address: u32, data: u8);
    fn store_half_word(&mut self, address: u32, data: u16);
    fn store_word(&mut self, address: u32, data: u32);
}

pub trait RV32IInterface: RV32IBus {
    fn read_register(&self, register: u8) -> u32;
    fn write_register(&mut self, register: u8, data: u32);

    fn read_pc(&self) -> u32;
    fn write_pc(&mut self, pc: u32);
}

pub struct RV32ICPU {
    register: [u32; 32],
    pc: u32,
    bus: Option<Rc<RefCell<dyn RV32IBus>>>,
}

impl RV32ICPU {
    pub fn new() -> RV32ICPU {
        RV32ICPU {
            register: [0; 32],
            pc: 0,
            bus: Option::None,
        }
    }

    pub fn connect_to_bus(&mut self, bus: Option<Rc<RefCell<dyn RV32IBus>>>) {
        self.bus = bus;
    }

    pub fn get_bus(self) -> Option<Rc<RefCell<dyn RV32IBus>>> {
        self.bus
    }

    pub fn execute_next(&mut self) {
        let instr = self.load_word(self.pc);
        self.pc = self.pc.wrapping_add(4);
        let (decoded_instr, executor) = decoder::decode(instr);
        executor(self, decoded_instr);
    }
}

impl RV32IBus for RV32ICPU {
    fn load_byte(&mut self, address: u32) -> u8 {
        match &self.bus {
            Some(b) => b.borrow_mut().load_byte(address),
            None => 0,
        }
    }

    fn load_half_word(&mut self, address: u32) -> u16 {
        match &self.bus {
            Some(b) => b.borrow_mut().load_half_word(address),
            None => 0,
        }
    }

    fn load_word(&mut self, address: u32) -> u32 {
        match &self.bus {
            Some(b) => b.borrow_mut().load_word(address),
            None => 0,
        }
    }

    fn store_byte(&mut self, address: u32, data: u8) {
        match &self.bus {
            Some(b) => {
                b.borrow_mut().store_byte(address, data);
            }
            _ => {}
        };
    }

    fn store_half_word(&mut self, address: u32, data: u16) {
        match &self.bus {
            Some(b) => {
                b.borrow_mut().store_half_word(address, data);
            }
            _ => {}
        };
    }

    fn store_word(&mut self, address: u32, data: u32) {
        match &self.bus {
            Some(b) => {
                b.borrow_mut().store_word(address, data);
            }
            _ => {}
        };
    }
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
