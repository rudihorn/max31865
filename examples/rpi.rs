//! Raspberry Pi demo
//!
//! # Connections
//!
//! IMPORTANT: Do *not* use PIN24 / BCM8 / CE0 as the NCS pin
//!
//! - PIN1 = 3V3 = VCC
//! - PIN19 = BCM10 = MOSI (SDA)
//! - PIN21 = BCM9 = MISO (AD0)
//! - PIN23 = BCM11 = SCLK (SCL)
//! - PIN22 = BCM25 = NCS
//! - PIN15 = BCM22 = RDY
//! - PIN6 = GND = GND
//! 
//! for further reference check https://pinout.xyz/#

extern crate linux_embedded_hal as hal;
extern crate max31865;

use std::thread;
use std::time::Duration;

use max31865::Max31865;
use hal::spidev::{self, SpidevOptions};
use hal::{Delay, Pin, Spidev};
use hal::sysfs_gpio::Direction;

fn main() {
    let mut spi = Spidev::open("/dev/spidev0.0").unwrap();
    let options = SpidevOptions::new()
        .max_speed_hz(1_000_000)
        .mode(spidev::SPI_MODE_3)
        .build();
    spi.configure(&options).unwrap();

    let ncs = Pin::new(25);
    ncs.export().unwrap();
    while !ncs.is_exported() {}
    ncs.set_direction(Direction::Out).unwrap();
    ncs.set_value(1).unwrap();

    let rdy = Pin::new(20);
    rdy.export().unwrap();
    while !rdy.is_exported() {}
    rdy.set_direction(Direction::In).unwrap();
    rdy.set_value(1).unwrap();

    let mut max31865 = Max31865::new(spi, ncs, rdy);

    max31865.configure();

    
}
