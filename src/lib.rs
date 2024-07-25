#![no_std]

use embedded_hal::i2c;
use crate::config::Config;
use crate::register_address::Register;

mod config;
mod register_address;

#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2C(E),
}

/// 顺序模式地址
const DEVICE_BASE_SEQ_ADDRESS: u8 = 0b10001;
/// 随机模式地址
const DEVICE_BASE_RANDOM_ADDRESS: u8 = 0b10001;

#[derive(Debug, )]
pub struct Rda5708m<I2C> {
    pub(crate) i2c: I2C,
    pub(crate) address: u8,
    pub(crate) config: Config,
}

impl<I2C, E> Rda5708m<I2C>
where
    I2C: i2c::I2c<Error=E>,
{
    pub fn new<A: Into<Address>>(i2c: I2C, address: A) -> Self <> {
        let a = address.into();
        Rda5708m {
            i2c,
            address: a.0,
            config: Config::default(),
        }
    }

    fn write_register(&mut self, register: u8, data: u16) -> Result<(), Error<E>> {
        self.i2c.write(self.address,
                       &[register,
                           ((data & 0xff00) >> 8) as u8,
                           (data & 0x00ff) as u8
                       ])
            .map_err(Error::I2C)
    }

    fn read_register(&mut self, register: u8) -> Result<u16, Error<E>> {
        let mut data = [0; 2];
        self.i2c.write_read(self.address,
                            &[register],
                            &mut data)
            .map_err(Error::I2C)?;
        Ok(((data[0] as u16) << 8) | data[1] as u16)
    }

    fn update_register(&mut self, register: u8, mask: u16, new_value: u16) -> Result<(), Error<E>> {
        let data = self.read_register(register)?;
        let data = data & !mask | new_value;
        Ok(self.write_register(register, data)?)
    }

    pub fn start(&mut self) -> Result<(), Error<E>> {
        let config = self.config;
        self.write_register(Register::RDA5807M_REG_CONFIG, config.config)?;
        self.write_register(Register::RDA5807M_REG_TUNING, config.tuning)?;
        Ok(())
    }

    pub fn end(&mut self) -> Result<(), Error<E>> {
        self.write_register(Register::RDA5807M_REG_CONFIG, 0x0)?;
        Ok(())
    }

    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }
}

/// I2C device address
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Address(pub(crate) u8);

impl Default for Address {
    fn default() -> Self {
        Address(DEVICE_BASE_RANDOM_ADDRESS)
    }
}

impl From<u8> for Address {
    fn from(value: u8) -> Self {
        Address(value)
    }
}

impl Address {
    pub fn seq() -> Self {
        Address(DEVICE_BASE_SEQ_ADDRESS)
    }
}
