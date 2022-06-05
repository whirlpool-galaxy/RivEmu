/*
 * Copyright (C) 2022 Jonathan Schild - All Rights Reserved
 */

//! # RivEmu
//!
//! Simple and easily extendable RISC-V emulator.
//!
//! - [Git](https://gitlab.com/nerds-forge/rivemu)
//! - [Issues & Bugs](https://gitlab.com/nerds-forge/rivemu/-/issues/new)
//!
//! # Autors and Copyright
//! Copyright (C) 2022 Jonathan Schild - All Rights Reserved
//!  
//! - Jonathan Schild

use std::cell::RefCell;
use std::env;
use std::process::*;
use std::rc::Rc;

use riv_emu::memory::simple_mem::Basic32Mem;
use riv_emu::peripheral::ascii_out::AsciiOut;
use riv_emu::peripheral::MMIOMapper;
use riv_emu::rv32i::*;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    let file_name = match args.get(1) {
        Some(s) => s,
        None => {
            println!("Missing image file!");
            return ExitCode::FAILURE;
        }
    };

    let start_addr = match args.get(2) {
        Some(s) => match u32::from_str_radix(s.trim_start_matches("0x"), 16) {
            Ok(addr) => addr,
            Err(_) => {
                println!("Can not parse start address!");
                return ExitCode::FAILURE;
            }
        },
        None => 0x00000000,
    };

    let cycles = match args.get(3) {
        Some(s) => match s.parse::<u32>() {
            Ok(addr) => addr,
            Err(_) => {
                println!("Can not parse number of cycles!");
                return ExitCode::FAILURE;
            }
        },
        None => 0,
    };

    let bus = Rc::new(RefCell::new(Basic32Mem::new()));

    match bus
        .borrow_mut()
        .load_memory_image(start_addr, file_name.to_string())
    {
        Ok(_) => (),
        Err(s) => {
            println!("{}", s);
            return ExitCode::FAILURE;
        }
    };

    let iomapper = Rc::new(RefCell::new(MMIOMapper::new(bus)));

    let res = iomapper
        .borrow_mut()
        .add_mapping(0x80000000, Rc::new(RefCell::new(AsciiOut::new())));

    match res {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    }

    let mut cpu = RV32ICPU::new();

    cpu.connect_to_bus(Option::Some(iomapper.clone() as Rc<RefCell<dyn RV32IBus>>));

    cpu.interrupt(0);

    if cycles == 0 {
        loop {
            cpu.execute_next();
        }
    } else {
        for _ in 0..cycles {
            cpu.execute_next();
        }
    }

    ExitCode::SUCCESS
}
