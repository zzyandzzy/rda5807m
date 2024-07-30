pub struct Register;

/// document:
/// https://atta.szlcsc.com/upload/public/pdf/source/20190304/C82537_BECFDFEE4CC96E1FC10CC52133444FD5.pdf
/// https://blog.csdn.net/m0_57585228/article/details/125940042
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
}

pub struct ConfigBitFlags;

impl ConfigBitFlags {
    // 音频输出高阻。0：高阻，1：正常
    pub const DHIZ: u16 = 1 << 15;
    // 静音。0：静音， 1：正常
    pub const DMUTE: u16 = 1 << 14;
    // 单声道。0：立体声，1：单声道
    pub const MONO: u16 = 1 << 13;
    // 增强低音。0：禁用，1：启用
    pub const BASS: u16 = 1 << 12;
    // 晶振启用。0：总是启用，1：仅在FM工作时启用
    pub const RCLKNOCAL: u16 = 1 << 11;
    // 晶振直接输入模式。0：正常，1：直接输入模式
    pub const RCLKDIRECT: u16 = 1 << 10;
    // 向上搜索。0：向下搜索，1：向上搜索
    pub const SEEKUP: u16 = 1 << 9;
    // 搜索启用。0：停止搜索，1：启用搜索
    // Seek begins in the direction specified by
    // SEEKUP and ends when a channel is found,
    // or the entire band has been searched.
    // The SEEK bit is set low and the STC bit is set
    // high when the seek operation completes.
    pub const SEEK: u16 = 1 << 8;
    // 搜索模式。0：达到边界处从另一边界开始搜索，1：到达边界处停止搜索
    pub const SKMODE: u16 = 1 << 7;

    // 晶振频率。
    // 000：32.768KHz
    pub const CLK_32768_KHZ: u16 = 0b0000_0000_0000_0000;
    // 001：12MHz
    pub const CLK_12_MHZ: u16 = 0b0000_0000_0001_0000;
    // 101：24MHz
    pub const CLK_24_MHZ: u16 = 0b0000_0000_0101_0000;
    // 010：13MHz
    pub const CLK_13_MHZ: u16 = 0b0000_0000_0010_0000;
    // 110：26MHz
    pub const CLK_26_MHZ: u16 = 0b0000_0000_0110_0000;
    // 011：26MHz
    pub const CLK_192_MHZ: u16 = 0b0000_0000_0011_0000;
    // 111：38.4MHz
    pub const CLK_384_MHZ: u16 = 0b0000_0000_0111_0000;

    // RDS/RBDS启用。0：不启用，1：启用
    pub const RDS: u16 = 1 << 3;
    // 使用新技术提高信号质量。0：不启用，1：启用
    pub const NEW: u16 = 1 << 2;
    // 软件复位。0：正常，1：复位
    pub const RESET: u16 = 1 << 1;
    // 上电启用。0：不启用，1：启用
    pub const ENABLE: u16 = 1;
}

pub struct TuningBitFlag;

impl TuningBitFlag {
    // Channel Select.
    // BAND = 0
    // Frequency =
    // Channel Spacing (kHz) x CHAN+ 87.0 MHz
    // BAND = 1or 2
    // Frequency =
    // Channel Spacing (kHz) x CHAN + 76.0 MHz
    // BAND = 3
    // Frequency =
    // Channel Spacing (kHz) x CHAN + 65.0 MHz
    // CHAN is updated after a seek operation.
    pub const CHAN_MASK: u16 = 0b1111_1111_1100_0000;
    pub const CHAN_SHIFT: u8 = 6;
    // 调谐。0：禁用，1：启用
    // The tune operation begins when the TUNE bit
    // is set high. The STC bit is set high when the
    // tune operation completes.
    // The tune bit is reset to low automatically when
    // the tune operation completes.
    pub const TUNE: u16 = 1 << 4;
    // 波段
    pub const BAND_MASK: u16 = 0b0000_0000_0000_1100;
    pub const BAND_SHIFT: u8 = 2;
    // 00: 87-108MHz(Us/Europe)
    pub const BAND_87_108_MHZ: u16 = 0b0000_0000_0000_0000;
    // 01: 76-91MHz(Japan)
    pub const BAND_76_91_MHZ: u16 = 0b0000_0000_0000_0100;
    // 10: 76-108MHz(world wide)
    pub const BAND_76_108_MHZ: u16 = 0b0000_0000_0000_1000;
    // 11: 65-76MHz(East Europe) or 50-65MHz
    pub const BAND_65_76_MHZ: u16 = 0b0000_0000_0000_1100;
    // 频率间隔
    pub const SPACE_MASK: u16 = 0b0000_0000_0000_0011;
    // 00: 100kHz
    pub const SPACE_100_KHZ: u16 = 0b0000_0000_0000_0000;
    // 01: 200kHz
    pub const SPACE_200_KHZ: u16 = 0b0000_0000_0000_0001;
    // 10: 50kHz
    pub const SPACE_50_KHZ: u16 = 0b0000_0000_0000_0010;
    // 11: 25kHz
    pub const SPACE_25_KHZ: u16 = 0b0000_0000_0000_0011;
}

pub struct VolumeBitFlag;

