/*
 * Copyright (C) 2022 Jonathan Schild - MIT License
 */

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
