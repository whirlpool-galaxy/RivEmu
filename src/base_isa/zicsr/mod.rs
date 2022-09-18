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

    struct Pmpcfg0 {
        // TODO
    }

    struct Pmpcfg1 {
        // TODO
    }

    struct Pmpcfg2 {
        // TODO
    }

    struct Pmpcfg3 {
        // TODO
    }

    struct Pmpcfg4 {
        // TODO
    }

    struct Pmpcfg5 {
        // TODO
    }

    struct Pmpcfg6 {
        // TODO
    }

    struct Pmpcfg7 {
        // TODO
    }

    struct Pmpcfg8 {
        // TODO
    }

    struct Pmpcfg9 {
        // TODO
    }

    struct Pmpcfg10 {
        // TODO
    }

    struct Pmpcfg11 {
        // TODO
    }

    struct Pmpcfg12 {
        // TODO
    }

    struct Pmpcfg13 {
        // TODO
    }

    struct Pmpcfg14 {
        // TODO
    }

    struct Pmpcfg15 {
        // TODO
    }

    struct Pmpaddr0 {
        // TODO
    }

    struct Pmpaddr1 {
        // TODO
    }

    struct Pmpaddr2 {
        // TODO
    }

    struct Pmpaddr3 {
        // TODO
    }

    struct Pmpaddr4 {
        // TODO
    }

    struct Pmpaddr5 {
        // TODO
    }

    struct Pmpaddr6 {
        // TODO
    }

    struct Pmpaddr7 {
        // TODO
    }

    struct Pmpaddr8 {
        // TODO
    }

    struct Pmpaddr9 {
        // TODO
    }

    struct Pmpaddr10 {
        // TODO
    }

    struct Pmpaddr11 {
        // TODO
    }

    struct Pmpaddr12 {
        // TODO
    }

    struct Pmpaddr13 {
        // TODO
    }

    struct Pmpaddr14 {
        // TODO
    }

    struct Pmpaddr15 {
        // TODO
    }

    struct Pmpaddr16 {
        // TODO
    }

    struct Pmpaddr17 {
        // TODO
    }

    struct Pmpaddr18 {
        // TODO
    }

    struct Pmpaddr19 {
        // TODO
    }

    struct Pmpaddr20 {
        // TODO
    }

    struct Pmpaddr21 {
        // TODO
    }

    struct Pmpaddr22 {
        // TODO
    }

    struct Pmpaddr23 {
        // TODO
    }

    struct Pmpaddr24 {
        // TODO
    }

    struct Pmpaddr25 {
        // TODO
    }

    struct Pmpaddr26 {
        // TODO
    }

    struct Pmpaddr27 {
        // TODO
    }

    struct Pmpaddr28 {
        // TODO
    }

    struct Pmpaddr29 {
        // TODO
    }

    struct Pmpaddr30 {
        // TODO
    }

    struct Pmpaddr31 {
        // TODO
    }

    struct Pmpaddr32 {
        // TODO
    }

    struct Pmpaddr33 {
        // TODO
    }

    struct Pmpaddr34 {
        // TODO
    }

    struct Pmpaddr35 {
        // TODO
    }

    struct Pmpaddr36 {
        // TODO
    }

    struct Pmpaddr37 {
        // TODO
    }

    struct Pmpaddr38 {
        // TODO
    }

    struct Pmpaddr39 {
        // TODO
    }

    struct Pmpaddr40 {
        // TODO
    }

    struct Pmpaddr41 {
        // TODO
    }

    struct Pmpaddr42 {
        // TODO
    }

    struct Pmpaddr43 {
        // TODO
    }

    struct Pmpaddr44 {
        // TODO
    }

    struct Pmpaddr45 {
        // TODO
    }

    struct Pmpaddr46 {
        // TODO
    }

    struct Pmpaddr47 {
        // TODO
    }

    struct Pmpaddr48 {
        // TODO
    }

    struct Pmpaddr49 {
        // TODO
    }

    struct Pmpaddr50 {
        // TODO
    }

    struct Pmpaddr51 {
        // TODO
    }

    struct Pmpaddr52 {
        // TODO
    }

    struct Pmpaddr53 {
        // TODO
    }

    struct Pmpaddr54 {
        // TODO
    }

    struct Pmpaddr55 {
        // TODO
    }

    struct Pmpaddr56 {
        // TODO
    }

    struct Pmpaddr57 {
        // TODO
    }

    struct Pmpaddr58 {
        // TODO
    }

    struct Pmpaddr59 {
        // TODO
    }

    struct Pmpaddr60 {
        // TODO
    }

    struct Pmpaddr61 {
        // TODO
    }

    struct Pmpaddr62 {
        // TODO
    }

    struct Pmpaddr63 {
        // TODO
    }

    struct Mcycle {
        // TODO
    }

    struct Minstret {
        // TODO
    }

    struct Mhpmcounter3 {
        // TODO
    }

    struct Mhpmcounter4 {
        // TODO
    }

    struct Mhpmcounter5 {
        // TODO
    }

    struct Mhpmcounter6 {
        // TODO
    }

    struct Mhpmcounter7 {
        // TODO
    }

    struct Mhpmcounter8 {
        // TODO
    }

    struct Mhpmcounter9 {
        // TODO
    }

    struct Mhpmcounter10 {
        // TODO
    }

    struct Mhpmcounter11 {
        // TODO
    }

    struct Mhpmcounter12 {
        // TODO
    }

    struct Mhpmcounter13 {
        // TODO
    }

    struct Mhpmcounter14 {
        // TODO
    }

    struct Mhpmcounter15 {
        // TODO
    }

    struct Mhpmcounter16 {
        // TODO
    }

    struct Mhpmcounter17 {
        // TODO
    }

    struct Mhpmcounter18 {
        // TODO
    }

    struct Mhpmcounter19 {
        // TODO
    }

    struct Mhpmcounter20 {
        // TODO
    }

    struct Mhpmcounter21 {
        // TODO
    }

    struct Mhpmcounter22 {
        // TODO
    }

    struct Mhpmcounter23 {
        // TODO
    }

    struct Mhpmcounter24 {
        // TODO
    }

    struct Mhpmcounter25 {
        // TODO
    }

    struct Mhpmcounter26 {
        // TODO
    }

    struct Mhpmcounter27 {
        // TODO
    }

    struct Mhpmcounter28 {
        // TODO
    }

    struct Mhpmcounter29 {
        // TODO
    }

    struct Mhpmcounter30 {
        // TODO
    }

    struct Mhpmcounter31 {
        // TODO
    }

    struct Mcycleh {
        // TODO
    }

    struct Minstreth {
        // TODO
    }

    struct Mhpmcounter3h {
        // TODO
    }

    struct Mhpmcounter4h {
        // TODO
    }

    struct Mhpmcounter31h {
        // TODO
    }

    struct Mcountinhibit {
        // TODO
    }

    struct Mhpmevent3 {
        // TODO
    }

    struct Mhpmevent4 {
        // TODO
    }

    struct Mhpmevent5 {
        // TODO
    }

    struct Mhpmevent6 {
        // TODO
    }

    struct Mhpmevent7 {
        // TODO
    }

    struct Mhpmevent8 {
        // TODO
    }

    struct Mhpmevent9 {
        // TODO
    }

    struct Mhpmevent10 {
        // TODO
    }

    struct Mhpmevent11 {
        // TODO
    }

    struct Mhpmevent12 {
        // TODO
    }

    struct Mhpmevent13 {
        // TODO
    }

    struct Mhpmevent14 {
        // TODO
    }

    struct Mhpmevent15 {
        // TODO
    }

    struct Mhpmevent16 {
        // TODO
    }

    struct Mhpmevent17 {
        // TODO
    }

    struct Mhpmevent18 {
        // TODO
    }

    struct Mhpmevent19 {
        // TODO
    }

    struct Mhpmevent20 {
        // TODO
    }

    struct Mhpmevent21 {
        // TODO
    }

    struct Mhpmevent22 {
        // TODO
    }

    struct Mhpmevent23 {
        // TODO
    }

    struct Mhpmevent24 {
        // TODO
    }

    struct Mhpmevent25 {
        // TODO
    }

    struct Mhpmevent26 {
        // TODO
    }

    struct Mhpmevent27 {
        // TODO
    }

    struct Mhpmevent28 {
        // TODO
    }

    struct Mhpmevent29 {
        // TODO
    }

    struct Mhpmevent30 {
        // TODO
    }

    struct Mhpmevent31 {
        // TODO
    }

    struct Tselect {
        // TODO
    }

    struct Tdata1 {
        // TODO
    }

    struct Tdata2 {
        // TODO
    }

    struct Tdata3 {
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

    struct Dscratch0 {
        // TODO
    }

    struct Dscratch1 {
        // TODO
    }

    impl Mvendorid {
        // TODO
    }

    impl ZicsrRegister for Mvendorid {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRO
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
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
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
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
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
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
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
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
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
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
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
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
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
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
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
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
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
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
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
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
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
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
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
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
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
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
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
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
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
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
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
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
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
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
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
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
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
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
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
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
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
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
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
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
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
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
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpcfg0 {
        // TODO
    }

    impl ZicsrRegister for Pmpcfg0 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpcfg1 {
        // TODO
    }

    impl ZicsrRegister for Pmpcfg1 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpcfg2 {
        // TODO
    }

    impl ZicsrRegister for Pmpcfg2 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpcfg3 {
        // TODO
    }

    impl ZicsrRegister for Pmpcfg3 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpcfg4 {
        // TODO
    }

    impl ZicsrRegister for Pmpcfg4 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpcfg5 {
        // TODO
    }

    impl ZicsrRegister for Pmpcfg5 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpcfg6 {
        // TODO
    }

    impl ZicsrRegister for Pmpcfg6 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpcfg7 {
        // TODO
    }

    impl ZicsrRegister for Pmpcfg7 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpcfg8 {
        // TODO
    }

    impl ZicsrRegister for Pmpcfg8 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpcfg9 {
        // TODO
    }

    impl ZicsrRegister for Pmpcfg9 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpcfg10 {
        // TODO
    }

    impl ZicsrRegister for Pmpcfg10 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpcfg11 {
        // TODO
    }

    impl ZicsrRegister for Pmpcfg11 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpcfg12 {
        // TODO
    }

    impl ZicsrRegister for Pmpcfg12 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpcfg13 {
        // TODO
    }

    impl ZicsrRegister for Pmpcfg13 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpcfg14 {
        // TODO
    }

    impl ZicsrRegister for Pmpcfg14 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpcfg15 {
        // TODO
    }

    impl ZicsrRegister for Pmpcfg15 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr0 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr0 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr1 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr1 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr2 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr2 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr3 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr3 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr4 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr4 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr5 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr5 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr6 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr6 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr7 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr7 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr8 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr8 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr9 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr9 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr10 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr10 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr11 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr11 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr12 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr12 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr13 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr13 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr14 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr14 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr15 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr15 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr16 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr16 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr17 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr17 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr18 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr18 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr19 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr19 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr20 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr20 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr21 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr21 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr22 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr22 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr23 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr23 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr24 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr24 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr25 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr25 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr26 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr26 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr27 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr27 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr28 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr28 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr29 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr29 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr30 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr30 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr31 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr31 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr32 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr32 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr33 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr33 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr34 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr34 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr35 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr35 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr36 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr36 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr37 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr37 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr38 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr38 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr39 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr39 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr40 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr40 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr41 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr41 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr42 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr42 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr43 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr43 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr44 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr44 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr45 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr45 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr46 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr46 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr47 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr47 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr48 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr48 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr49 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr49 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr50 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr50 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr51 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr51 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr52 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr52 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr53 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr53 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr54 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr54 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr55 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr55 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr56 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr56 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr57 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr57 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr58 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr58 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr59 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr59 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr60 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr60 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr61 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr61 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr62 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr62 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Pmpaddr63 {
        // TODO
    }

    impl ZicsrRegister for Pmpaddr63 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
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
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
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
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmcounter3 {
        // TODO
    }

    impl ZicsrRegister for Mhpmcounter3 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmcounter4 {
        // TODO
    }

    impl ZicsrRegister for Mhpmcounter4 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmcounter5 {
        // TODO
    }

    impl ZicsrRegister for Mhpmcounter5 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmcounter6 {
        // TODO
    }

    impl ZicsrRegister for Mhpmcounter6 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmcounter7 {
        // TODO
    }

    impl ZicsrRegister for Mhpmcounter7 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmcounter8 {
        // TODO
    }

    impl ZicsrRegister for Mhpmcounter8 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmcounter9 {
        // TODO
    }

    impl ZicsrRegister for Mhpmcounter9 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmcounter10 {
        // TODO
    }

    impl ZicsrRegister for Mhpmcounter10 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmcounter11 {
        // TODO
    }

    impl ZicsrRegister for Mhpmcounter11 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmcounter12 {
        // TODO
    }

    impl ZicsrRegister for Mhpmcounter12 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmcounter13 {
        // TODO
    }

    impl ZicsrRegister for Mhpmcounter13 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmcounter14 {
        // TODO
    }

    impl ZicsrRegister for Mhpmcounter14 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmcounter15 {
        // TODO
    }

    impl ZicsrRegister for Mhpmcounter15 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmcounter16 {
        // TODO
    }

    impl ZicsrRegister for Mhpmcounter16 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmcounter17 {
        // TODO
    }

    impl ZicsrRegister for Mhpmcounter17 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmcounter18 {
        // TODO
    }

    impl ZicsrRegister for Mhpmcounter18 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmcounter19 {
        // TODO
    }

    impl ZicsrRegister for Mhpmcounter19 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmcounter20 {
        // TODO
    }

    impl ZicsrRegister for Mhpmcounter20 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmcounter21 {
        // TODO
    }

    impl ZicsrRegister for Mhpmcounter21 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmcounter22 {
        // TODO
    }

    impl ZicsrRegister for Mhpmcounter22 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmcounter23 {
        // TODO
    }

    impl ZicsrRegister for Mhpmcounter23 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmcounter24 {
        // TODO
    }

    impl ZicsrRegister for Mhpmcounter24 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmcounter25 {
        // TODO
    }

    impl ZicsrRegister for Mhpmcounter25 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmcounter26 {
        // TODO
    }

    impl ZicsrRegister for Mhpmcounter26 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmcounter27 {
        // TODO
    }

    impl ZicsrRegister for Mhpmcounter27 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmcounter28 {
        // TODO
    }

    impl ZicsrRegister for Mhpmcounter28 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmcounter29 {
        // TODO
    }

    impl ZicsrRegister for Mhpmcounter29 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmcounter30 {
        // TODO
    }

    impl ZicsrRegister for Mhpmcounter30 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmcounter31 {
        // TODO
    }

    impl ZicsrRegister for Mhpmcounter31 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
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
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
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
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmcounter3h {
        // TODO
    }

    impl ZicsrRegister for Mhpmcounter3h {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmcounter4h {
        // TODO
    }

    impl ZicsrRegister for Mhpmcounter4h {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmcounter31h {
        // TODO
    }

    impl ZicsrRegister for Mhpmcounter31h {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
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
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmevent3 {
        // TODO
    }

    impl ZicsrRegister for Mhpmevent3 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmevent4 {
        // TODO
    }

    impl ZicsrRegister for Mhpmevent4 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmevent5 {
        // TODO
    }

    impl ZicsrRegister for Mhpmevent5 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmevent6 {
        // TODO
    }

    impl ZicsrRegister for Mhpmevent6 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmevent7 {
        // TODO
    }

    impl ZicsrRegister for Mhpmevent7 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmevent8 {
        // TODO
    }

    impl ZicsrRegister for Mhpmevent8 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmevent9 {
        // TODO
    }

    impl ZicsrRegister for Mhpmevent9 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmevent10 {
        // TODO
    }

    impl ZicsrRegister for Mhpmevent10 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmevent11 {
        // TODO
    }

    impl ZicsrRegister for Mhpmevent11 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmevent12 {
        // TODO
    }

    impl ZicsrRegister for Mhpmevent12 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmevent13 {
        // TODO
    }

    impl ZicsrRegister for Mhpmevent13 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmevent14 {
        // TODO
    }

    impl ZicsrRegister for Mhpmevent14 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmevent15 {
        // TODO
    }

    impl ZicsrRegister for Mhpmevent15 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmevent16 {
        // TODO
    }

    impl ZicsrRegister for Mhpmevent16 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmevent17 {
        // TODO
    }

    impl ZicsrRegister for Mhpmevent17 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmevent18 {
        // TODO
    }

    impl ZicsrRegister for Mhpmevent18 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmevent19 {
        // TODO
    }

    impl ZicsrRegister for Mhpmevent19 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmevent20 {
        // TODO
    }

    impl ZicsrRegister for Mhpmevent20 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmevent21 {
        // TODO
    }

    impl ZicsrRegister for Mhpmevent21 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmevent22 {
        // TODO
    }

    impl ZicsrRegister for Mhpmevent22 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmevent23 {
        // TODO
    }

    impl ZicsrRegister for Mhpmevent23 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmevent24 {
        // TODO
    }

    impl ZicsrRegister for Mhpmevent24 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmevent25 {
        // TODO
    }

    impl ZicsrRegister for Mhpmevent25 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmevent26 {
        // TODO
    }

    impl ZicsrRegister for Mhpmevent26 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmevent27 {
        // TODO
    }

    impl ZicsrRegister for Mhpmevent27 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmevent28 {
        // TODO
    }

    impl ZicsrRegister for Mhpmevent28 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmevent29 {
        // TODO
    }

    impl ZicsrRegister for Mhpmevent29 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmevent30 {
        // TODO
    }

    impl ZicsrRegister for Mhpmevent30 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Mhpmevent31 {
        // TODO
    }

    impl ZicsrRegister for Mhpmevent31 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
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
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Tdata1 {
        // TODO
    }

    impl ZicsrRegister for Tdata1 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Tdata2 {
        // TODO
    }

    impl ZicsrRegister for Tdata2 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Tdata3 {
        // TODO
    }

    impl ZicsrRegister for Tdata3 {
        fn get_access_mode() -> AccessMode {
            AccessMode::MRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
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
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
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
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
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
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Dscratch0 {
        // TODO
    }

    impl ZicsrRegister for Dscratch0 {
        fn get_access_mode() -> AccessMode {
            AccessMode::DRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }

    impl Dscratch1 {
        // TODO
    }

    impl ZicsrRegister for Dscratch1 {
        fn get_access_mode() -> AccessMode {
            AccessMode::DRW
        }
        fn get_zicsr() -> ZiscrVal {
            // TODO
            ZiscrVal::Mxlen32(0)
        }

        fn set_zicsr(value: ZiscrVal) -> bool {
            // TODO
            false
        }
    }
}
