/*
 * Copyright (C) 2022 Jonathan Schild - All Rights Reserved
 */

use std::{str::from_utf8, vec::Vec};

use super::*;

pub struct AsciiOut {
    buffer: Vec<u8>,
    base_address: u32,
}

impl AsciiOut {
    pub fn new() -> AsciiOut {
        AsciiOut {
            buffer: Vec::new(),
            base_address: 0x80000000,
        }
    }

    fn flush(&mut self) {
        let s = from_utf8(self.buffer.as_slice());
        match s {
            Ok(st) => print!("{}", st),
            Err(_) => (),
        };
        self.buffer.clear();
    }

    fn clear(&mut self) {
        self.buffer.clear();
    }

    fn append(&mut self, c: u8) {
        self.buffer.push(c);
    }
}

impl RV32IBus for AsciiOut {
    fn load_byte(&mut self, address: u32) -> u8 {
        0
    }

    fn load_half_word(&mut self, address: u32) -> u16 {
        0
    }

    fn load_word(&mut self, address: u32) -> u32 {
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

    fn store_half_word(&mut self, address: u32, data: u16) {}

    fn store_word(&mut self, address: u32, data: u32) {}
}
