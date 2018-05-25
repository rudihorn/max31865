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
//! ```
//! 
//! #![no_std]
//! #![no_main]
//! 
//! extern crate cortex_m;
//! #[macro_use]
//! extern crate cortex_m_rt as rt;
//! extern crate panic_abort;
//! extern crate max31865;
//! extern crate embedded_hal as hal;
//! extern crate stm32f103xx_hal as dev_hal;
//! 
//! use dev_hal::spi::Spi;
//! use dev_hal::prelude::*;
//! use rt::ExceptionFrame;
//! use max31865::{Max31865, SensorType, FilterMode};
//! 
//! entry!(main);
//! 
//! fn main() -> ! {
//!     let dp = dev_hal::stm32f103xx::Peripherals::take().unwrap();
//!     let mut flash = dp.FLASH.constrain();
//!     let mut rcc = dp.RCC.constrain();
//!     let clocks = rcc.cfgr.freeze(&mut flash.acr);
//!     let gpioa = dp.GPIOA.split(&mut rcc.apb2);
//!     let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);
//! 
//!     let nss = gpiob.pb12.into_push_pull_output(&mut gpiob.crh);
//!     let sck = gpiob.pb13.into_alternate_push_pull(&mut gpiob.crh);
//!     let miso = gpiob.pb14;
//!     let mosi = gpiob.pb15.into_alternate_push_pull(&mut gpiob.crh);
//!     let rdy = gpioa.pa8;
//! 
//!     let spi1 = Spi::spi2(
//!         dp.SPI2,
//!         (sck, miso, mosi),
//!         max31865::MODE,
//!         100_000.hz(),
//!         clocks,
//!         &mut rcc.apb1,
//!     );
//! 
//!     let mut max31865 = Max31865::new(spi1, nss, rdy).unwrap();
//!     max31865.set_calibration(43234).unwrap();
//!     max31865.configure(true, true, false, SensorType::ThreeWire, FilterMode::Filter50Hz).unwrap();
//! 
//!     let mut last = 0;
//! 
//!     loop {
//!         if max31865.is_ready().unwrap() {
//!             let temp = max31865.read_default_conversion().unwrap();
//! 
//!             if temp != last {
//!                 last = temp;
//!                 // temperature in Celcius = temp / 100
//!             }
//!         }
//! 
//!     }
//! }
//! 
//! exception!(HardFault, hard_fault);
//! 
//! fn hard_fault(ef: &ExceptionFrame) -> ! {
//!     panic!("{:#?}", ef)
//! }
//! 
//! exception!(*, default_handler);
//! 
//! fn default_handler(irqn: i16) {
//!     panic!("Unhandled exception (IRQn = {})", irqn);
//! }
//! ```
// Auto-generated. Do not modify.
