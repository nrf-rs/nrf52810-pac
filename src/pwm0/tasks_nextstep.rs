#[doc = "Writer for register TASKS_NEXTSTEP"]
pub type W = crate::W<u32, super::TASKS_NEXTSTEP>;
#[doc = "Register TASKS_NEXTSTEP `reset()`'s with value 0"]
impl crate::ResetValue for super::TASKS_NEXTSTEP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Steps by one value in the current sequence on all enabled channels if DECODER.MODE=NextStep. Does not cause PWM generation to start if not running.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TASKS_NEXTSTEP_AW {
    #[doc = "1: Trigger task"]
    TRIGGER,
}
impl From<TASKS_NEXTSTEP_AW> for bool {
    #[inline(always)]
    fn from(variant: TASKS_NEXTSTEP_AW) -> Self {
        match variant {
            TASKS_NEXTSTEP_AW::TRIGGER => true,
        }
    }
}
#[doc = "Write proxy for field `TASKS_NEXTSTEP`"]
pub struct TASKS_NEXTSTEP_W<'a> {
    w: &'a mut W,
}
impl<'a> TASKS_NEXTSTEP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TASKS_NEXTSTEP_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TASKS_NEXTSTEP_AW::TRIGGER)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Steps by one value in the current sequence on all enabled channels if DECODER.MODE=NextStep. Does not cause PWM generation to start if not running."]
    #[inline(always)]
    pub fn tasks_nextstep(&mut self) -> TASKS_NEXTSTEP_W {
        TASKS_NEXTSTEP_W { w: self }
    }
}
