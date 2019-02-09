#[doc = r" Register block"]
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
#[doc = "Block protect configuration register 0"]
pub struct CONFIG0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Block protect configuration register 0"]
pub mod config0;
#[doc = "Block protect configuration register 1"]
pub struct CONFIG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Block protect configuration register 1"]
pub mod config1;
#[doc = "Disable protection mechanism in debug mode"]
pub struct DISABLEINDEBUG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Disable protection mechanism in debug mode"]
pub mod disableindebug;
