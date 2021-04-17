//! An example of reading the temperature and writing it to a screen.
//!
//! # Devices
//!
//! - SSD1306 OLED display via I2C
//! - Max31865 Temperature Sensor via SPI
//!
//! Connections
//!
//! SSD 1306
//! - PB8 : I2C SCK
//! - PB9 : I2C SDA
//!
//! MAX31865
//! - PB12 : Negated Slave Select
//! - PB13 : SPI Clock
//! - PB14 : MISO
//! - PB15 : MOSI
//! - PA8 : Ready Pin
//!
//! # Build instructions
//!
//! - Build using: `cargo build --example stm32_ssd1306 --target thumbv7m-none-eabi --release`


#![no_std]
#![no_main]

#[macro_use]
extern crate cortex_m_rt;
extern crate panic_halt;
extern crate stm32f1xx_hal as hal;

use core::fmt::Write;
use stm32f1xx_hal::{
    i2c::{BlockingI2c, DutyCycle, Mode},
    prelude::*,
    spi::Spi,
    stm32,
};
use max31865::{FilterMode, Max31865, SensorType};
use ssd1306::{mode::TerminalMode, Builder, I2CDIBuilder};

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);
    let gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    // I2C pins
    let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);

    let i2c = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Fast {
            frequency: 400_000.hz(),
            duty_cycle: DutyCycle::Ratio2to1,
        },
        clocks,
        &mut rcc.apb1,
        1000,
        10,
        1000,
        1000,
    );

    let interface = I2CDIBuilder::new().init(i2c);
    let mut disp: TerminalMode<_,_> = Builder::new().connect(interface).into();
    disp.init().unwrap();
    disp.clear().unwrap();

    disp.write_str("init temp sensor").unwrap();

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
    max31865.set_calibration(43234);
    max31865
        .configure(
            true,
            true,
            false,
            SensorType::ThreeWire,
            FilterMode::Filter50Hz,
        )
        .unwrap();

    let mut last = 0;

    loop {
        if max31865.is_ready().unwrap() {
            let temp = max31865.read_default_conversion().unwrap();

            if temp != last {
                last = temp;

                let n = temp / 100;
                let dec = temp % 100;

                disp.clear().unwrap();
                write!(disp, "Temp: {}.{}\n", n, dec).unwrap();
                disp.flush().unwrap();
            }
        }
    }
}
