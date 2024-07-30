#![no_std]

use crate::register_address::{
    ConfigBitFlags, Register, RssiBitFlag, StatusRegister, TuningBitFlag, VolumeBitFlag,
    VolumeRegister,
};

pub mod register_address;

#[derive(Debug)]
pub enum Error<E> {
    // I²C bus error
    I2C(E),
}

// 顺序模式地址
const DEVICE_BASE_SEQ_ADDRESS: u8 = 0b10001;
// 随机模式地址
const DEVICE_BASE_RANDOM_ADDRESS: u8 = 0b10001;
// 设备ID
const DEVICE_ID: u16 = 0x5804;

#[derive(Debug)]
pub struct Rda5708m<I2C> {
    pub(crate) i2c: I2C,
    pub(crate) address: u8,
}

impl<I2C, E> Rda5708m<I2C>
where
    I2C: embedded_hal::blocking::i2c::Write<Error = E>
        + embedded_hal::blocking::i2c::Read<Error = E>
        + embedded_hal::blocking::i2c::WriteRead<Error = E>,
{
    pub fn new<A: Into<Address>>(i2c: I2C, address: A) -> Self {
        let a = address.into();
        Rda5708m { i2c, address: a.0 }
    }

    fn write_register(&mut self, register: u8, data: u16) -> Result<(), Error<E>> {
        self.i2c
            .write(
                self.address,
                &[register, (data >> 8) as u8, (data & 0xff) as u8],
            )
            .map_err(Error::I2C)
    }

    fn read_register(&mut self, register: u8) -> Result<u16, Error<E>> {
        let mut data = [0; 2];
        self.i2c
            .write_read(self.address, &[register], &mut data)
            .map_err(Error::I2C)?;
        Ok(((data[0] as u16) << 8) | data[1] as u16)
    }

    fn update_register(
        &mut self,
        register: u8,
        mask: u16,
        new_mask_value: u16,
    ) -> Result<(), Error<E>> {
        let data = self.read_register(register)?;
        self.update_register_by_old(register, data, mask, new_mask_value)
    }

    fn update_register_by_old(
        &mut self,
        register: u8,
        old_value: u16,
        mask: u16,
        new_value: u16,
    ) -> Result<(), Error<E>> {
        // 提取new_value中的对应mask位置的值
        let new_mask_value = (new_value & mask) | (old_value & !mask);
        self.write_register(register, new_mask_value)
    }

    pub fn check_id(&mut self) -> Result<bool, Error<E>> {
        let id = self.read_register(Register::RDA5807M_REG_CHIPID)?;
        Ok(id == DEVICE_ID)
    }

    // Start the device
    pub fn start(&mut self) -> Result<(), Error<E>> {
        let config = ConfigBitFlags::DHIZ
            | ConfigBitFlags::DMUTE
            | ConfigBitFlags::BASS
            | ConfigBitFlags::SEEKUP
            | ConfigBitFlags::RDS
            | ConfigBitFlags::NEW
            | ConfigBitFlags::ENABLE;
        let tuning = TuningBitFlag::BAND_87_108_MHZ | TuningBitFlag::SPACE_100_KHZ;
        self.write_register(Register::RDA5807M_REG_CONFIG, config)?;
        self.write_register(Register::RDA5807M_REG_TUNING, tuning)
    }

    // Stop the device
    pub fn stop(&mut self) -> Result<(), Error<E>> {
        self.write_register(Register::RDA5807M_REG_CONFIG, 0x0)
    }

    // Destroy driver instance, return I²C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }

    // set device volume
    pub fn set_volume(&mut self, volume: u8) -> Result<(), Error<E>> {
        let mut volume = volume;
        if volume > 15 {
            volume = 15;
        }
        self.update_register(
            Register::RDA5807M_REG_VOLUME,
            VolumeBitFlag::VOLUME_MASK,
            volume as u16,
        )
    }

    pub fn get_volume(&mut self) -> Result<VolumeRegister, Error<E>> {
        let config = self.read_register(Register::RDA5807M_REG_VOLUME)?;
        Ok(VolumeRegister::from_u16(config))
    }

    // set device mute
    pub fn mute(&mut self, mute: bool) -> Result<(), Error<E>> {
        self.update_register(
            Register::RDA5807M_REG_CONFIG,
            ConfigBitFlags::DMUTE,
            if mute { ConfigBitFlags::DMUTE } else { 0 },
        )
    }

    /// 自动搜台信号阈值强度默认为8
    /// 数值越低搜到的台越多
    pub fn set_seek_threshold(&mut self, threshold: u8) -> Result<(), Error<E>> {
        self.update_register(
            Register::RDA5807M_REG_VOLUME,
            VolumeBitFlag::SEEK_TH_MASK,
            (threshold as u16) << VolumeBitFlag::SEEK_TH_SHIFT,
        )
    }

    // volume up
    pub fn volume_up(&mut self) -> Result<(), Error<E>> {
        let config = self.read_register(Register::RDA5807M_REG_VOLUME)?;
        let volume = config & VolumeBitFlag::VOLUME_MASK;
        let mut volume = volume + 1;
        if volume > 15 {
            volume = 15;
        }
        self.update_register_by_old(
            Register::RDA5807M_REG_VOLUME,
            config,
            VolumeBitFlag::VOLUME_MASK,
            volume,
        )
    }

    // volume down
    pub fn volume_down(&mut self) -> Result<(), Error<E>> {
        let config = self.read_register(Register::RDA5807M_REG_VOLUME)?;
        let volume = config & VolumeBitFlag::VOLUME_MASK;
        let mut volume = volume - 1;
        if volume > 15 {
            volume = 0;
        }
        self.update_register_by_old(
            Register::RDA5807M_REG_VOLUME,
            config,
            VolumeBitFlag::VOLUME_MASK,
            volume,
        )
    }

    pub fn seek_up(&mut self, wrap: bool) -> Result<(), Error<E>> {
        self.update_register(
            Register::RDA5807M_REG_CONFIG,
            ConfigBitFlags::SEEKUP | ConfigBitFlags::SEEK | ConfigBitFlags::SKMODE,
            ConfigBitFlags::SEEKUP
                | ConfigBitFlags::SEEK
                | if wrap { 0 } else { ConfigBitFlags::SKMODE },
        )
    }

    pub fn seek_down(&mut self, wrap: bool) -> Result<(), Error<E>> {
        self.update_register(
            Register::RDA5807M_REG_CONFIG,
            ConfigBitFlags::SEEKUP | ConfigBitFlags::SEEK | ConfigBitFlags::SKMODE,
            0 | ConfigBitFlags::SEEK | if wrap { 0 } else { ConfigBitFlags::SKMODE },
        )
    }

    pub fn get_rssi(&mut self) -> Result<u8, Error<E>> {
        let rssi = self.read_register(Register::RDA5807M_REG_RSSI)?;
        Ok(((rssi & RssiBitFlag::RSSI_MASK) >> RssiBitFlag::RSSI_SHIFT) as u8)
    }

    fn get_band_and_spacing(&mut self) -> Result<(u8, u8, u16), Error<E>> {
        let config = self.read_register(Register::RDA5807M_REG_TUNING)?;
        let band = (config & TuningBitFlag::BAND_MASK) >> TuningBitFlag::BAND_SHIFT;
        let spacing = config & TuningBitFlag::SPACE_MASK;
        Ok((band as u8, spacing as u8, config))
    }

    pub fn get_frequency(&mut self) -> Result<u32, Error<E>> {
        let (band, spacing, _) = self.get_band_and_spacing()?;
        let spacing: u32 = match spacing {
            0b00 => 100,
            0b01 => 200,
            0b10 => 50,
            0b11 => 25,
            _ => 0,
        };
        let chan = self.get_status()?.readchan as u32;
        let freq = match band {
            0b00 => 87_000 + spacing * chan,
            0b01 | 0b10 => 76_000 + spacing * chan,
            0b11 => 65_000 + spacing * chan,
            _ => 0,
        };
        Ok(freq)
    }

    pub fn set_frequency(&mut self, freq: u32) -> Result<(), Error<E>> {
        let (band, spacing, config) = self.get_band_and_spacing()?;
        let spacing: u32 = match spacing {
            0b00 => 100,
            0b01 => 200,
            0b10 => 50,
            0b11 => 25,
            _ => 0,
        };
        let chan = match band {
            0b00 => (freq - 87_000) / spacing,
            0b01 | 0b10 => (freq - 76_000) / spacing,
            0b11 => (freq - 65_000) / spacing,
            _ => 0,
        };

        let mask = TuningBitFlag::CHAN_MASK | TuningBitFlag::TUNE;
        let new_mask_value = ((chan as u16) << TuningBitFlag::CHAN_SHIFT) | TuningBitFlag::TUNE;
        self.update_register_by_old(Register::RDA5807M_REG_TUNING, config, mask, new_mask_value)
    }

    pub fn get_status(&mut self) -> Result<StatusRegister, Error<E>> {
        let status_flag = self.read_register(Register::RDA5807M_REG_STATUS)?;
        Ok(StatusRegister::from_u16(status_flag))
    }
}

// I2C device address
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
