#[doc = "RXD data pointer"]
pub struct PTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RXD data pointer"]
pub mod ptr;
#[doc = "Maximum number of bytes in receive buffer"]
pub struct MAXCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Maximum number of bytes in receive buffer"]
pub mod maxcnt;
#[doc = "Number of bytes received in last granted transaction"]
pub struct AMOUNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Number of bytes received in last granted transaction"]
pub mod amount;
#[doc = "EasyDMA list type"]
pub struct LIST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EasyDMA list type"]
pub mod list;
