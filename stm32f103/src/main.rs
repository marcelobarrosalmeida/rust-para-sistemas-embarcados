#![feature(used)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;
extern crate panic_semihosting; // panicking behavior

use core::fmt::Write;

use cortex_m::asm;
use cortex_m_semihosting::hio;

#[link_section = ".vector_table.interrupts"]
#[used]
static INTERRUPTS: [extern "C" fn(); 66] = [default_handler; 66];

extern "C" fn default_handler() {
    asm::bkpt();
}

fn main() {
    let mut stdout = hio::hstdout().unwrap();
    writeln!(stdout, "Alo mundo !").unwrap();
}


