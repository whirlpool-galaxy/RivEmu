/*
 * Copyright (C) 2022 Jonathan Schild - All Rights Reserved
 */

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use crate::rv32i::RV32IBus;

pub struct Basic32Mem {
    mem: HashMap<u32, u32>,
}

impl Basic32Mem {
    pub fn new() -> Basic32Mem {
        Basic32Mem {
            mem: HashMap::new(),
        }
    }

    pub fn load_memory_image(&mut self, memory_image_path: String) {
        let file = match File::open(memory_image_path) {
            Ok(f) => f,
            Err(_) => panic!("FileIOError"),
        };

        let reader = file.bytes();

        let mut counter = 0u32;

        for byte in reader {
            self.store_byte(counter, byte.unwrap_or(0));
            counter += 1;
        }
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
            panic!("misaligned memory access")
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
            panic!("misaligned memory access")
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
            panic!("misaligned memory access")
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
            panic!("misaligned memory access")
        }
        self.mem.insert(addr, data);
    }
}

fn word_aligned_address(address: u32) -> (u32, u32) {
    (address >> 2, address % 4)
}
