#![no_main]
#![no_std]

{% if microcontroller == "nrf52840" %}
use nrf52840_hal::{self as hal, Timer};
{%- endif %}

use embedded_hal::blocking::delay::DelayMs;

use {{ project_name }} as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    let board = hal::pac::Peripherals::take().unwrap();

    let mut timer = Timer::new(board.TIMER0);

    timer.delay_ms(1000u32);

    loop {
        defmt::info!("Hello");
        timer.delay_ms(1000u32);

        defmt::info!("World");
        timer.delay_ms(1000u32);
    }
}
