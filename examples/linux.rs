use linux_embedded_hal::I2cdev;
use rda5807m::{Address, Rda5708m};

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut rda5807m = Rda5708m::new(dev, Address::default());
    // start
    rda5807m.start().unwrap();
    // set volume
    rda5807m.set_volume(1).unwrap();
    // get freq
    let freq = rda5807m.get_frequency().unwrap();
    // set freq
    rda5807m.set_frequency(101700).unwrap();
    // seek up
    rda5807m.seek_up(true).unwrap();
    // stop
    rda5807m.stop().unwrap();
}
