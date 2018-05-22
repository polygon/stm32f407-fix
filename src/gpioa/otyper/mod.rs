#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OTYPER {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `OT15`"]
pub type OT15R = ::gpioi::otyper::OT15R;
#[doc = "Possible values of the field `OT14`"]
pub type OT14R = ::gpioi::otyper::OT15R;
#[doc = "Possible values of the field `OT13`"]
pub type OT13R = ::gpioi::otyper::OT15R;
#[doc = "Possible values of the field `OT12`"]
pub type OT12R = ::gpioi::otyper::OT15R;
#[doc = "Possible values of the field `OT11`"]
pub type OT11R = ::gpioi::otyper::OT15R;
#[doc = "Possible values of the field `OT10`"]
pub type OT10R = ::gpioi::otyper::OT15R;
#[doc = "Possible values of the field `OT9`"]
pub type OT9R = ::gpioi::otyper::OT15R;
#[doc = "Possible values of the field `OT8`"]
pub type OT8R = ::gpioi::otyper::OT15R;
#[doc = "Possible values of the field `OT7`"]
pub type OT7R = ::gpioi::otyper::OT15R;
#[doc = "Possible values of the field `OT6`"]
pub type OT6R = ::gpioi::otyper::OT15R;
#[doc = "Possible values of the field `OT5`"]
pub type OT5R = ::gpioi::otyper::OT15R;
#[doc = "Possible values of the field `OT4`"]
pub type OT4R = ::gpioi::otyper::OT15R;
#[doc = "Possible values of the field `OT3`"]
pub type OT3R = ::gpioi::otyper::OT15R;
#[doc = "Possible values of the field `OT2`"]
pub type OT2R = ::gpioi::otyper::OT15R;
#[doc = "Possible values of the field `OT1`"]
pub type OT1R = ::gpioi::otyper::OT15R;
#[doc = "Possible values of the field `OT0`"]
pub type OT0R = ::gpioi::otyper::OT15R;
#[doc = "Values that can be written to the field `OT15`"]
pub type OT15W = ::gpioi::otyper::OT15W;
#[doc = r" Proxy"]
pub struct _OT15W<'a> {
    w: &'a mut W,
}
impl<'a> _OT15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OT15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is used as push-pull output"]
    #[inline]
    pub fn push_pull(self) -> &'a mut W {
        self.variant(::gpioi::otyper::OT15W::PUSH_PULL)
    }
    #[doc = "Pin is used as open-drain"]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(::gpioi::otyper::OT15W::OPEN_DRAIN)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OT14`"]
pub type OT14W = ::gpioi::otyper::OT15W;
#[doc = r" Proxy"]
pub struct _OT14W<'a> {
    w: &'a mut W,
}
impl<'a> _OT14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OT14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is used as push-pull output"]
    #[inline]
    pub fn push_pull(self) -> &'a mut W {
        self.variant(::gpioi::otyper::OT15W::PUSH_PULL)
    }
    #[doc = "Pin is used as open-drain"]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(::gpioi::otyper::OT15W::OPEN_DRAIN)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OT13`"]
pub type OT13W = ::gpioi::otyper::OT15W;
#[doc = r" Proxy"]
pub struct _OT13W<'a> {
    w: &'a mut W,
}
impl<'a> _OT13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OT13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is used as push-pull output"]
    #[inline]
    pub fn push_pull(self) -> &'a mut W {
        self.variant(::gpioi::otyper::OT15W::PUSH_PULL)
    }
    #[doc = "Pin is used as open-drain"]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(::gpioi::otyper::OT15W::OPEN_DRAIN)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OT12`"]
pub type OT12W = ::gpioi::otyper::OT15W;
#[doc = r" Proxy"]
pub struct _OT12W<'a> {
    w: &'a mut W,
}
impl<'a> _OT12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OT12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is used as push-pull output"]
    #[inline]
    pub fn push_pull(self) -> &'a mut W {
        self.variant(::gpioi::otyper::OT15W::PUSH_PULL)
    }
    #[doc = "Pin is used as open-drain"]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(::gpioi::otyper::OT15W::OPEN_DRAIN)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OT11`"]
