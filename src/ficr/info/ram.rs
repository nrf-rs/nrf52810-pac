#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RAM {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `RAM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAMR {
    #[doc = "24 kByte RAM"]
    K24,
    #[doc = "Unspecified"]
    UNSPECIFIED,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl RAMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            RAMR::K24 => 24,
            RAMR::UNSPECIFIED => 4294967295,
            RAMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> RAMR {
        match value {
            24 => RAMR::K24,
            4294967295 => RAMR::UNSPECIFIED,
            i => RAMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `K24`"]
    #[inline]
    pub fn is_k24(&self) -> bool {
        *self == RAMR::K24
    }
    #[doc = "Checks if the value of the field is `UNSPECIFIED`"]
    #[inline]
    pub fn is_unspecified(&self) -> bool {
        *self == RAMR::UNSPECIFIED
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - RAM variant"]
    #[inline]
    pub fn ram(&self) -> RAMR {
        RAMR::_from({
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        })
    }
}
