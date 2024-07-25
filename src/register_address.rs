pub struct Register;

/// document: https://blog.csdn.net/m0_57585228/article/details/125940042
impl Register {
    pub const RDA5807M_REG_CHIPID: u8 = 0x00;
    pub const RDA5807M_REG_CONFIG: u8 = 0x02;
    pub const RDA5807M_REG_TUNING: u8 = 0x03;
    pub const RDA5807M_REG_GPIO: u8 = 0x04;
    pub const RDA5807M_REG_VOLUME: u8 = 0x05;
    pub const RDA5807M_REG_I2S: u8 = 0x06;
    pub const RDA5807M_REG_BLEND: u8 = 0x07;
    pub const RDA5807M_REG_FREQ: u8 = 0x08;
    pub const RDA5807M_REG_STATUS: u8 = 0x0A;
    pub const RDA5807M_REG_RSSI: u8 = 0x0B;
    pub const RDA5807M_REG_RDSA: u8 = 0x0C;
    pub const RDA5807M_REG_RDSB: u8 = 0x0D;
    pub const RDA5807M_REG_RDSC: u8 = 0x0E;
    pub const RDA5807M_REG_RDSD: u8 = 0x0F;
    pub const RDA5800_REG_LNA: u8 = 0x10;
    pub const RDA5807M_REG_SEEK: u8 = 0x20;
}

pub struct ConfigBitFlags;

impl ConfigBitFlags {
    /// 音频输出高阻。0：高阻，1：正常
    pub const DHIZ: u16 = 1 << 15;
    /// 静音。0：静音， 1：正常
    pub const DMUTE: u16 = 1 << 14;
    /// 单声道。0：立体声，1：单声道
    pub const MONO: u16 = 1 << 13;
    /// 增强低音。0：禁用，1：启用
    pub const BASS: u16 = 1 << 12;
    /// 晶振启用。0：总是启用，1：仅在FM工作时启用
    pub const RCLKNOCAL: u16 = 1 << 11;
    /// 晶振直接输入模式。0：正常，1：直接输入模式
    pub const RCLKDIRECT: u16 = 1 << 10;
    /// 向上搜索。0：向下搜索，1：向上搜索
    pub const SEEKUP: u16 = 1 << 9;
    /// 搜索启用。0：停止搜索，1：启用搜索
    pub const SEEK: u16 = 1 << 8;
    /// 搜索模式。0：达到边界处从另一边界开始搜索，1：到达边界处停止搜索
    pub const SKMODE: u16 = 1 << 7;
    /// RDS/RBDS启用。0：不启用，1：启用
    pub const RDS: u16 = 1 << 4;
    /// 使用新技术提高信号质量。0：不启用，1：启用
    pub const NEW: u16 = 1 << 3;
    /// 软件复位。0：正常，1：复位
    pub const RESET: u16 = 1 << 2;
    /// 上电启用。0：不启用，1：启用
    pub const ENABLE: u16 = 1;
}

pub struct ConfigClkBitFlag;

/// 晶振频率。
impl ConfigClkBitFlag {
    /// 000：32.768KHz
    pub const CLK32768K: u16 = 0b0000_0000_0000_0000;
    /// 001：12MHz
    pub const CLK12M: u16 = 0b0000_0000_0001_0000;
    /// 101：24MHz
    pub const CLK24M: u16 = 0b0000_0000_0101_0000;
    /// 010：13MHz
    pub const CLK13M: u16 = 0b0000_0000_0010_0000;
    /// 110：26MHz
    pub const CLK26M: u16 = 0b0000_0000_0110_0000;
    /// 011：26MHz
    pub const CLK192M: u16 = 0b0000_0000_0011_0000;
    /// 111：38.4MHz
    pub const CLK384M: u16 = 0b0000_0000_0111_0000;
}

pub struct TuningBitFlag;

impl TuningBitFlag {

}

#[cfg(test)]
mod tests {
    use crate::register_address::{ConfigBitFlags, ConfigClkBitFlag};

    #[test]
    fn default_config() {
        assert_eq!(0b0000_0000_0000_0001, ConfigBitFlags::ENABLE);
        assert_eq!(0b0000_0000_0111_0000, ConfigClkBitFlag::CLK384M);
        assert_eq!(0b0000_0000_0000_0000, ConfigClkBitFlag::CLK32768K);
    }
}