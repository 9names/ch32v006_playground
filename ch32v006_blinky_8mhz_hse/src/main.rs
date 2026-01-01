#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]

// LEDs on ch32v006 evt are not connected to anything by default.
// I've jumpered them to D0 and C0

use ch32_hal::{
    self as hal,
    prelude::Hertz,
    rcc::{Hse, HseMode},
};

use hal::delay::Delay;
use hal::gpio::{Level, Output};
use hal::println;

#[panic_handler]
fn _panic(info: &core::panic::PanicInfo) -> ! {
    println!("{info}",);
    loop {
        qingke::riscv::asm::nop();
    }
}

#[qingke_rt::entry]
fn main() -> ! {
    hal::debug::SDIPrint::enable();
    // Print something immediately to confirm that SDI is working
    println!("hello world!");

    // For HSE with 8Mhz external XTAL
    let mut rcc_config = ch32_hal::Config {
        rcc: hal::rcc::Config::SYSCLK_FREQ_24MHZ_HSE,
        ..Default::default()
    };

    rcc_config.rcc.hse = Some(Hse {
        freq: Hertz(8_000_000),
        mode: HseMode::Oscillator,
    });

    let p = hal::init(rcc_config);

    let mut delay = Delay;

    let mut led = Output::new(p.PD0, Level::Low, Default::default());
    let mut led2 = Output::new(p.PC0, Level::Low, Default::default());
    let mut counter = 0;
    loop {
        println!("loop {counter}");
        counter += 1;
        led.set_high();
        delay.delay_ms(250);
        led2.set_high();
        delay.delay_ms(250);
        led.set_low();
        delay.delay_ms(250);
        led2.set_low();
        delay.delay_ms(250);
    }
}
