#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::OTG_FS_DTXFSTS3 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct INEPTFSAVR {
    bits: u16,
}
impl INEPTFSAVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - IN endpoint TxFIFO space available"]
    #[inline]
    pub fn ineptfsav(&self) -> INEPTFSAVR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        INEPTFSAVR { bits }
    }
}
