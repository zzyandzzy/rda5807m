mod commons;

use embedded_hal_mock::eh1::i2c::{Transaction as I2cTransaction};
use crate::commons::{ConfigBitFlags, destroy, DEVICE_BASE_RANDOM_ADDRESS, new, Register, TuningBitFlag};


#[test]
fn can_start() {
    let config_bit = ConfigBitFlags::DHIZ | ConfigBitFlags::DMUTE | ConfigBitFlags::BASS |
        ConfigBitFlags::SEEKUP | ConfigBitFlags::RDS | ConfigBitFlags::NEW | ConfigBitFlags::ENABLE;
    let tuning_bit = TuningBitFlag::BAND_76_108_MHZ;
    let expectations = [
        I2cTransaction::write(DEVICE_BASE_RANDOM_ADDRESS,
                              vec![Register::RDA5807M_REG_CONFIG,
                                   (config_bit >> 8) as u8, config_bit as u8]),
        I2cTransaction::write(DEVICE_BASE_RANDOM_ADDRESS,
                              vec![Register::RDA5807M_REG_TUNING,
                                   (tuning_bit >> 8) as u8, tuning_bit as u8]),
    ];
    let mut dev = new(&expectations);
    dev.start().unwrap();
    destroy(dev);
}

#[test]
fn can_end() {
    let expectations = [
        I2cTransaction::write(DEVICE_BASE_RANDOM_ADDRESS,
                              vec![Register::RDA5807M_REG_CONFIG, 0x0, 0x0])];
    let mut dev = new(&expectations);
    dev.end().unwrap();
    destroy(dev);
}