#![no_main]
#![no_std]

// See
// https://github.com/knurling-rs/app-template/blob/main/src/bin/hello.rs
// https://nitschinger.at/Getting-Started-with-the-nRF52840-in-Rust/
// https://ferrous-systems.com/blog/defmt-rtt-linker-error/

use nrf52840_hal as _; // memory layout
use defmt_rtt as _; // global logger

// use nrf52840_hal::pac::Peripherals;
// use nrf52840_hal::temp::Temp;

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello!");
    exit();
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    defmt::error!("panicked ");
    exit()
}

pub fn exit() -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}