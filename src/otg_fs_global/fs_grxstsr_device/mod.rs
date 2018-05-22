#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::FS_GRXSTSR_DEVICE {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct EPNUMR {
    bits: u8,
}
impl EPNUMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BCNTR {
    bits: u16,
}
impl BCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `DPID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPIDR {
    #[doc = "DATA0"]
    DATA0,
    #[doc = "DATA1"]
    DATA1,
    #[doc = "DATA2"]
    DATA2,
    #[doc = "MDATA"]
    MDATA,
}
impl DPIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DPIDR::DATA0 => 0,
            DPIDR::DATA1 => 2,
            DPIDR::DATA2 => 1,
            DPIDR::MDATA => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DPIDR {
        match value {
            0 => DPIDR::DATA0,
            2 => DPIDR::DATA1,
            1 => DPIDR::DATA2,
            3 => DPIDR::MDATA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DATA0`"]
    #[inline]
    pub fn is_data0(&self) -> bool {
        *self == DPIDR::DATA0
    }
    #[doc = "Checks if the value of the field is `DATA1`"]
    #[inline]
    pub fn is_data1(&self) -> bool {
        *self == DPIDR::DATA1
    }
    #[doc = "Checks if the value of the field is `DATA2`"]
    #[inline]
    pub fn is_data2(&self) -> bool {
        *self == DPIDR::DATA2
    }
    #[doc = "Checks if the value of the field is `MDATA`"]
    #[inline]
    pub fn is_mdata(&self) -> bool {
        *self == DPIDR::MDATA
    }
}
#[doc = "Possible values of the field `PKTSTS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PKTSTSR {
    #[doc = "Global OUT NAK "]
    GONAK,
    #[doc = "OUT data packet received"]
    OUTRCV,
    #[doc = "OUT transfer completed"]
    OUTCMPL,
    #[doc = "SETUP transaction completed"]
    SETUPCMPL,
    #[doc = "SETUP data packet received"]
    SETUPRX,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PKTSTSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PKTSTSR::GONAK => 1,
            PKTSTSR::OUTRCV => 2,
            PKTSTSR::OUTCMPL => 3,
            PKTSTSR::SETUPCMPL => 4,
            PKTSTSR::SETUPRX => 6,
            PKTSTSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PKTSTSR {
        match value {
            1 => PKTSTSR::GONAK,
            2 => PKTSTSR::OUTRCV,
            3 => PKTSTSR::OUTCMPL,
            4 => PKTSTSR::SETUPCMPL,
            6 => PKTSTSR::SETUPRX,
            i => PKTSTSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `GONAK`"]
    #[inline]
    pub fn is_gonak(&self) -> bool {
        *self == PKTSTSR::GONAK
    }
    #[doc = "Checks if the value of the field is `OUTRCV`"]
    #[inline]
    pub fn is_outrcv(&self) -> bool {
        *self == PKTSTSR::OUTRCV
    }
    #[doc = "Checks if the value of the field is `OUTCMPL`"]
    #[inline]
    pub fn is_outcmpl(&self) -> bool {
        *self == PKTSTSR::OUTCMPL
    }
    #[doc = "Checks if the value of the field is `SETUPCMPL`"]
    #[inline]
    pub fn is_setupcmpl(&self) -> bool {
        *self == PKTSTSR::SETUPCMPL
    }
    #[doc = "Checks if the value of the field is `SETUPRX`"]
    #[inline]
    pub fn is_setuprx(&self) -> bool {
        *self == PKTSTSR::SETUPRX
    }
}
#[doc = r" Value of the field"]
pub struct FRMNUMR {
    bits: u8,
}
impl FRMNUMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Endpoint number"]
    #[inline]
    pub fn epnum(&self) -> EPNUMR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EPNUMR { bits }
    }
    #[doc = "Bits 4:14 - Byte count"]
    #[inline]
    pub fn bcnt(&self) -> BCNTR {
        let bits = {
            const MASK: u16 = 2047;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        BCNTR { bits }
    }
    #[doc = "Bits 15:16 - Data PID"]
    #[inline]
    pub fn dpid(&self) -> DPIDR {
        DPIDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 17:20 - Packet status"]
    #[inline]
    pub fn pktsts(&self) -> PKTSTSR {
        PKTSTSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 21:24 - Frame number"]
    #[inline]
    pub fn frmnum(&self) -> FRMNUMR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FRMNUMR { bits }
    }
}
