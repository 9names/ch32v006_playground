#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]

// LEDs on ch32v006 evt are not connected to anything by default.
// I've jumpered them to D0 and C0

use ch32_hal as hal;
use hal::delay::Delay;
use hal::gpio::{Level, Output};
mod sdiprint_ufmt;
use sdiprint_ufmt::*;

#[panic_handler]
fn _panic(info: &core::panic::PanicInfo) -> ! {
    println!("{info}",);
    loop {
        qingke::riscv::asm::nop();
    }
}

#[qingke_rt::entry]
fn main() -> ! {
    sdiprint_ufmt::SDIPrintUfmt::enable();
    // Print something immediately to confirm that SDI is working
    uprintln!("hello world!");

    // For HSE
    // let p = hal::init(ch32_hal::Config {
    //     rcc: hal::rcc::Config::SYSCLK_FREQ_24MHZ_HSE,
    //     ..Default::default()
    // });

    // For HSI we need to mess with AHB divider (48Mhz is too much? need to check datasheet)
    use ch32_hal::pac::rcc::vals::Hpre;
    let mut config = ch32_hal::Config {
        rcc: hal::rcc::Config::SYSCLK_FREQ_48MHZ_HSI,
        ..Default::default()
    };
    config.rcc.ahb_pre = Hpre::DIV2;
    let p = hal::init(config);

    uprintln!("still alive!");

    let mut delay = Delay;

    let mut led = Output::new(p.PD0, Level::Low, Default::default());
    let mut led2 = Output::new(p.PC0, Level::Low, Default::default());
    let mut counter = 0;
    loop {
        uprintln!("loop {}", counter);
        counter += 1;
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
