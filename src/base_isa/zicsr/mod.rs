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
    XLen64(u64),
    XLen32(u32),
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
    fn get_zicsr(value: ZiscrVal) -> ZiscrVal;
    fn set_zicsr(value: ZiscrVal) -> bool;
}

mod machine {
    use super::{AccessMode, ZicsrRegister, ZiscrVal};

    struct Mvendorid {
        // TODO
    }

    struct Marchid {
        // TODO
    }

    struct Mimpid {
        // TODO
    }

    struct Mhartid {
        // TODO
    }

    struct Mconfigptr {
        // TODO
    }

    struct Mstatus {
        // TODO
    }

    struct Misa {
        // TODO
    }

    struct Medeleg {
        // TODO
    }

    struct Mideleg {
        // TODO
    }

    struct Mie {
        // TODO
    }

    struct Mtvec {
        // TODO
    }

    struct Mcounteren {
        // TODO
    }

    struct Mstatush {
        // TODO
    }

    struct Mscratch {
        // TODO
    }

    struct Mepc {
        // TODO
    }

    struct Mcause {
        // TODO
    }

    struct Mtval {
        // TODO
    }

    struct Mip {
        // TODO
    }

    struct Mtinst {
        // TODO
    }

    struct Mtval2 {
        // TODO
    }

    struct Menvcfg {
        // TODO
    }

    struct Menvcfgh {
        // TODO
    }

    struct Mseccfg {
        // TODO
    }

    struct Mseccfgh {
        // TODO
    }

    struct Pmpcfg {
        // TODO
    }

    struct Pmpaddr {
        // TODO
    }

    struct Mcycle {
        // TODO
    }

    struct Minstret {
        // TODO
    }

    struct Mhpmcounter {
        // TODO
    }

    struct Mcycleh {
        // TODO
    }

    struct Minstreth {
        // TODO
    }

    struct MhpmcounterXh {
        // TODO
    }

    struct Mcountinhibit {
        // TODO
    }

    struct Mhpmevent {
        // TODO
    }

    struct Tselect {
        // TODO
    }

    struct Tdata {
        // TODO
    }

    struct Mcontext {
        // TODO
    }

    struct Dcsr {
        // TODO
    }

    struct Dpc {
        // TODO
    }

    struct Dscratch {
        // TODO
    }

    impl Mvendorid {
        // TODO
    }