pub type OT11W = ::gpioi::otyper::OT15W;
#[doc = r" Proxy"]
pub struct _OT11W<'a> {
    w: &'a mut W,
}
impl<'a> _OT11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OT11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is used as push-pull output"]
    #[inline]
    pub fn push_pull(self) -> &'a mut W {
        self.variant(::gpioi::otyper::OT15W::PUSH_PULL)
    }
    #[doc = "Pin is used as open-drain"]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(::gpioi::otyper::OT15W::OPEN_DRAIN)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OT10`"]
pub type OT10W = ::gpioi::otyper::OT15W;
#[doc = r" Proxy"]
pub struct _OT10W<'a> {
    w: &'a mut W,
}
impl<'a> _OT10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OT10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is used as push-pull output"]
    #[inline]
    pub fn push_pull(self) -> &'a mut W {
        self.variant(::gpioi::otyper::OT15W::PUSH_PULL)
    }
    #[doc = "Pin is used as open-drain"]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(::gpioi::otyper::OT15W::OPEN_DRAIN)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OT9`"]
pub type OT9W = ::gpioi::otyper::OT15W;
#[doc = r" Proxy"]
pub struct _OT9W<'a> {
    w: &'a mut W,
}
impl<'a> _OT9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OT9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is used as push-pull output"]
    #[inline]
    pub fn push_pull(self) -> &'a mut W {
        self.variant(::gpioi::otyper::OT15W::PUSH_PULL)
    }
    #[doc = "Pin is used as open-drain"]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(::gpioi::otyper::OT15W::OPEN_DRAIN)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OT8`"]
pub type OT8W = ::gpioi::otyper::OT15W;
#[doc = r" Proxy"]
pub struct _OT8W<'a> {
    w: &'a mut W,
}
impl<'a> _OT8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OT8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is used as push-pull output"]
    #[inline]
    pub fn push_pull(self) -> &'a mut W {
        self.variant(::gpioi::otyper::OT15W::PUSH_PULL)
    }
    #[doc = "Pin is used as open-drain"]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(::gpioi::otyper::OT15W::OPEN_DRAIN)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OT7`"]
pub type OT7W = ::gpioi::otyper::OT15W;
#[doc = r" Proxy"]
pub struct _OT7W<'a> {
    w: &'a mut W,
}
impl<'a> _OT7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OT7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is used as push-pull output"]
    #[inline]
    pub fn push_pull(self) -> &'a mut W {
        self.variant(::gpioi::otyper::OT15W::PUSH_PULL)
    }
    #[doc = "Pin is used as open-drain"]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(::gpioi::otyper::OT15W::OPEN_DRAIN)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OT6`"]
pub type OT6W = ::gpioi::otyper::OT15W;
#[doc = r" Proxy"]
pub struct _OT6W<'a> {
    w: &'a mut W,
}
impl<'a> _OT6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OT6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is used as push-pull output"]
    #[inline]
    pub fn push_pull(self) -> &'a mut W {
        self.variant(::gpioi::otyper::OT15W::PUSH_PULL)
    }
    #[doc = "Pin is used as open-drain"]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(::gpioi::otyper::OT15W::OPEN_DRAIN)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OT5`"]
pub type OT5W = ::gpioi::otyper::OT15W;
#[doc = r" Proxy"]
pub struct _OT5W<'a> {
    w: &'a mut W,
}
impl<'a> _OT5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OT5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is used as push-pull output"]
    #[inline]
    pub fn push_pull(self) -> &'a mut W {
        self.variant(::gpioi::otyper::OT15W::PUSH_PULL)
    }
    #[doc = "Pin is used as open-drain"]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(::gpioi::otyper::OT15W::OPEN_DRAIN)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OT4`"]
pub type OT4W = ::gpioi::otyper::OT15W;
#[doc = r" Proxy"]
pub struct _OT4W<'a> {
    w: &'a mut W,
}
impl<'a> _OT4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OT4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is used as push-pull output"]
    #[inline]
    pub fn push_pull(self) -> &'a mut W {
        self.variant(::gpioi::otyper::OT15W::PUSH_PULL)
    }
    #[doc = "Pin is used as open-drain"]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(::gpioi::otyper::OT15W::OPEN_DRAIN)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OT3`"]
pub type OT3W = ::gpioi::otyper::OT15W;
#[doc = r" Proxy"]
pub struct _OT3W<'a> {
    w: &'a mut W,
}
impl<'a> _OT3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OT3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is used as push-pull output"]
    #[inline]
    pub fn push_pull(self) -> &'a mut W {
        self.variant(::gpioi::otyper::OT15W::PUSH_PULL)
    }
    #[doc = "Pin is used as open-drain"]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(::gpioi::otyper::OT15W::OPEN_DRAIN)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OT2`"]
