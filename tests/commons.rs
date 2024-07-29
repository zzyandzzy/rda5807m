use embedded_hal_mock::eh0::i2c::{Mock as I2cMock, Transaction as I2cTransaction};
use rda5807m::{Address, Rda5708m};

// 随机模式地址
pub const DEVICE_BASE_RANDOM_ADDRESS: u8 = 0b10001;

pub fn new(expectations: &[I2cTransaction]) -> Rda5708m<I2cMock> {
    let i2c = I2cMock::new(expectations);
    Rda5708m::new(i2c, Address::default())
}

pub fn destroy(rda5807m: Rda5708m<I2cMock>) {
    rda5807m.destroy().done();
}
