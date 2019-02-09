#[doc = "Description cluster: RAMn power control register. The RAM size will vary depending on product variant, and the RAMn register will only be present if the corresponding RAM AHB slave is present on the device."]
pub struct POWER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description cluster: RAMn power control register. The RAM size will vary depending on product variant, and the RAMn register will only be present if the corresponding RAM AHB slave is present on the device."]
pub mod power;
#[doc = "Description cluster: RAMn power control set register"]
pub struct POWERSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description cluster: RAMn power control set register"]
pub mod powerset;
#[doc = "Description cluster: RAMn power control clear register"]
pub struct POWERCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description cluster: RAMn power control clear register"]
pub mod powerclr;
