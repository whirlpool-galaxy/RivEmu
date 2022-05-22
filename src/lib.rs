/*
 * Copyright (C) 2022 Jonathan Schild - All Rights Reserved
 */

mod utility;

pub mod memory;
pub mod rv32i;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
