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

#![no_std]

extern crate cortex_m;
extern crate embedded_graphics;
extern crate embedded_hal;
extern crate ssd1306;
extern crate panic_abort;
extern crate max31865;
extern crate stm32f103xx_hal as hal;

use hal::i2c::{DutyCycle, I2c, Mode};
use hal::spi::Spi;
use hal::prelude::*;
use ssd1306::{Builder, mode::TerminalMode};
use ssd1306::prelude::*;
use core::fmt::Write;
use max31865::{Max31865, SensorType, FilterMode};

fn main() {
    let dp = hal::stm32f103xx::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);
    let gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    // I2C pins
    let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);

    let i2c = I2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Fast {
            frequency: 400_000,
            duty_cycle: DutyCycle::Ratio1to1,
        },
        clocks,
        &mut rcc.apb1,
    );


    let mut disp: TerminalMode<_> = Builder::new().with_size(DisplaySize::Display128x32).connect_i2c(i2c).into();
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
    max31865.set_calibration(43234).unwrap();
    max31865.configure(true, true, false, SensorType::ThreeWire, FilterMode::Filter50Hz).unwrap();

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