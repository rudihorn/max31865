//! A generic driver for the MAX31865 RTD to Digital converter
//! 
//! # References
//! - Datasheet: https://datasheets.maximintegrated.com/en/ds/MAX31865.pdf

#![feature(unsize)]
#![no_std]

extern crate embedded_hal as hal;

use hal::digital::{InputPin, OutputPin};
use hal::spi::{Mode, Phase, Polarity};
use hal::blocking::spi;

use core::marker::Unsize;
use core::mem;

pub const MODE : Mode = Mode {
    phase: Phase::CaptureOnSecondTransition,
    polarity: Polarity::IdleHigh    
};

pub mod temp_conversion;

pub enum FilterMode {
    Filter60Hz = 0,
    Filter50Hz = 1
}

pub enum SensorType {
    TwoOrFourWire = 0,
    ThreeWire = 1,
}

pub struct Max31865<SPI, NCS, RDY> {
    spi: SPI,
    ncs: NCS,
    rdy: RDY,
    calibration: u32,
}


impl<E, SPI, NCS, RDY> Max31865<SPI, NCS, RDY>
where 
    SPI: spi::Write<u8, Error = E> + spi::Transfer<u8, Error = E>,
    NCS: OutputPin,
    RDY: InputPin
{
    pub fn new(
        spi: SPI,
        ncs: NCS,
        rdy: RDY,
    ) -> Result<Max31865<SPI, NCS, RDY>, E>
    {
        let default_calib = 40000;

        let max31865 = Max31865 {
            spi,
            ncs,
            rdy,
            calibration: default_calib, /* value in ohms multiplied by 100 */
        };

        Ok(max31865)
    }

    /// V_BIAS is required to correctly perform conversion
    /// Conversion mode: true to automatically perform conversion, otherwise normally off
    /// One Shot, only perform detection once 
    pub fn configure(&mut self, vbias: bool, conversion_mode: bool, one_shot: bool,
        sensor_type: SensorType, filter_mode: FilterMode) -> Result<(), E> {
        let conf : u8 = ((vbias as u8) << 7) |
            ((conversion_mode as u8) << 6) |
            ((one_shot as u8) << 5) |
            ((sensor_type as u8) << 4) | 
            (filter_mode as u8);

        self.write(Register::CONFIG, conf)?;

        Ok(())
    }

    pub fn set_calibration(&mut self, calib : u32) -> Result<(), E> {
        self.calibration = calib;
        Ok(())
    }

    pub fn read_default_conversion(&mut self) -> Result<u32, E> {
        let raw = self.read_raw()?;
        let ohms = ((raw >> 1) as u32 * self.calibration) >> 15;
        let temp = temp_conversion::lookup_temperature(ohms as u16);

        Ok(temp)
    }

    pub fn read_raw(&mut self) -> Result<u16, E> {
        let msb : u16 = self.read(Register::RTD_MSB)? as u16;
        let lsb : u16 = self.read(Register::RTD_LSB)? as u16;
        
        Ok((msb << 8) | lsb)
    }

    pub fn is_ready(&self) -> Result<bool, E> {
        Ok(self.rdy.is_high())
    }

    fn read(&mut self, reg: Register) -> Result<u8, E> {
        let buffer: [u8; 2] = self.read_many(reg)?;
        Ok(buffer[1])
    }

    fn read_many<B>(&mut self, reg: Register) -> Result<B, E> 
    where B: Unsize<[u8]>,
    {
        let mut buffer: B  = unsafe { mem::zeroed() };
        {
            let slice: &mut [u8] = &mut buffer;
            slice[0] = reg.read_address();
            self.ncs.set_low();
            self.spi.transfer(slice)?;
            self.ncs.set_high();
        }

        Ok(buffer)
    }

    fn write(&mut self, reg: Register, val: u8) -> Result<(), E> {
        self.ncs.set_low();
        self.spi.write(&[reg.write_address(), val])?;
        self.ncs.set_high();
        Ok(())
    }
}

#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[derive(Clone, Copy)]
enum Register {
    CONFIG = 0x00,
    RTD_MSB = 0x01,
    RTD_LSB = 0x02,
    HIGH_FAULT_THRESHOLD_MSB = 0x03,
    HIGH_FAULT_THRESHOLD_LSB = 0x04,
    LOW_FAULT_THRESHOLD_MSB = 0x05,
    LOW_FAULT_THRESHOLD_LSB = 0x06,
    FAULT_STATUS = 0x07
}

const R: u8 = 1 << 7;
const W: u8 = 0 << 7;

impl Register {
    fn read_address(&self) -> u8 {
        *self as u8 | R
    }

    fn write_address(&self) -> u8 {
        *self as u8 | W
    }
}