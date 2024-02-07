#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::USB;
use embassy_rp::usb::{Driver, InterruptHandler};
use embassy_rp::i2c::{self, Config};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

use core::fmt::Write;

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

#[embassy_executor::task]
async fn logger_task(driver: Driver<'static, USB>) {
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // Configure USB serial logging
    let driver = Driver::new(p.USB, Irqs);
    spawner.spawn(logger_task(driver)).unwrap();
    Timer::after_secs(1).await;

    // OLED display connecter to GPIO 4/5
    log::info!("set up i2c");

    let sda = p.PIN_4;
    let scl = p.PIN_5;
    let i2c = i2c::I2c::new_blocking(p.I2C0, scl, sda, Config::default());

    let interface = I2CDisplayInterface::new(i2c);

    let mut display =
        Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0).into_terminal_mode();
    display.init().unwrap();
    let _ = display.clear();
    /* Endless loop */
    log::info!("starting loop");
    loop {
        for c in 97..123 {
            let _ = display.write_str(unsafe { core::str::from_utf8_unchecked(&[c]) });
        }
        for c in 65..91 {
            let _ = display.write_str(unsafe { core::str::from_utf8_unchecked(&[c]) });
        }
        Timer::after_secs(1).await;
        log::info!("looping");
    }
}
