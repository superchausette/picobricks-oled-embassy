#![no_std]
#![no_main]

use core::fmt::Write;
use embassy_executor::Spawner;
use embassy_rp::adc::{Adc, Channel, Config as AdcOnfig, InterruptHandler};
use embassy_rp::bind_interrupts;
use embassy_rp::{
    gpio::{Input, Level, Output, Pull},
    i2c::{self, Config},
};
use embassy_time::Timer;

use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};
use {defmt_rtt as _, panic_probe as _};

mod char_generator;

bind_interrupts!(struct Irqs {
    ADC_IRQ_FIFO => InterruptHandler;
});

enum DisplayScreen {
    Delay,
    Characters,
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // Potentiometer
    let mut adc = Adc::new(p.ADC, Irqs, AdcOnfig::default());
    let mut potentiometer_ch = Channel::new_pin(p.PIN_26, Pull::None);

    // Button that will display delay screen when pressed
    let button: Input<'_, embassy_rp::peripherals::PIN_10> = Input::new(p.PIN_10, Pull::Up);
    // Led that will be lit on delay screen
    let mut led = Output::new(p.PIN_7, Level::Low);

    // OLED display connecter to GPIO 4/5
    let sda = p.PIN_4;
    let scl = p.PIN_5;
    let i2c = i2c::I2c::new_blocking(p.I2C0, scl, sda, Config::default());
    let interface = I2CDisplayInterface::new(i2c);
    let mut display =
        Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0).into_terminal_mode();

    // Initial and clear OLED display
    display.init().unwrap();
    let _ = display.clear();

    let mut display_screen = DisplayScreen::Characters;
    let mut generator = char_generator::CharGenerator::new();
    /* Endless loop*/
    loop {
        // Compute delay from potentiometer reading (max = 1000 ms)
        let potentiometer_reading = adc.read(&mut potentiometer_ch).await.unwrap();
        let delay = potentiometer_reading as u64 * 1000 / 4096;

        // Read the button to find if we want to see the delay.
        // Clear the display if the screen change.
        if button.is_high() {
            if matches!(display_screen, DisplayScreen::Characters) {
                let _ = display.clear();
            }
            display_screen = DisplayScreen::Delay;
            led.set_high();
        } else if matches!(display_screen, DisplayScreen::Delay) {
            let _ = display.clear();
            display_screen = DisplayScreen::Characters;
            led.set_low();
        }

        // Display either the characters or the delay screen
        match display_screen {
            DisplayScreen::Delay => {
                let _ = display.set_position(0, 0);
                let _ = write!(display, "Delay: {} ms     ", delay);
            }
            DisplayScreen::Characters => {
                let _ = display.print_char(generator.next() as char);
            }
        }

        Timer::after_millis(delay).await;
    }
}
