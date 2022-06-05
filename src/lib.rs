/*
 * Copyright (C) 2022 Jonathan Schild - All Rights Reserved
 */

//! # LibRivEmu
//!
//! Easily extendable library to emulate RISC-V.
//!
//! - [Git](https://gitlab.com/nerds-forge/rivemu)
//! - [Issues & Bugs](https://gitlab.com/nerds-forge/rivemu/-/issues/new)
//!
//! ## Features
//!
//! List of planed and implemented features. And a list of features that are considered stable
//!
//! ### Implementation status
//!
//! - [x] rv32i
//! - [ ] Zicsr
//!     - [ ] Machine
//!     - [ ] Supervisor
//!     - [ ] User
//! - [ ] A
//! - [ ] M
//! - [ ] F
//! - [ ] D
//!
//! ### Release status
//!
//! - [ ] rv32i
//! - [ ] Zicsr
//!     - [ ] Machine
//!     - [ ] Supervisor
//!     - [ ] User
//! - [ ] A
//! - [ ] M
//! - [ ] F
//! - [ ] D
//!
//! # Autors and Copyright
//! Copyright (C) 2022 Jonathan Schild - All Rights Reserved
//!  
//! - Jonathan Schild

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
