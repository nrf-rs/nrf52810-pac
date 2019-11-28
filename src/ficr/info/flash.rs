#[doc = "Reader of register FLASH"]
pub type R = crate::R<u32, super::FLASH>;
#[doc = "Flash variant\n\nValue on reset: 192"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_A {
    #[doc = "192: 192 kByte flash"]
    K192,
    #[doc = "4294967295: Unspecified"]
    UNSPECIFIED,
}
impl From<FLASH_A> for u32 {
    #[inline(always)]
    fn from(variant: FLASH_A) -> Self {
        match variant {
            FLASH_A::K192 => 192,
            FLASH_A::UNSPECIFIED => 4294967295,
        }
    }
}
#[doc = "Reader of field `FLASH`"]
pub type FLASH_R = crate::R<u32, FLASH_A>;
impl FLASH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, FLASH_A> {
        use crate::Variant::*;
        match self.bits {
            192 => Val(FLASH_A::K192),
            4294967295 => Val(FLASH_A::UNSPECIFIED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `K192`"]
    #[inline(always)]
    pub fn is_k192(&self) -> bool {
        *self == FLASH_A::K192
    }
    #[doc = "Checks if the value of the field is `UNSPECIFIED`"]
    #[inline(always)]
    pub fn is_unspecified(&self) -> bool {
        *self == FLASH_A::UNSPECIFIED
    }
}
impl R {
    #[doc = "Bits 0:31 - Flash variant"]
    #[inline(always)]
    pub fn flash(&self) -> FLASH_R {
        FLASH_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
