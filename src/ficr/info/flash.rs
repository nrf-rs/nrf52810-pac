#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::FLASH {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `FLASH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHR {
    #[doc = "192 kByte flash"]
    K192,
    #[doc = "Unspecified"]
    UNSPECIFIED,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl FLASHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            FLASHR::K192 => 192,
            FLASHR::UNSPECIFIED => 4294967295,
            FLASHR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> FLASHR {
        match value {
            192 => FLASHR::K192,
            4294967295 => FLASHR::UNSPECIFIED,
            i => FLASHR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `K192`"]
    #[inline]
    pub fn is_k192(&self) -> bool {
        *self == FLASHR::K192
    }
    #[doc = "Checks if the value of the field is `UNSPECIFIED`"]
    #[inline]
    pub fn is_unspecified(&self) -> bool {
        *self == FLASHR::UNSPECIFIED
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - Flash variant"]
    #[inline]
    pub fn flash(&self) -> FLASHR {
        FLASHR::_from({
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        })
    }
}
