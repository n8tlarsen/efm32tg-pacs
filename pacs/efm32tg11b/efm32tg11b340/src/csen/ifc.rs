#[doc = "Register `IFC` writer"]
pub struct W(crate::W<IFC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<IFC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMP` writer - Clear CMP Interrupt Flag"]
pub type CMP_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 0>;
#[doc = "Field `CONV` writer - Clear CONV Interrupt Flag"]
pub type CONV_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 1>;
#[doc = "Field `EOS` writer - Clear EOS Interrupt Flag"]
pub type EOS_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 2>;
#[doc = "Field `DMAOF` writer - Clear DMAOF Interrupt Flag"]
pub type DMAOF_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 3>;
#[doc = "Field `APORTCONFLICT` writer - Clear APORTCONFLICT Interrupt Flag"]
pub type APORTCONFLICT_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 4>;
impl W {
    #[doc = "Bit 0 - Clear CMP Interrupt Flag"]
    #[inline(always)]
    pub fn cmp(&mut self) -> CMP_W {
        CMP_W::new(self)
    }
    #[doc = "Bit 1 - Clear CONV Interrupt Flag"]
    #[inline(always)]
    pub fn conv(&mut self) -> CONV_W {
        CONV_W::new(self)
    }
    #[doc = "Bit 2 - Clear EOS Interrupt Flag"]
    #[inline(always)]
    pub fn eos(&mut self) -> EOS_W {
        EOS_W::new(self)
    }
    #[doc = "Bit 3 - Clear DMAOF Interrupt Flag"]
    #[inline(always)]
    pub fn dmaof(&mut self) -> DMAOF_W {
        DMAOF_W::new(self)
    }
    #[doc = "Bit 4 - Clear APORTCONFLICT Interrupt Flag"]
    #[inline(always)]
    pub fn aportconflict(&mut self) -> APORTCONFLICT_W {
        APORTCONFLICT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flag Clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifc](index.html) module"]
pub struct IFC_SPEC;
impl crate::RegisterSpec for IFC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ifc::W](W) writer structure"]
impl crate::Writable for IFC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IFC to value 0"]
impl crate::Resettable for IFC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
