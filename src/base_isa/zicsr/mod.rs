/*
 * Copyright (C) 2022 Jonathan Schild - MIT License
 */

// DOC

//! TBA
//!
//! # Authors and Copyright
//! Copyright (C) 2022 Jonathan Schild - MIT License
//!  
//! - Jonathan Schild

enum ZiscrVal {
    Mxlen64(u64),
    Mxlen32(u32),
}

enum AccessMode {
    DRW,
    MRW,
    MRO,
    HRW,
    HRO,
    SRW,
    SRO,
    URW,
    URO,
}

trait ZicsrRegister {
    fn get_access_mode() -> AccessMode;
    fn get_zicsr() -> ZiscrVal;
    fn set_zicsr(value: ZiscrVal) -> bool;
}

mod machine {
}