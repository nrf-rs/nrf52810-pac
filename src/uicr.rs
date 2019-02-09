#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 20usize],
    #[doc = "0x14 - Description collection: Reserved for Nordic firmware design"]
    pub nrffw: [NRFFW; 15],
    #[doc = "0x50 - Description collection: Reserved for Nordic hardware design"]
    pub nrfhw: [NRFHW; 12],
    #[doc = "0x80 - Description collection: Reserved for customer"]
    pub customer: [CUSTOMER; 32],
    _reserved1: [u8; 256usize],
    #[doc = "0x200 - Description collection: Mapping of the nRESET function (see POWER chapter for details)"]
    pub pselreset: [PSELRESET; 2],
    #[doc = "0x208 - Access port protection"]
    pub approtect: APPROTECT,
}
#[doc = "Description collection: Reserved for Nordic firmware design"]
pub struct NRFFW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection: Reserved for Nordic firmware design"]
pub mod nrffw;
#[doc = "Description collection: Reserved for Nordic hardware design"]
pub struct NRFHW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection: Reserved for Nordic hardware design"]
pub mod nrfhw;
#[doc = "Description collection: Reserved for customer"]
pub struct CUSTOMER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection: Reserved for customer"]
pub mod customer;
#[doc = "Description collection: Mapping of the nRESET function (see POWER chapter for details)"]
pub struct PSELRESET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection: Mapping of the nRESET function (see POWER chapter for details)"]
pub mod pselreset;
#[doc = "Access port protection"]
pub struct APPROTECT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access port protection"]
pub mod approtect;
