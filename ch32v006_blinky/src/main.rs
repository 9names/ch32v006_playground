#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]

// LEDs on ch32v006 evt are not connected to anything by default.
// I've jumpered them to D0 and C0

use hal::delay::Delay;
use hal::gpio::{Level, Output};
use hal::println;
use {ch32_hal as hal, panic_halt as _};

#[qingke_rt::entry]
fn main() -> ! {
    hal::debug::SDIPrint::enable();
    // Print something immediately to confirm that SDI is working
    println!("hello world!");

    let p = hal::init(ch32_hal::Config {
        rcc: hal::rcc::Config::SYSCLK_FREQ_24MHZ_HSE,
        ..Default::default()
    });

    let mut delay = Delay;

    let mut led = Output::new(p.PD0, Level::Low, Default::default());
    let mut led2 = Output::new(p.PC0, Level::Low, Default::default());

    loop {
        led.set_high();
        delay.delay_ms(1000);
        led2.set_high();
        delay.delay_ms(1000);
        led.set_low();
        delay.delay_ms(1000);
        led2.set_low();
        delay.delay_ms(1000);
    }
}
