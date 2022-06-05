/*
 * Copyright (C) 2022 Jonathan Schild - All Rights Reserved
 */

//! Module for peripheral devices.
//!
//! # Autors and Copyright
//! Copyright (C) 2022 Jonathan Schild - All Rights Reserved
//!  
//! - Jonathan Schild

pub mod ascii_out;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::memory::simple_mem::Basic32Mem;

use super::rv32i::RV32IBus;

/// Maps 64 Byte region of memory to special [RV32IBus] devices. Base address must be 64 Byte aligned.
pub struct MMIOMapper {
    default_bus: Rc<RefCell<dyn RV32IBus>>,
    mmio_map: HashMap<u32, Rc<RefCell<dyn RV32IBus>>>,
}

impl MMIOMapper {
    pub fn new(default_bus: Rc<RefCell<dyn RV32IBus>>) -> MMIOMapper {
        MMIOMapper {
            default_bus,
            mmio_map: HashMap::new(),
        }
    }

    pub fn add_mapping(
        &mut self,
        base_address: u32,
        device: Rc<RefCell<dyn RV32IBus>>,
    ) -> Result<(), &'static str> {
        if base_address != MMIOMapper::get_base(base_address) {
            Err("Is not a base address!")
        } else {
            self.mmio_map.insert(base_address, device);
            Ok(())
        }
    }

    fn get_base(address: u32) -> u32 {
        address & !0b111111
    }

    fn get_mapping(&self, address: u32) -> Rc<RefCell<dyn RV32IBus>> {
        match self.mmio_map.get(&MMIOMapper::get_base(address)) {
            Some(b) => b.clone(),
            None => self.default_bus.clone(),
        }
    }
}

impl RV32IBus for MMIOMapper {
    fn load_byte(&mut self, address: u32) -> u8 {
        self.get_mapping(address).borrow_mut().load_byte(address)
    }

    fn load_half_word(&mut self, address: u32) -> u16 {
        self.get_mapping(address)
            .borrow_mut()
            .load_half_word(address)
    }

    fn load_word(&mut self, address: u32) -> u32 {
        self.get_mapping(address).borrow_mut().load_word(address)
    }

    fn store_byte(&mut self, address: u32, data: u8) {
        self.get_mapping(address)
            .borrow_mut()
            .store_byte(address, data);
    }

    fn store_half_word(&mut self, address: u32, data: u16) {
        self.get_mapping(address)
            .borrow_mut()
            .store_half_word(address, data);
    }

    fn store_word(&mut self, address: u32, data: u32) {
        self.get_mapping(address)
            .borrow_mut()
            .store_word(address, data);
    }
}

impl Default for Basic32Mem {
    fn default() -> Self {
        Self::new()
    }
}