pub type OT2W = ::gpioi::otyper::OT15W;
#[doc = r" Proxy"]
pub struct _OT2W<'a> {
    w: &'a mut W,
}
impl<'a> _OT2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OT2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is used as push-pull output"]
    #[inline]
    pub fn push_pull(self) -> &'a mut W {
        self.variant(::gpioi::otyper::OT15W::PUSH_PULL)
    }
    #[doc = "Pin is used as open-drain"]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(::gpioi::otyper::OT15W::OPEN_DRAIN)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OT1`"]
pub type OT1W = ::gpioi::otyper::OT15W;
#[doc = r" Proxy"]
pub struct _OT1W<'a> {
    w: &'a mut W,
}
impl<'a> _OT1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OT1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is used as push-pull output"]
    #[inline]
    pub fn push_pull(self) -> &'a mut W {
        self.variant(::gpioi::otyper::OT15W::PUSH_PULL)
    }
    #[doc = "Pin is used as open-drain"]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(::gpioi::otyper::OT15W::OPEN_DRAIN)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OT0`"]
pub type OT0W = ::gpioi::otyper::OT15W;
#[doc = r" Proxy"]
pub struct _OT0W<'a> {
    w: &'a mut W,
}
impl<'a> _OT0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OT0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is used as push-pull output"]
    #[inline]
    pub fn push_pull(self) -> &'a mut W {
        self.variant(::gpioi::otyper::OT15W::PUSH_PULL)
    }
    #[doc = "Pin is used as open-drain"]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(::gpioi::otyper::OT15W::OPEN_DRAIN)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 15 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ot15(&self) -> OT15R {
        OT15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ot14(&self) -> OT14R {
        OT14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ot13(&self) -> OT13R {
        OT13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ot12(&self) -> OT12R {
        OT12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ot11(&self) -> OT11R {
        OT11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ot10(&self) -> OT10R {
        OT10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ot9(&self) -> OT9R {
        OT9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ot8(&self) -> OT8R {
        OT8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ot7(&self) -> OT7R {
        OT7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ot6(&self) -> OT6R {
        OT6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ot5(&self) -> OT5R {
        OT5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ot4(&self) -> OT4R {
        OT4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ot3(&self) -> OT3R {
        OT3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ot2(&self) -> OT2R {
        OT2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ot1(&self) -> OT1R {
        OT1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ot0(&self) -> OT0R {
        OT0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 15 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ot15(&mut self) -> _OT15W {
        _OT15W { w: self }
    }
    #[doc = "Bit 14 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ot14(&mut self) -> _OT14W {
        _OT14W { w: self }
    }
    #[doc = "Bit 13 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ot13(&mut self) -> _OT13W {
        _OT13W { w: self }
    }
    #[doc = "Bit 12 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ot12(&mut self) -> _OT12W {
        _OT12W { w: self }
    }
    #[doc = "Bit 11 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ot11(&mut self) -> _OT11W {
        _OT11W { w: self }
    }
    #[doc = "Bit 10 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ot10(&mut self) -> _OT10W {
        _OT10W { w: self }
    }
    #[doc = "Bit 9 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ot9(&mut self) -> _OT9W {
        _OT9W { w: self }
    }
    #[doc = "Bit 8 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ot8(&mut self) -> _OT8W {
        _OT8W { w: self }
    }
    #[doc = "Bit 7 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ot7(&mut self) -> _OT7W {
        _OT7W { w: self }
    }
    #[doc = "Bit 6 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ot6(&mut self) -> _OT6W {
        _OT6W { w: self }
    }
    #[doc = "Bit 5 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ot5(&mut self) -> _OT5W {
        _OT5W { w: self }
    }
    #[doc = "Bit 4 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ot4(&mut self) -> _OT4W {
        _OT4W { w: self }
    }
    #[doc = "Bit 3 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ot3(&mut self) -> _OT3W {
        _OT3W { w: self }
    }
    #[doc = "Bit 2 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ot2(&mut self) -> _OT2W {
        _OT2W { w: self }
    }
    #[doc = "Bit 1 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ot1(&mut self) -> _OT1W {
        _OT1W { w: self }
    }
    #[doc = "Bit 0 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ot0(&mut self) -> _OT0W {
        _OT0W { w: self }
    }
}