    impl ZicsrRegister for Mvendorid {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRO
        }
        fn get_zicsr(value: ZiscrVal) -> ZiscrVal {
            // TODO
            ZiscrVal::XLen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Marchid {
        // TODO
    }

    impl ZicsrRegister for Marchid {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRO
        }
        fn get_zicsr(value: ZiscrVal) -> ZiscrVal {
            // TODO
            ZiscrVal::XLen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mimpid {
        // TODO
    }

    impl ZicsrRegister for Mimpid {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRO
        }
        fn get_zicsr(value: ZiscrVal) -> ZiscrVal {
            // TODO
            ZiscrVal::XLen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhartid {
        // TODO
    }

    impl ZicsrRegister for Mhartid {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRO
        }
        fn get_zicsr(value: ZiscrVal) -> ZiscrVal {
            // TODO
            ZiscrVal::XLen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mconfigptr {
        // TODO
    }

    impl ZicsrRegister for Mconfigptr {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRO
        }
        fn get_zicsr(value: ZiscrVal) -> ZiscrVal {
            // TODO
            ZiscrVal::XLen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mstatus {
        // TODO
    }

    impl ZicsrRegister for Mstatus {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr(value: ZiscrVal) -> ZiscrVal {
            // TODO
            ZiscrVal::XLen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Misa {
        // TODO
    }

    impl ZicsrRegister for Misa {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr(value: ZiscrVal) -> ZiscrVal {
            // TODO
            ZiscrVal::XLen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Medeleg {
        // TODO
    }

    impl ZicsrRegister for Medeleg {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr(value: ZiscrVal) -> ZiscrVal {
            // TODO
            ZiscrVal::XLen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mideleg {
        // TODO
    }

    impl ZicsrRegister for Mideleg {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr(value: ZiscrVal) -> ZiscrVal {
            // TODO
            ZiscrVal::XLen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mie {
        // TODO
    }

    impl ZicsrRegister for Mie {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr(value: ZiscrVal) -> ZiscrVal {
            // TODO
            ZiscrVal::XLen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mtvec {
        // TODO
    }

    impl ZicsrRegister for Mtvec {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr(value: ZiscrVal) -> ZiscrVal {
            // TODO
            ZiscrVal::XLen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mcounteren {
        // TODO
    }

    impl ZicsrRegister for Mcounteren {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr(value: ZiscrVal) -> ZiscrVal {
            // TODO
            ZiscrVal::XLen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mstatush {
        // TODO
    }

    impl ZicsrRegister for Mstatush {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr(value: ZiscrVal) -> ZiscrVal {
            // TODO
            ZiscrVal::XLen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mscratch {
        // TODO
    }

    impl ZicsrRegister for Mscratch {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr(value: ZiscrVal) -> ZiscrVal {
            // TODO
            ZiscrVal::XLen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mepc {
        // TODO
    }

    impl ZicsrRegister for Mepc {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr(value: ZiscrVal) -> ZiscrVal {
            // TODO
            ZiscrVal::XLen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mcause {
        // TODO
    }

    impl ZicsrRegister for Mcause {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr(value: ZiscrVal) -> ZiscrVal {
            // TODO
            ZiscrVal::XLen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mtval {
        // TODO
    }

    impl ZicsrRegister for Mtval {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr(value: ZiscrVal) -> ZiscrVal {
            // TODO
            ZiscrVal::XLen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mip {
        // TODO
    }

    impl ZicsrRegister for Mip {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr(value: ZiscrVal) -> ZiscrVal {
            // TODO
            ZiscrVal::XLen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mtinst {
        // TODO
    }

    impl ZicsrRegister for Mtinst {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr(value: ZiscrVal) -> ZiscrVal {
            // TODO
            ZiscrVal::XLen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mtval2 {
        // TODO
    }

    impl ZicsrRegister for Mtval2 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr(value: ZiscrVal) -> ZiscrVal {
            // TODO
            ZiscrVal::XLen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Menvcfg {
        // TODO
    }

    impl ZicsrRegister for Menvcfg {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr(value: ZiscrVal) -> ZiscrVal {
            // TODO
            ZiscrVal::XLen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Menvcfgh {
        // TODO
    }

    impl ZicsrRegister for Menvcfgh {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr(value: ZiscrVal) -> ZiscrVal {
            // TODO
            ZiscrVal::XLen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mseccfg {
        // TODO
    }

    impl ZicsrRegister for Mseccfg {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr(value: ZiscrVal) -> ZiscrVal {
            // TODO
            ZiscrVal::XLen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mseccfgh {
        // TODO
    }

    impl ZicsrRegister for Mseccfgh {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr(value: ZiscrVal) -> ZiscrVal {
            // TODO
            ZiscrVal::XLen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpcfg {
        // TODO
    }

    impl ZicsrRegister for Pmpcfg {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr(value: ZiscrVal) -> ZiscrVal {
            // TODO
            ZiscrVal::XLen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr(value: ZiscrVal) -> ZiscrVal {
            // TODO
            ZiscrVal::XLen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mcycle {
        // TODO
    }

    impl ZicsrRegister for Mcycle {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr(value: ZiscrVal) -> ZiscrVal {
            // TODO
            ZiscrVal::XLen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Minstret {
        // TODO
    }

    impl ZicsrRegister for Minstret {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr(value: ZiscrVal) -> ZiscrVal {
            // TODO
            ZiscrVal::XLen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmcounter {
        // TODO
    }

    impl ZicsrRegister for Mhpmcounter {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr(value: ZiscrVal) -> ZiscrVal {
            // TODO
            ZiscrVal::XLen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mcycleh {
        // TODO
    }

    impl ZicsrRegister for Mcycleh {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr(value: ZiscrVal) -> ZiscrVal {
            // TODO
            ZiscrVal::XLen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Minstreth {
        // TODO
    }

    impl ZicsrRegister for Minstreth {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr(value: ZiscrVal) -> ZiscrVal {
            // TODO
            ZiscrVal::XLen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl MhpmcounterXh {
        // TODO
    }

    impl ZicsrRegister for MhpmcounterXh {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr(value: ZiscrVal) -> ZiscrVal {
            // TODO
            ZiscrVal::XLen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mcountinhibit {
        // TODO
    }

    impl ZicsrRegister for Mcountinhibit {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr(value: ZiscrVal) -> ZiscrVal {
            // TODO
            ZiscrVal::XLen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmevent {
        // TODO
    }

    impl ZicsrRegister for Mhpmevent {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr(value: ZiscrVal) -> ZiscrVal {
            // TODO
            ZiscrVal::XLen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Tselect {
        // TODO
    }

    impl ZicsrRegister for Tselect {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr(value: ZiscrVal) -> ZiscrVal {
            // TODO
            ZiscrVal::XLen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Tdata {
        // TODO
    }

    impl ZicsrRegister for Tdata {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr(value: ZiscrVal) -> ZiscrVal {
            // TODO
            ZiscrVal::XLen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mcontext {
        // TODO
    }

    impl ZicsrRegister for Mcontext {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr(value: ZiscrVal) -> ZiscrVal {
            // TODO
            ZiscrVal::XLen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Dcsr {
        // TODO
    }

    impl ZicsrRegister for Dcsr {
        fn get_access_mode() -> AccessMode {
            AccessMode::DRW
        }
        fn get_zicsr(value: ZiscrVal) -> ZiscrVal {
            // TODO
            ZiscrVal::XLen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Dpc {
        // TODO
    }

    impl ZicsrRegister for Dpc {
        fn get_access_mode() -> AccessMode {
            AccessMode::DRW
        }
        fn get_zicsr(value: ZiscrVal) -> ZiscrVal {
            // TODO
            ZiscrVal::XLen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Dscratch {
        // TODO
    }

    impl ZicsrRegister for Dscratch {
        fn get_access_mode() -> AccessMode {
            AccessMode::DRW
        }
        fn get_zicsr(value: ZiscrVal) -> ZiscrVal {
            // TODO
            ZiscrVal::XLen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }
}
