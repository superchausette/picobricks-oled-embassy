#![no_std]
#![no_main]

use core::fmt::Write;
use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::{gpio::{Level, Output, Pin}, i2c::{self, Config}};
use embassy_time::Timer;

use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};
use {defmt_rtt as _, panic_probe as _};

mod char_generator;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // OLED display connecter to GPIO 4/5
    let sda = p.PIN_4;
    let scl = p.PIN_5;
    let i2c = i2c::I2c::new_blocking(p.I2C0, scl, sda, Config::default());

    let interface = I2CDisplayInterface::new(i2c);
    let mut display =
        Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0).into_terminal_mode();

    display.init().unwrap();
    let _ = display.clear();

    let mut generator = char_generator::CharGenerator::new();

    /* Endless loop*/
    loop {
        let _ = display.print_char(generator.next() as char);
        Timer::after_millis(100).await;
    }
}
