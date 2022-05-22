/*
 * Copyright (C) 2022 Jonathan Schild - All Rights Reserved
 */

use std::cell::RefCell;
use std::env;
use std::process::*;
use std::rc::Rc;

use riv_emu::memory::simple_mem::Basic32Mem;
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
        None => 0x00000040,
    };

    let cycles = match args.get(3) {
        Some(s) => match u32::from_str_radix(s, 10) {
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

    let mut cpu = RV32ICPU::new();

    cpu.connect_to_bus(Option::Some(bus.clone() as Rc<RefCell<dyn RV32IBus>>));

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
