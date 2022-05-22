use riv_emu::rv32i::*;
use riv_emu::memory::simple_mem::Basic32Mem;
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let mut cpu = RV32ICPU::new();
    let bus = Rc::new(RefCell::new(Basic32Mem::new()));
    cpu.connect_to_bus(Option::Some(bus.clone() as Rc<RefCell<dyn RV32IBus>>));
    bus.borrow_mut().load_memory_image("test.bin".to_string());
    
    for n in 1..10 {
        cpu.execute_next();
    }
}
