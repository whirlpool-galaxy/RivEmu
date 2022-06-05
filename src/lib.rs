/*
 * Copyright (C) 2022 Jonathan Schild - MIT License
 */

#![doc = include_str!("../README.md")]

mod utility;

pub mod memory;
pub mod peripheral;
pub mod rv32i;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
