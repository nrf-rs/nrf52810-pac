#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1536usize],
    #[doc = "0x600 - Block protect configuration register 0"]
    pub config0: CONFIG0,
    #[doc = "0x604 - Block protect configuration register 1"]
    pub config1: CONFIG1,
    #[doc = "0x608 - Disable protection mechanism in debug mode"]
    pub disableindebug: DISABLEINDEBUG,
}
#[doc = "Block protect configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [config0](config0) module"]
pub type CONFIG0 = crate::Reg<u32, _CONFIG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG0;
#[doc = "`read()` method returns [config0::R](config0::R) reader structure"]
impl crate::Readable for CONFIG0 {}
#[doc = "`write(|w| ..)` method takes [config0::W](config0::W) writer structure"]
impl crate::Writable for CONFIG0 {}
#[doc = "Block protect configuration register 0"]
pub mod config0;
#[doc = "Block protect configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [config1](config1) module"]
pub type CONFIG1 = crate::Reg<u32, _CONFIG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG1;
#[doc = "`read()` method returns [config1::R](config1::R) reader structure"]
impl crate::Readable for CONFIG1 {}
#[doc = "`write(|w| ..)` method takes [config1::W](config1::W) writer structure"]
impl crate::Writable for CONFIG1 {}
#[doc = "Block protect configuration register 1"]
pub mod config1;
#[doc = "Disable protection mechanism in debug mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [disableindebug](disableindebug) module"]
pub type DISABLEINDEBUG = crate::Reg<u32, _DISABLEINDEBUG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DISABLEINDEBUG;
#[doc = "`read()` method returns [disableindebug::R](disableindebug::R) reader structure"]
impl crate::Readable for DISABLEINDEBUG {}
#[doc = "`write(|w| ..)` method takes [disableindebug::W](disableindebug::W) writer structure"]
impl crate::Writable for DISABLEINDEBUG {}
#[doc = "Disable protection mechanism in debug mode"]
pub mod disableindebug;
