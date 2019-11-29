#[doc = "Part code\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [part](part) module"]
pub type PART = crate::Reg<u32, _PART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PART;
#[doc = "`read()` method returns [part::R](part::R) reader structure"]
impl crate::Readable for PART {}
#[doc = "Part code"]
pub mod part;
#[doc = "Part variant, hardware version and production configuration\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [variant](variant) module"]
pub type VARIANT = crate::Reg<u32, _VARIANT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VARIANT;
#[doc = "`read()` method returns [variant::R](variant::R) reader structure"]
impl crate::Readable for VARIANT {}
#[doc = "Part variant, hardware version and production configuration"]
pub mod variant;
#[doc = "Package option\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [package](package) module"]
pub type PACKAGE = crate::Reg<u32, _PACKAGE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PACKAGE;
#[doc = "`read()` method returns [package::R](package::R) reader structure"]
impl crate::Readable for PACKAGE {}
#[doc = "Package option"]
pub mod package;
#[doc = "RAM variant\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ram](ram) module"]
pub type RAM = crate::Reg<u32, _RAM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAM;
#[doc = "`read()` method returns [ram::R](ram::R) reader structure"]
impl crate::Readable for RAM {}
#[doc = "RAM variant"]
pub mod ram;
#[doc = "Flash variant\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flash](flash) module"]
pub type FLASH = crate::Reg<u32, _FLASH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH;
#[doc = "`read()` method returns [flash::R](flash::R) reader structure"]
impl crate::Readable for FLASH {}
#[doc = "Flash variant"]
pub mod flash;