impl VolumeBitFlag {
    // If 0, generate 5ms interrupt;
    // If 1, interrupt last until read reg0CH action
    // occurs.
    pub const INT_MODE: u16 = 1 << 15;
    // Default value is 00; When = 10, will add the
    // RSSI seek mode.
    pub const SEEK_MODE_MASK: u16 = 0b0110_0000_0000_0000;
    pub const SEEK_MODE_SHIFT: u8 = 13;
    pub const RSVD: u16 = 1 << 12;
    // Seek SNR threshold value
    pub const SEEK_TH_MASK: u16 = 0b0000_1111_0000_0000;
    pub const SEEK_TH_SHIFT: u8 = 8;
    // LNA input port selection bit:
    // 00: no input
    // 01: LNAN
    // 10: LNAP
    // 11: dual port input
    pub const LNA_PORT_SEL_MASK: u16 = 0b0000_0000_1100_0000;
    pub const LNA_PORT_SEL_SHIFT: u8 = 6;
    // Lna working current bit:
    // 00=1.8mA
    // 01=2.1mA
    // 10=2.5mA
    // 11=3.0mA
    pub const LNA_ICSEL_BIT_MASK: u16 = 0b0000_0000_0011_0000;
    pub const LNA_ICSEL_BIT_SHIFT: u8 = 4;
    // DAC Gain Control Bits (Volume).
    // 0000=min; 1111=max
    pub const VOLUME_MASK: u16 = 0b0000_0000_0000_1111;
}

#[derive(Debug, Default)]
pub struct VolumeRegister {
    pub int_mode: bool,
    pub seek_mode: u8,
    pub rsvd: bool,
    pub seek_th: u8,
    pub lna_port_sel: u8,
    pub lna_icsel: u8,
    pub volume: u8,
}

impl VolumeRegister {
    pub fn from_u16(data: u16) -> Self {
        VolumeRegister {
            int_mode: (data & VolumeBitFlag::INT_MODE) != 0,
            seek_mode: ((data & VolumeBitFlag::SEEK_MODE_MASK) >> VolumeBitFlag::SEEK_MODE_SHIFT)
                as u8,
            rsvd: (data & VolumeBitFlag::RSVD) != 0,
            seek_th: ((data & VolumeBitFlag::SEEK_TH_MASK) >> VolumeBitFlag::SEEK_TH_SHIFT) as u8,
            lna_port_sel: ((data & VolumeBitFlag::LNA_PORT_SEL_MASK)
                >> VolumeBitFlag::LNA_PORT_SEL_SHIFT) as u8,
            lna_icsel: ((data & VolumeBitFlag::LNA_ICSEL_BIT_MASK)
                >> VolumeBitFlag::LNA_ICSEL_BIT_SHIFT) as u8,
            volume: (data & VolumeBitFlag::VOLUME_MASK) as u8,
        }
    }
}

pub struct StatusBitFlag;

impl StatusBitFlag {
    // RDS ready
    // 0 = No RDS/RBDS group ready(default)
    // 1 = New RDS/RBDS group ready
    pub const RDSR: u16 = 1 << 15;
    // 调谐搜索。0: 没有完成，1: 完成
    // Seek/Tune Complete.
    // 0 = Not complete
    // 1 = Complete
    // The seek/tune complete flag is set when the
    // seek or tune operation completes.
    pub const STC: u16 = 1 << 14;
    // 搜索状态。0: 搜索成功，1: 搜索失败
    // Seek Fail.
    // 0 = Seek successful; 1 = Seek failure
    // The seek fail flag is set when the seek
    // operation fails to find a channel with an RSSI
    // level greater than SEEKTH[3:0].
    pub const SF: u16 = 1 << 13;
    // RDS Synchronization
    // 0 = RDS decoder not synchronized(default)
    // 1 = RDS decoder synchronized
    // Available only in RDS Verbose mode
    pub const RDDS: u16 = 1 << 12;
    // When RDS enable:
    // 1 = Block E has been found
    // 0 = no Block E has been found
    pub const BLK_E: u16 = 1 << 11;
    // Stereo Indicator.
    // 0 = Mono; 1 = Stereo
    pub const ST: u16 = 1 << 10;
    // 信道值
    // 频率计算方法：
    // 如果BAND=00，Frequency = Channel Spacing(kHz) x READCHAN[9:0] + 87MHz
    // 如果BAND=01 or BAND=10，Frequency = Channel Spacing(kHz) x READCHAN[9:0] + 76MHz
    // 如果BAND=11，Frequency = Channel Spacing(kHz) x READCHAN[9:0] + 65MHz
    pub const READ_CHAN_MASK: u16 = 0b0000_0011_1111_1111;
}

#[derive(Debug, Default)]
pub struct StatusRegister {
    pub rdsr: bool,
    pub stc: bool,
    pub sf: bool,
    pub rdss: bool,
    pub blk_e: bool,
    pub st: bool,
    pub readchan: u16,
}

impl StatusRegister {
    pub fn from_u16(data: u16) -> Self {
        StatusRegister {
            rdsr: (data & StatusBitFlag::RDSR) != 0,
            stc: (data & StatusBitFlag::STC) != 0,
            sf: (data & StatusBitFlag::SF) != 0,
            rdss: (data & StatusBitFlag::RDDS) != 0,
            blk_e: (data & StatusBitFlag::BLK_E) != 0,
            st: (data & StatusBitFlag::ST) != 0,
            readchan: data & StatusBitFlag::READ_CHAN_MASK,
        }
    }
}

pub struct RssiBitFlag;

impl RssiBitFlag {
    // 信号强度Mask
    pub const RSSI_MASK: u16 = 0b1111_1110_0000_0000;
    pub const RSSI_SHIFT: u8 = 9;
}
