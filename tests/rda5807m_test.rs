use embedded_hal_mock::eh0::i2c::Transaction as I2cTransaction;

use crate::commons::{destroy, new, DEVICE_BASE_RANDOM_ADDRESS};
use crate::register_address::{Register, TuningBitFlag};

mod commons;
mod register_address;

#[test]
fn can_start() {
    let config_bit = 0xd30d;
    let chan = 0x13f;
    let tuning_bit = chan << TuningBitFlag::CHAN_SHIFT;
    let expectations = [
        I2cTransaction::write(
            DEVICE_BASE_RANDOM_ADDRESS,
            vec![
                Register::RDA5807M_REG_CONFIG,
                (config_bit >> 8) as u8,
                config_bit as u8,
            ],
        ),
        I2cTransaction::write_read(
            DEVICE_BASE_RANDOM_ADDRESS,
            vec![Register::RDA5807M_REG_TUNING],
            vec![(tuning_bit >> 8) as u8, tuning_bit as u8],
        ),
        I2cTransaction::write(
            DEVICE_BASE_RANDOM_ADDRESS,
            vec![
                Register::RDA5807M_REG_TUNING,
                (tuning_bit >> 8) as u8,
                tuning_bit as u8,
            ],
        ),
    ];
    let mut dev = new(&expectations);
    dev.start().unwrap();
    destroy(dev);
}

#[test]
fn can_stop() {
    let expectations = [I2cTransaction::write(
        DEVICE_BASE_RANDOM_ADDRESS,
        vec![Register::RDA5807M_REG_CONFIG, 0x0, 0x0],
    )];
    let mut dev = new(&expectations);
    dev.stop().unwrap();
    destroy(dev);
}

#[test]
fn can_read_rssi() {
    let expectations = [I2cTransaction::write_read(
        DEVICE_BASE_RANDOM_ADDRESS,
        vec![Register::RDA5807M_REG_RSSI],
        vec![0xfe, 0x0],
    )];
    let mut dev = new(&expectations);
    let rssi = dev.get_rssi().unwrap();
    assert_eq!(rssi, 127);
    destroy(dev);
}

#[test]
fn can_read_freq() {
    let chan = 0x13f;
    let tuning_bit = chan << TuningBitFlag::CHAN_SHIFT
        | TuningBitFlag::TUNE
        | TuningBitFlag::BAND_87_108_MHZ
        | TuningBitFlag::SPACE_100_KHZ;
    let expectations = [
        I2cTransaction::write_read(
            DEVICE_BASE_RANDOM_ADDRESS,
            vec![Register::RDA5807M_REG_TUNING],
            vec![(tuning_bit >> 8) as u8, tuning_bit as u8],
        ),
        I2cTransaction::write_read(
            DEVICE_BASE_RANDOM_ADDRESS,
            vec![Register::RDA5807M_REG_STATUS],
            vec![(chan >> 8) as u8, chan as u8],
        ),
    ];
    let mut dev = new(&expectations);
    let freq = dev.get_frequency().unwrap();
    assert_eq!(freq, 118_900);
    destroy(dev);
}

#[test]
fn can_set_freq() {
    let tuning_bit = 0x13f << TuningBitFlag::CHAN_SHIFT
        | TuningBitFlag::TUNE
        | TuningBitFlag::BAND_87_108_MHZ
        | TuningBitFlag::SPACE_100_KHZ;
    let expectations = [
        I2cTransaction::write_read(
            DEVICE_BASE_RANDOM_ADDRESS,
            vec![Register::RDA5807M_REG_TUNING],
            vec![(tuning_bit >> 8) as u8, tuning_bit as u8],
        ),
        I2cTransaction::write(
            DEVICE_BASE_RANDOM_ADDRESS,
            vec![
                Register::RDA5807M_REG_TUNING,
                (tuning_bit >> 8) as u8,
                tuning_bit as u8,
            ],
        ),
    ];
    let mut dev = new(&expectations);
    dev.set_frequency(118_900).unwrap();
    destroy(dev);
}
