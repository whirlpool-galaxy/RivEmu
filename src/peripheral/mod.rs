/*
 * Copyright (C) 2022 Jonathan Schild - All Rights Reserved
 */

use std::collections::HashMap;

use super::rv32i::RV32IBus;

struct MMIOMapper {
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
        mut self,
        base_address: u32,
        device: Rc<RefCell<dyn RV32IBus>>,
    ) -> Result<(), &str> {
        if base_address != get_base(base_address) {
            Err("Is not a base address!")
        }
        mmio_map.insert(base_address, device);
        Ok()
    }

    fn get_base(address: u32) -> u32 {
        address & !0b111111
    }

    fn get_mapping(&self, address: u32) -> Rc<RefCell<dyn RV32IBus>> {
        match mmio_map.get(get_base(address)) {
            Some(b) => b,
            None => default_bus,
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
