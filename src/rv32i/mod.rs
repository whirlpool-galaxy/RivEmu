/*
 * Copyright (C) 2022 Jonathan Schild - All Rights Reserved
 */

//! Basic functions, traits and structs for `rv32i` `CPU`s.
//!
//! # Authors and Copyright
//! Copyright (C) 2022 Jonathan Schild - All Rights Reserved
//!  
//! - Jonathan Schild

mod decoder;
mod executor;

use std::cell::RefCell;
use std::panic;
use std::rc::Rc;

/// Decoded instructions.
pub enum BaseInstruction {
    RType {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    IType {
        rd: u8,
        rs1: u8,
        imm: i32,
    },
    /// Combined `S-Type` or `B-Type` instruction.
    SBType {
        rs1: u8,
        rs2: u8,
        imm: i32,
    },
    /// Combined `U-Type` or `J-Type` instruction.
    UJType {
        rd: u8,
        imm: i32,
    },
    /// Type for unknown instructions.
    Unknown {
        instruction: u32,
    },
}

/// Trait to represent an interface to a 32-bit bus.
pub trait RV32IBus {
    /// Loads a byte at address `address`.
    fn load_byte(&mut self, address: u32) -> u8;

    /// Loads two bytes at address `address`.
    fn load_half_word(&mut self, address: u32) -> u16;

    /// Loads four bytes at address `address`.
    fn load_word(&mut self, address: u32) -> u32;

    /// Stores a byte at data at address `address`.
    fn store_byte(&mut self, address: u32, data: u8);

    /// Stores two bytes at data at address `address`.
    fn store_half_word(&mut self, address: u32, data: u16);

    /// Stores four bytes at data at address `address`.
    fn store_word(&mut self, address: u32, data: u32);
}

/// Trait to represent a `rv32i` based `CPU`.
pub trait RV32IInterface: RV32IBus {
    /// Reads a value from register file.
    ///
    /// If `register > 31` the behaviour is undefined. The program may panic.
    fn read_register(&self, register: u8) -> u32;

    /// Writes a value to register file.
    ///
    /// Writes to register 0 are ignored.
    /// If `register > 31` the behaviour is undefined. The program may panic.
    fn write_register(&mut self, register: u8, data: u32);

    /// Reads the value of the `program counter` register.
    fn read_pc(&self) -> u32;

    /// Writes a new value to the `program counter` register.
    fn write_pc(&mut self, pc: u32);

    /// Fetches, decodes and executes the next instruction.
    ///
    /// The `program counter` is incremented by 4 bytes after fetching the instruction.
    fn execute_next(&mut self);

    /// Causes an interrupt to occur.
    ///
    /// **WARNING:** Implementations is incomplete. Pleas view source code of the implementation bevor usage.
    fn interrupt(&mut self, number: u32);

    /// Returns the interrupt numbers for ` (Reset, ebreak, ecall)`.
    fn get_std_irns(&self) -> (u32, u32, u32);
}

/// An implementation of a `rv32i` `cpu`.
pub struct RV32ICPU {
    /// Register file.
    register: [u32; 32],
    /// Program counter.
    pc: u32,
    /// Interface to the main bus.
    bus: Option<Rc<RefCell<dyn RV32IBus>>>,
    /// Start address of the `interrupt vector`.
    irv_offset: u32,
    /// Length of the `interrupt vector`.
    irv_length: u32,
    /// Interrupt number for `reset`.
    reset_irn: u32,
    /// Interrupt number for `ebreak`.
    ebreak_irn: u32,
    /// Interrupt number for `ecall`.
    ecall_irn: u32,
}

impl RV32ICPU {
    /// Creates a new [RV32ICPU] with default values.
    ///
    /// - [RV32ICPU::irv_offset] is 0x00.
    /// - [RV32ICPU::irv_length] is 0x40.
    /// - [RV32ICPU::reset_irn] is 0.
    /// - [RV32ICPU::ebreak_irn] is 1.
    /// - [RV32ICPU::ecall_irn] is 2.
    pub fn new() -> RV32ICPU {
        RV32ICPU {
            register: [0; 32],
            pc: 0,
            bus: Option::None,
            irv_offset: 0x00,
            irv_length: 0x40,
            reset_irn: 0,
            ebreak_irn: 1,
            ecall_irn: 2,
        }
    }

    /// Connect [RV32IBus] to `CPU`.
    pub fn connect_to_bus(&mut self, bus: Option<Rc<RefCell<dyn RV32IBus>>>) {
        self.bus = bus;
    }

    /// Returns the [RV32IBus] the `CPU` is connected to.
    pub fn get_bus(&self) -> Option<Rc<RefCell<dyn RV32IBus>>> {
        self.bus.clone()
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
        if let Some(b) = &self.bus {
            b.borrow_mut().store_byte(address, data);
        };
    }

    fn store_half_word(&mut self, address: u32, data: u16) {
        if let Some(b) = &self.bus {
            b.borrow_mut().store_half_word(address, data);
        };
    }

    fn store_word(&mut self, address: u32, data: u32) {
        if let Some(b) = &self.bus {
            b.borrow_mut().store_word(address, data);
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

    fn execute_next(&mut self) {
        let instr = self.load_word(self.pc);
        self.pc = self.pc.wrapping_add(4);
        let (decoded_instr, executor) = decoder::decode(instr);
        executor(self, decoded_instr);
    }

    fn interrupt(&mut self, number: u32) {
        if number * 4 >= self.irv_length {
            eprintln!("Unknown interrupt number: {}", number);
        }
        if let Some(b) = &self.bus {
            self.pc = b.borrow_mut().load_word(self.irv_offset + (number * 4));
        };
    }

    fn get_std_irns(&self) -> (u32, u32, u32) {
        (self.reset_irn, self.ebreak_irn, self.ecall_irn)
    }
}

impl Default for RV32ICPU {
    /// Calls [RV32ICPU::new()].
    fn default() -> Self {
        Self::new()
    }
}
