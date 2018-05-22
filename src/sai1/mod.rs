#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x04 - SAI AConfiguration register 1"]
    pub sai_acr1: SAI_ACR1,
    #[doc = "0x08 - SAI AConfiguration register 2"]
    pub sai_acr2: SAI_ACR2,
    #[doc = "0x0c - SAI AFrame configuration register"]
    pub sai_afrcr: SAI_AFRCR,
    #[doc = "0x10 - SAI ASlot register"]
    pub sai_aslotr: SAI_ASLOTR,
    #[doc = "0x14 - SAI AInterrupt mask register2"]
    pub sai_aim: SAI_AIM,
    #[doc = "0x18 - SAI AStatus register"]
    pub sai_asr: SAI_ASR,
    #[doc = "0x1c - SAI AClear flag register"]
    pub sai_aclrfr: SAI_ACLRFR,
    #[doc = "0x20 - SAI AData register"]
    pub sai_adr: SAI_ADR,
    #[doc = "0x24 - SAI BConfiguration register 1"]
    pub sai_bcr1: SAI_BCR1,
    #[doc = "0x28 - SAI BConfiguration register 2"]
    pub sai_bcr2: SAI_BCR2,
    #[doc = "0x2c - SAI BFrame configuration register"]
    pub sai_bfrcr: SAI_BFRCR,
    #[doc = "0x30 - SAI BSlot register"]
    pub sai_bslotr: SAI_BSLOTR,
    #[doc = "0x34 - SAI BInterrupt mask register2"]
    pub sai_bim: SAI_BIM,
    #[doc = "0x38 - SAI BStatus register"]
    pub sai_bsr: SAI_BSR,
    #[doc = "0x3c - SAI BClear flag register"]
    pub sai_bclrfr: SAI_BCLRFR,
    #[doc = "0x40 - SAI BData register"]
    pub sai_bdr: SAI_BDR,
}
#[doc = "SAI AConfiguration register 1"]
pub struct SAI_ACR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SAI AConfiguration register 1"]
pub mod sai_acr1;
#[doc = "SAI BConfiguration register 1"]
pub struct SAI_BCR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SAI BConfiguration register 1"]
pub mod sai_bcr1;
#[doc = "SAI AConfiguration register 2"]
pub struct SAI_ACR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SAI AConfiguration register 2"]
pub mod sai_acr2;
#[doc = "SAI BConfiguration register 2"]
pub struct SAI_BCR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SAI BConfiguration register 2"]
pub mod sai_bcr2;
#[doc = "SAI AFrame configuration register"]
pub struct SAI_AFRCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SAI AFrame configuration register"]
pub mod sai_afrcr;
#[doc = "SAI BFrame configuration register"]
pub struct SAI_BFRCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SAI BFrame configuration register"]
pub mod sai_bfrcr;
#[doc = "SAI ASlot register"]
pub struct SAI_ASLOTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SAI ASlot register"]
pub mod sai_aslotr;
#[doc = "SAI BSlot register"]
pub struct SAI_BSLOTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SAI BSlot register"]
pub mod sai_bslotr;
#[doc = "SAI AInterrupt mask register2"]
pub struct SAI_AIM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SAI AInterrupt mask register2"]
pub mod sai_aim;
#[doc = "SAI BInterrupt mask register2"]
pub struct SAI_BIM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SAI BInterrupt mask register2"]
pub mod sai_bim;
#[doc = "SAI AStatus register"]
pub struct SAI_ASR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SAI AStatus register"]
pub mod sai_asr;
#[doc = "SAI BStatus register"]
pub struct SAI_BSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SAI BStatus register"]
pub mod sai_bsr;
#[doc = "SAI AClear flag register"]
pub struct SAI_ACLRFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SAI AClear flag register"]
pub mod sai_aclrfr;
#[doc = "SAI BClear flag register"]
pub struct SAI_BCLRFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SAI BClear flag register"]
pub mod sai_bclrfr;
#[doc = "SAI AData register"]
pub struct SAI_ADR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SAI AData register"]
pub mod sai_adr;
#[doc = "SAI BData register"]
pub struct SAI_BDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SAI BData register"]
pub mod sai_bdr;
