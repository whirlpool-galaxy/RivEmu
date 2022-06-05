/*
 * Copyright (C) 2022 Jonathan Schild - All Rights Reserved
 */

//! Emulating memory with a standard [HashMap].
//!
//! # Autors and Copyright
//! Copyright (C) 2022 Jonathan Schild - All Rights Reserved
//!  
//! - Jonathan Schild

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use crate::rv32i::RV32IBus;

/// Basic [HashMap] based memory.
pub struct Basic32Mem {
    mem: HashMap<u32, u32>,
}

impl Basic32Mem {
    /// Create new [Basic32Mem] initialised with 0.
    pub fn new() -> Basic32Mem {
        Basic32Mem {
            mem: HashMap::new(),
        }
    }

    /// Load memory image from file `memory_image_file` with starting add address `start_addr`.
    ///
    /// - `start_addr`: Start address of memory image. Must be 4 byte aligned.
    /// - `memory_image_path`: Path to memory image file.
    ///
    /// Returns [Result] containing `()` or an error message string.
    pub fn load_memory_image(
        &mut self,
        start_addr: u32,
        memory_image_path: String,
    ) -> Result<(), &str> {
        if start_addr % 4 != 0 {
            return Err("Start address misaligned!");
        }

        let file = match File::open(memory_image_path) {
            Ok(f) => f,
            Err(_) => return Err("Can't open file!"),
        };

        let reader = file.bytes();

        let mut counter = start_addr;

        for byte in reader {
            self.store_byte(counter, byte.unwrap_or(0));
            counter += 1;
        }

        Ok(())
    }
}

impl crate::rv32i::RV32IBus for Basic32Mem {
    fn load_byte(&mut self, address: u32) -> u8 {
        let (addr, off) = word_aligned_address(address);
        let data = match self.mem.get(&addr) {
            Some(d) => *d,
            _ => 0,
        };
        if off == 0 {
            (data & 0x000000FF) as u8
        } else if off == 1 {
            ((data & 0x0000FF00) >> 8) as u8
        } else if off == 2 {
            ((data & 0x00FF0000) >> 16) as u8
        } else {
            ((data & 0xFF000000) >> 24) as u8
        }
    }

    fn load_half_word(&mut self, address: u32) -> u16 {
        let (addr, off) = word_aligned_address(address);
        if off % 2 != 0 {
            panic!("Misaligned memory access at: 0x{:08x}", address);
        }
        let data = match self.mem.get(&addr) {
            Some(d) => *d,
            _ => 0,
        };
        if off == 0 {
            (data & 0x0000FFFF) as u16
        } else if off == 2 {
            ((data & 0xFFFF0000) >> 16) as u16
        } else {
            0
        }
    }

    fn load_word(&mut self, address: u32) -> u32 {
        let (addr, off) = word_aligned_address(address);
        if off != 0 {
            panic!("Misaligned memory access at: 0x{:08x}", address);
        }
        match self.mem.get(&addr) {
            Some(d) => *d,
            _ => 0,
        }
    }

    fn store_byte(&mut self, address: u32, data: u8) {
        let (addr, off) = word_aligned_address(address);
        let mut temp = match self.mem.get(&addr) {
            Some(d) => *d,
            _ => 0,
        };
        if off == 0 {
            temp &= 0xFFFFFF00;
            temp |= data as u32
        } else if off == 1 {
            temp &= 0xFFFF00FF;
            temp |= (data as u32) << 8;
        } else if off == 2 {
            temp &= 0xFF00FFFF;
            temp |= (data as u32) << 16;
        } else {
            temp &= 0x00FFFFFF;
            temp |= (data as u32) << 24;
        }
        self.mem.insert(addr, temp);
    }

    fn store_half_word(&mut self, address: u32, data: u16) {
        let (addr, off) = word_aligned_address(address);
        if off % 2 != 0 {
            panic!("Misaligned memory access at: 0x{:08x}", address);
        }
        let mut temp = match self.mem.get(&addr) {
            Some(d) => *d,
            _ => 0,
        };
        if off == 0 {
            temp &= 0xFFFF0000;
            temp |= data as u32
        } else if off == 2 {
            temp &= 0x0000FFFF;
            temp |= (data as u32) << 16;
        }
        self.mem.insert(addr, temp);
    }

    fn store_word(&mut self, address: u32, data: u32) {
        let (addr, off) = word_aligned_address(address);
        if off != 0 {
            panic!("Misaligned memory access at: 0x{:08x}", address);
        }
        self.mem.insert(addr, data);
    }
}

/// Converts an address in a 4 byte aligned part and an offset.
///
/// - `address`: Address to convert.
///
/// Returns a tuple `(aligned, offset)` where `aligned` is `address >> 2` and `offset` is `address % 4`.
fn word_aligned_address(address: u32) -> (u32, u32) {
    (address >> 2, address % 4)
}
