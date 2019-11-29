#[doc = "Reader of register VARIANT"]
pub type R = crate::R<u32, super::VARIANT>;
#[doc = "Part variant, hardware version and production configuration, encoded as ASCII\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VARIANT_A {
    #[doc = "1094795585: AAAA"]
    AAAA,
    #[doc = "1094795568: AAA0"]
    AAA0,
    #[doc = "1094795841: AABA"]
    AABA,
    #[doc = "1094795842: AABB"]
    AABB,
    #[doc = "1094795824: AAB0"]
    AAB0,
    #[doc = "1094796097: AACA"]
    AACA,
    #[doc = "1094796098: AACB"]
    AACB,
    #[doc = "1094796080: AAC0"]
    AAC0,
    #[doc = "4294967295: Unspecified"]
    UNSPECIFIED,
}
impl From<VARIANT_A> for u32 {
    #[inline(always)]
    fn from(variant: VARIANT_A) -> Self {
        match variant {
            VARIANT_A::AAAA => 1094795585,
            VARIANT_A::AAA0 => 1094795568,
            VARIANT_A::AABA => 1094795841,
            VARIANT_A::AABB => 1094795842,
            VARIANT_A::AAB0 => 1094795824,
            VARIANT_A::AACA => 1094796097,
            VARIANT_A::AACB => 1094796098,
            VARIANT_A::AAC0 => 1094796080,
            VARIANT_A::UNSPECIFIED => 4294967295,
        }
    }
}
#[doc = "Reader of field `VARIANT`"]
pub type VARIANT_R = crate::R<u32, VARIANT_A>;
impl VARIANT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, VARIANT_A> {
        use crate::Variant::*;
        match self.bits {
            1094795585 => Val(VARIANT_A::AAAA),
            1094795568 => Val(VARIANT_A::AAA0),
            1094795841 => Val(VARIANT_A::AABA),
            1094795842 => Val(VARIANT_A::AABB),
            1094795824 => Val(VARIANT_A::AAB0),
            1094796097 => Val(VARIANT_A::AACA),
            1094796098 => Val(VARIANT_A::AACB),
            1094796080 => Val(VARIANT_A::AAC0),
            4294967295 => Val(VARIANT_A::UNSPECIFIED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AAAA`"]
    #[inline(always)]
    pub fn is_aaaa(&self) -> bool {
        *self == VARIANT_A::AAAA
    }
    #[doc = "Checks if the value of the field is `AAA0`"]
    #[inline(always)]
    pub fn is_aaa0(&self) -> bool {
        *self == VARIANT_A::AAA0
    }
    #[doc = "Checks if the value of the field is `AABA`"]
    #[inline(always)]
    pub fn is_aaba(&self) -> bool {
        *self == VARIANT_A::AABA
    }
    #[doc = "Checks if the value of the field is `AABB`"]
    #[inline(always)]
    pub fn is_aabb(&self) -> bool {
        *self == VARIANT_A::AABB
    }
    #[doc = "Checks if the value of the field is `AAB0`"]
    #[inline(always)]
    pub fn is_aab0(&self) -> bool {
        *self == VARIANT_A::AAB0
    }
    #[doc = "Checks if the value of the field is `AACA`"]
    #[inline(always)]
    pub fn is_aaca(&self) -> bool {
        *self == VARIANT_A::AACA
    }
    #[doc = "Checks if the value of the field is `AACB`"]
    #[inline(always)]
    pub fn is_aacb(&self) -> bool {
        *self == VARIANT_A::AACB
    }
    #[doc = "Checks if the value of the field is `AAC0`"]
    #[inline(always)]
    pub fn is_aac0(&self) -> bool {
        *self == VARIANT_A::AAC0
    }
    #[doc = "Checks if the value of the field is `UNSPECIFIED`"]
    #[inline(always)]
    pub fn is_unspecified(&self) -> bool {
        *self == VARIANT_A::UNSPECIFIED
    }
}
impl R {
    #[doc = "Bits 0:31 - Part variant, hardware version and production configuration, encoded as ASCII"]
    #[inline(always)]
    pub fn variant(&self) -> VARIANT_R {
        VARIANT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
