/*
 * Copyright (C) 2022 Jonathan Schild - MIT License
 */

//! Module for peripheral devices.
//!
//! # Authors and Copyright
//! Copyright (C) 2022 Jonathan Schild - MIT License
//!  
//! - Jonathan Schild

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::base_isa::rv32i::RV32IBus;

/// Maps 64 Byte region of memory to special [RV32IBus] devices. Base address must be 64 Byte aligned.
pub struct MMIOMapper {
    default_bus: Rc<RefCell<dyn RV32IBus>>,
    mmio_map: HashMap<u32, Rc<RefCell<dyn RV32IBus>>>,
}

impl MMIOMapper {
    /// Create a new [MMIOMapper].
    pub fn new(default_bus: Rc<RefCell<dyn RV32IBus>>) -> MMIOMapper {
        MMIOMapper {
            default_bus,
            mmio_map: HashMap::new(),
        }
    }

    /// Add a new mapping at `base_address` which maps to `device`.
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

    /// Returns the 64 Bytes aligned base address for a given address.
    fn get_base(address: u32) -> u32 {
        address & !0b111111
    }

    /// Returns the mapped device for a given address.
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

pub mod ascii_out {
    //! Simple ascii output device.
    //!
    //! # Authors and Copyright
    //! Copyright (C) 2022 Jonathan Schild - MIT License
    //!  
    //! - Jonathan Schild

    use std::{str::from_utf8, vec::Vec};

    use super::*;

    /// Simple ascii output device.
    pub struct AsciiOut {
        buffer: Vec<u8>,
        _base_address: u32,
    }

    impl AsciiOut {
        /// Create a new [AsciiOut] instance.
        pub fn new() -> AsciiOut {
            AsciiOut {
                buffer: Vec::new(),
                _base_address: 0x80000000,
            }
        }

        /// Flushes the output buffer to the standard output.
        fn flush(&mut self) {
            let s = from_utf8(self.buffer.as_slice());

            if let Ok(st) = s {
                print!("{}", st)
            };

            self.buffer.clear();
        }

        /// Clears the output buffer.
        fn clear(&mut self) {
            self.buffer.clear();
        }

        /// Append a ascii character to the output buffer.
        fn append(&mut self, c: u8) {
            self.buffer.push(c);
        }
    }

    impl RV32IBus for AsciiOut {
        fn load_byte(&mut self, _address: u32) -> u8 {
            0
        }

        fn load_half_word(&mut self, _address: u32) -> u16 {
            0
        }

        fn load_word(&mut self, _address: u32) -> u32 {
            0
        }

        fn store_byte(&mut self, address: u32, data: u8) {
            let offset = address & 0b111111;
            if offset == 0 {
                self.flush();
            } else if offset == 1 {
                self.clear();
            } else if offset == 2 {
                self.append(data);
            }
        }

        fn store_half_word(&mut self, _address: u32, _data: u16) {}

        fn store_word(&mut self, _address: u32, _data: u32) {}
    }

    impl Default for AsciiOut {
        /// Calls [AsciiOut::new()].
        fn default() -> Self {
            Self::new()
        }
    }
}
