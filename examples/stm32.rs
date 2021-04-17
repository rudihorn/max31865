//! An example of reading the temperature.
//!
//! # Devices
//!
//! - SSD1306 OLED display via I2C
//!
//! Connections
//!
//! MAX31865
//! - PB12 : Negated Slave Select
//! - PB13 : SPI Clock
//! - PB14 : MISO
//! - PB15 : MOSI
//! - PA8 : Ready Pin!
//!
//! # Build instructions
//!
//! - Build using: `cargo build --example stm32 --target thumbv7m-none-eabi --release`

#![no_std]
#![no_main]

#[macro_use]
extern crate cortex_m_rt;
extern crate panic_halt;

use cortex_m_semihosting::hprintln;
use max31865::{FilterMode, Max31865, SensorType};
use stm32f1xx_hal::{pac, prelude::*, spi::Spi};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // Microcontroller setup
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // GPIO ports setup
    let gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    let nss = gpiob.pb12.into_push_pull_output(&mut gpiob.crh);
    let sck = gpiob.pb13.into_alternate_push_pull(&mut gpiob.crh);
    let miso = gpiob.pb14;
    let mosi = gpiob.pb15.into_alternate_push_pull(&mut gpiob.crh);
    let rdy = gpioa.pa8;

    let spi1 = Spi::spi2(
        dp.SPI2,
        (sck, miso, mosi),
        max31865::MODE,
        100_000.hz(),
        clocks,
        &mut rcc.apb1,
    );

    let mut max31865 = Max31865::new(spi1, nss, rdy).unwrap();

    // Optionally set the calibration reference resistance by specifying the
    // reference resistance in ohms multiplied by 100. See documentation for
    // `set_calibration` function.

    //  max31865.set_calibration(43234);

    max31865
        .configure(
            true,
            true,
            false,
            SensorType::TwoOrFourWire,
            FilterMode::Filter50Hz,
        )
        .unwrap();

    let mut last = 0;

    loop {
        if max31865.is_ready().unwrap() {
            let temp = max31865.read_default_conversion().unwrap();

            hprintln!("temp:{}.{:0>2}", temp / 100, (temp % 100).abs()).unwrap();

            if temp != last {
                last = temp;
                // The temperature value in Celsius is `temp / 100`.
            }
        }
    }
}
