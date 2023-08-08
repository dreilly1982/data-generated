#[doc = "Analog-to-digital converter"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc {
    ptr: *mut u8,
}
unsafe impl Send for Adc {}
unsafe impl Sync for Adc {}
impl Adc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "control register 2"]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "sample time register 1"]
    #[inline(always)]
    pub const fn smpr1(self) -> crate::common::Reg<regs::Smpr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "sample time register 2"]
    #[inline(always)]
    pub const fn smpr2(self) -> crate::common::Reg<regs::Smpr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "injected channel data offset register x"]
    #[inline(always)]
    pub const fn jofr(self, n: usize) -> crate::common::Reg<regs::Jofr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize + n * 4usize) as _) }
    }
    #[doc = "watchdog higher threshold register"]
    #[inline(always)]
    pub const fn htr(self) -> crate::common::Reg<regs::Htr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
    #[doc = "watchdog lower threshold register"]
    #[inline(always)]
    pub const fn ltr(self) -> crate::common::Reg<regs::Ltr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize) as _) }
    }
    #[doc = "regular sequence register 1"]
    #[inline(always)]
    pub const fn sqr1(self) -> crate::common::Reg<regs::Sqr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(44usize) as _) }
    }
    #[doc = "regular sequence register 2"]
    #[inline(always)]
    pub const fn sqr2(self) -> crate::common::Reg<regs::Sqr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(48usize) as _) }
    }
    #[doc = "regular sequence register 3"]
    #[inline(always)]
    pub const fn sqr3(self) -> crate::common::Reg<regs::Sqr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(52usize) as _) }
    }
    #[doc = "injected sequence register"]
    #[inline(always)]
    pub const fn jsqr(self) -> crate::common::Reg<regs::Jsqr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(56usize) as _) }
    }
    #[doc = "injected data register x"]
    #[inline(always)]
    pub const fn jdr(self, n: usize) -> crate::common::Reg<regs::Jdr, crate::common::R> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(60usize + n * 4usize) as _) }
    }
    #[doc = "regular data register"]
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<regs::Dr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(76usize) as _) }
    }
}
pub mod regs {
    #[doc = "control register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "Analog watchdog channel select bits"]
        #[inline(always)]
        pub const fn awdch(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Analog watchdog channel select bits"]
        #[inline(always)]
        pub fn set_awdch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Interrupt enable for EOC"]
        #[inline(always)]
        pub const fn eocie(&self) -> super::vals::Eocie {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Eocie::from_bits(val as u8)
        }
        #[doc = "Interrupt enable for EOC"]
        #[inline(always)]
        pub fn set_eocie(&mut self, val: super::vals::Eocie) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "Analog watchdog interrupt enable"]
        #[inline(always)]
        pub const fn awdie(&self) -> super::vals::Awdie {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Awdie::from_bits(val as u8)
        }
        #[doc = "Analog watchdog interrupt enable"]
        #[inline(always)]
        pub fn set_awdie(&mut self, val: super::vals::Awdie) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "Interrupt enable for injected channels"]
        #[inline(always)]
        pub const fn jeocie(&self) -> super::vals::Jeocie {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Jeocie::from_bits(val as u8)
        }
        #[doc = "Interrupt enable for injected channels"]
        #[inline(always)]
        pub fn set_jeocie(&mut self, val: super::vals::Jeocie) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "Scan mode"]
        #[inline(always)]
        pub const fn scan(&self) -> super::vals::Scan {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Scan::from_bits(val as u8)
        }
        #[doc = "Scan mode"]
        #[inline(always)]
        pub fn set_scan(&mut self, val: super::vals::Scan) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "Enable the watchdog on a single channel in scan mode"]
        #[inline(always)]
        pub const fn awdsgl(&self) -> super::vals::Awdsgl {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Awdsgl::from_bits(val as u8)
        }
        #[doc = "Enable the watchdog on a single channel in scan mode"]
        #[inline(always)]
        pub fn set_awdsgl(&mut self, val: super::vals::Awdsgl) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "Automatic injected group conversion"]
        #[inline(always)]
        pub const fn jauto(&self) -> super::vals::Jauto {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::Jauto::from_bits(val as u8)
        }
        #[doc = "Automatic injected group conversion"]
        #[inline(always)]
        pub fn set_jauto(&mut self, val: super::vals::Jauto) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
        #[doc = "Discontinuous mode on regular channels"]
        #[inline(always)]
        pub const fn discen(&self) -> super::vals::Discen {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Discen::from_bits(val as u8)
        }
        #[doc = "Discontinuous mode on regular channels"]
        #[inline(always)]
        pub fn set_discen(&mut self, val: super::vals::Discen) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "Discontinuous mode on injected channels"]
        #[inline(always)]
        pub const fn jdiscen(&self) -> super::vals::Jdiscen {
            let val = (self.0 >> 12usize) & 0x01;
            super::vals::Jdiscen::from_bits(val as u8)
        }
        #[doc = "Discontinuous mode on injected channels"]
        #[inline(always)]
        pub fn set_jdiscen(&mut self, val: super::vals::Jdiscen) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
        }
        #[doc = "Discontinuous mode channel count"]
        #[inline(always)]
        pub const fn discnum(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[doc = "Discontinuous mode channel count"]
        #[inline(always)]
        pub fn set_discnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
        }
        #[doc = "Analog watchdog enable on injected channels"]
        #[inline(always)]
        pub const fn jawden(&self) -> super::vals::Jawden {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::Jawden::from_bits(val as u8)
        }
        #[doc = "Analog watchdog enable on injected channels"]
        #[inline(always)]
        pub fn set_jawden(&mut self, val: super::vals::Jawden) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
        }
        #[doc = "Analog watchdog enable on regular channels"]
        #[inline(always)]
        pub const fn awden(&self) -> super::vals::Awden {
            let val = (self.0 >> 23usize) & 0x01;
            super::vals::Awden::from_bits(val as u8)
        }
        #[doc = "Analog watchdog enable on regular channels"]
        #[inline(always)]
        pub fn set_awden(&mut self, val: super::vals::Awden) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
        }
        #[doc = "Resolution"]
        #[inline(always)]
        pub const fn res(&self) -> super::vals::Res {
            let val = (self.0 >> 24usize) & 0x03;
            super::vals::Res::from_bits(val as u8)
        }
        #[doc = "Resolution"]
        #[inline(always)]
        pub fn set_res(&mut self, val: super::vals::Res) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
        }
        #[doc = "Overrun interrupt enable"]
        #[inline(always)]
        pub const fn ovrie(&self) -> super::vals::Ovrie {
            let val = (self.0 >> 26usize) & 0x01;
            super::vals::Ovrie::from_bits(val as u8)
        }
        #[doc = "Overrun interrupt enable"]
        #[inline(always)]
        pub fn set_ovrie(&mut self, val: super::vals::Ovrie) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
        }
    }
    impl Default for Cr1 {
        #[inline(always)]
        fn default() -> Cr1 {
            Cr1(0)
        }
    }
    #[doc = "control register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2(pub u32);
    impl Cr2 {
        #[doc = "A/D Converter ON / OFF"]
        #[inline(always)]
        pub const fn adon(&self) -> super::vals::Adon {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Adon::from_bits(val as u8)
        }
        #[doc = "A/D Converter ON / OFF"]
        #[inline(always)]
        pub fn set_adon(&mut self, val: super::vals::Adon) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Continuous conversion"]
        #[inline(always)]
        pub const fn cont(&self) -> super::vals::Cont {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Cont::from_bits(val as u8)
        }
        #[doc = "Continuous conversion"]
        #[inline(always)]
        pub fn set_cont(&mut self, val: super::vals::Cont) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "Direct memory access mode (for single ADC mode)"]
        #[inline(always)]
        pub const fn dma(&self) -> super::vals::Dma {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Dma::from_bits(val as u8)
        }
        #[doc = "Direct memory access mode (for single ADC mode)"]
        #[inline(always)]
        pub fn set_dma(&mut self, val: super::vals::Dma) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "DMA disable selection (for single ADC mode)"]
        #[inline(always)]
        pub const fn dds(&self) -> super::vals::Dds {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Dds::from_bits(val as u8)
        }
        #[doc = "DMA disable selection (for single ADC mode)"]
        #[inline(always)]
        pub fn set_dds(&mut self, val: super::vals::Dds) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "End of conversion selection"]
        #[inline(always)]
        pub const fn eocs(&self) -> super::vals::Eocs {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::Eocs::from_bits(val as u8)
        }
        #[doc = "End of conversion selection"]
        #[inline(always)]
        pub fn set_eocs(&mut self, val: super::vals::Eocs) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
        #[doc = "Data alignment"]
        #[inline(always)]
        pub const fn align(&self) -> super::vals::Align {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Align::from_bits(val as u8)
        }
        #[doc = "Data alignment"]
        #[inline(always)]
        pub fn set_align(&mut self, val: super::vals::Align) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "External event select for injected group"]
        #[inline(always)]
        pub const fn jextsel(&self) -> super::vals::Jextsel {
            let val = (self.0 >> 16usize) & 0x0f;
            super::vals::Jextsel::from_bits(val as u8)
        }
        #[doc = "External event select for injected group"]
        #[inline(always)]
        pub fn set_jextsel(&mut self, val: super::vals::Jextsel) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
        }
        #[doc = "External trigger enable for injected channels"]
        #[inline(always)]
        pub const fn jexten(&self) -> super::vals::Jexten {
            let val = (self.0 >> 20usize) & 0x03;
            super::vals::Jexten::from_bits(val as u8)
        }
        #[doc = "External trigger enable for injected channels"]
        #[inline(always)]
        pub fn set_jexten(&mut self, val: super::vals::Jexten) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
        }
        #[doc = "Start conversion of injected channels"]
        #[inline(always)]
        pub const fn jswstart(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Start conversion of injected channels"]
        #[inline(always)]
        pub fn set_jswstart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "External event select for regular group"]
        #[inline(always)]
        pub const fn extsel(&self) -> super::vals::Extsel {
            let val = (self.0 >> 24usize) & 0x0f;
            super::vals::Extsel::from_bits(val as u8)
        }
        #[doc = "External event select for regular group"]
        #[inline(always)]
        pub fn set_extsel(&mut self, val: super::vals::Extsel) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
        }
        #[doc = "External trigger enable for regular channels"]
        #[inline(always)]
        pub const fn exten(&self) -> super::vals::Exten {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Exten::from_bits(val as u8)
        }
        #[doc = "External trigger enable for regular channels"]
        #[inline(always)]
        pub fn set_exten(&mut self, val: super::vals::Exten) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
        #[doc = "Start conversion of regular channels"]
        #[inline(always)]
        pub const fn swstart(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Start conversion of regular channels"]
        #[inline(always)]
        pub fn set_swstart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Cr2 {
        #[inline(always)]
        fn default() -> Cr2 {
            Cr2(0)
        }
    }
    #[doc = "regular data register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dr(pub u32);
    impl Dr {
        #[doc = "Regular data"]
        #[inline(always)]
        pub const fn data(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Regular data"]
        #[inline(always)]
        pub fn set_data(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Dr {
        #[inline(always)]
        fn default() -> Dr {
            Dr(0)
        }
    }
    #[doc = "watchdog higher threshold register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Htr(pub u32);
    impl Htr {
        #[doc = "Analog watchdog higher threshold"]
        #[inline(always)]
        pub const fn ht(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Analog watchdog higher threshold"]
        #[inline(always)]
        pub fn set_ht(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Htr {
        #[inline(always)]
        fn default() -> Htr {
            Htr(0)
        }
    }
    #[doc = "injected data register x"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Jdr(pub u32);
    impl Jdr {
        #[doc = "Injected data"]
        #[inline(always)]
        pub const fn jdata(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Injected data"]
        #[inline(always)]
        pub fn set_jdata(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Jdr {
        #[inline(always)]
        fn default() -> Jdr {
            Jdr(0)
        }
    }
    #[doc = "injected channel data offset register x"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Jofr(pub u32);
    impl Jofr {
        #[doc = "Data offset for injected channel x"]
        #[inline(always)]
        pub const fn joffset(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Data offset for injected channel x"]
        #[inline(always)]
        pub fn set_joffset(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Jofr {
        #[inline(always)]
        fn default() -> Jofr {
            Jofr(0)
        }
    }
    #[doc = "injected sequence register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Jsqr(pub u32);
    impl Jsqr {
        #[doc = "1st conversion in injected sequence"]
        #[inline(always)]
        pub const fn jsq(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 5usize;
            let val = (self.0 >> offs) & 0x1f;
            val as u8
        }
        #[doc = "1st conversion in injected sequence"]
        #[inline(always)]
        pub fn set_jsq(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 0usize + n * 5usize;
            self.0 = (self.0 & !(0x1f << offs)) | (((val as u32) & 0x1f) << offs);
        }
        #[doc = "Injected sequence length"]
        #[inline(always)]
        pub const fn jl(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "Injected sequence length"]
        #[inline(always)]
        pub fn set_jl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
    }
    impl Default for Jsqr {
        #[inline(always)]
        fn default() -> Jsqr {
            Jsqr(0)
        }
    }
    #[doc = "watchdog lower threshold register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ltr(pub u32);
    impl Ltr {
        #[doc = "Analog watchdog lower threshold"]
        #[inline(always)]
        pub const fn lt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Analog watchdog lower threshold"]
        #[inline(always)]
        pub fn set_lt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Ltr {
        #[inline(always)]
        fn default() -> Ltr {
            Ltr(0)
        }
    }
    #[doc = "sample time register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Smpr1(pub u32);
    impl Smpr1 {
        #[doc = "Channel 10 sampling time selection"]
        #[inline(always)]
        pub const fn smp(&self, n: usize) -> super::vals::SampleTime {
            assert!(n < 9usize);
            let offs = 0usize + n * 3usize;
            let val = (self.0 >> offs) & 0x07;
            super::vals::SampleTime::from_bits(val as u8)
        }
        #[doc = "Channel 10 sampling time selection"]
        #[inline(always)]
        pub fn set_smp(&mut self, n: usize, val: super::vals::SampleTime) {
            assert!(n < 9usize);
            let offs = 0usize + n * 3usize;
            self.0 = (self.0 & !(0x07 << offs)) | (((val.to_bits() as u32) & 0x07) << offs);
        }
        #[doc = "Sample time bits"]
        #[inline(always)]
        pub const fn smpx_x(&self) -> super::vals::SmprSmpxX {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            super::vals::SmprSmpxX::from_bits(val as u32)
        }
        #[doc = "Sample time bits"]
        #[inline(always)]
        pub fn set_smpx_x(&mut self, val: super::vals::SmprSmpxX) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Smpr1 {
        #[inline(always)]
        fn default() -> Smpr1 {
            Smpr1(0)
        }
    }
    #[doc = "sample time register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Smpr2(pub u32);
    impl Smpr2 {
        #[doc = "Channel 0 sampling time selection"]
        #[inline(always)]
        pub const fn smp(&self, n: usize) -> super::vals::SampleTime {
            assert!(n < 10usize);
            let offs = 0usize + n * 3usize;
            let val = (self.0 >> offs) & 0x07;
            super::vals::SampleTime::from_bits(val as u8)
        }
        #[doc = "Channel 0 sampling time selection"]
        #[inline(always)]
        pub fn set_smp(&mut self, n: usize, val: super::vals::SampleTime) {
            assert!(n < 10usize);
            let offs = 0usize + n * 3usize;
            self.0 = (self.0 & !(0x07 << offs)) | (((val.to_bits() as u32) & 0x07) << offs);
        }
        #[doc = "Sample time bits"]
        #[inline(always)]
        pub const fn smpx_x(&self) -> super::vals::SmprSmpxX {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            super::vals::SmprSmpxX::from_bits(val as u32)
        }
        #[doc = "Sample time bits"]
        #[inline(always)]
        pub fn set_smpx_x(&mut self, val: super::vals::SmprSmpxX) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Smpr2 {
        #[inline(always)]
        fn default() -> Smpr2 {
            Smpr2(0)
        }
    }
    #[doc = "regular sequence register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqr1(pub u32);
    impl Sqr1 {
        #[doc = "13th conversion in regular sequence"]
        #[inline(always)]
        pub const fn sq(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 5usize;
            let val = (self.0 >> offs) & 0x1f;
            val as u8
        }
        #[doc = "13th conversion in regular sequence"]
        #[inline(always)]
        pub fn set_sq(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 0usize + n * 5usize;
            self.0 = (self.0 & !(0x1f << offs)) | (((val as u32) & 0x1f) << offs);
        }
        #[doc = "Regular channel sequence length"]
        #[inline(always)]
        pub const fn l(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "Regular channel sequence length"]
        #[inline(always)]
        pub fn set_l(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
    }
    impl Default for Sqr1 {
        #[inline(always)]
        fn default() -> Sqr1 {
            Sqr1(0)
        }
    }
    #[doc = "regular sequence register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqr2(pub u32);
    impl Sqr2 {
        #[doc = "7th conversion in regular sequence"]
        #[inline(always)]
        pub const fn sq(&self, n: usize) -> u8 {
            assert!(n < 6usize);
            let offs = 0usize + n * 5usize;
            let val = (self.0 >> offs) & 0x1f;
            val as u8
        }
        #[doc = "7th conversion in regular sequence"]
        #[inline(always)]
        pub fn set_sq(&mut self, n: usize, val: u8) {
            assert!(n < 6usize);
            let offs = 0usize + n * 5usize;
            self.0 = (self.0 & !(0x1f << offs)) | (((val as u32) & 0x1f) << offs);
        }
    }
    impl Default for Sqr2 {
        #[inline(always)]
        fn default() -> Sqr2 {
            Sqr2(0)
        }
    }
    #[doc = "regular sequence register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqr3(pub u32);
    impl Sqr3 {
        #[doc = "1st conversion in regular sequence"]
        #[inline(always)]
        pub const fn sq(&self, n: usize) -> u8 {
            assert!(n < 6usize);
            let offs = 0usize + n * 5usize;
            let val = (self.0 >> offs) & 0x1f;
            val as u8
        }
        #[doc = "1st conversion in regular sequence"]
        #[inline(always)]
        pub fn set_sq(&mut self, n: usize, val: u8) {
            assert!(n < 6usize);
            let offs = 0usize + n * 5usize;
            self.0 = (self.0 & !(0x1f << offs)) | (((val as u32) & 0x1f) << offs);
        }
    }
    impl Default for Sqr3 {
        #[inline(always)]
        fn default() -> Sqr3 {
            Sqr3(0)
        }
    }
    #[doc = "status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Analog watchdog flag"]
        #[inline(always)]
        pub const fn awd(&self) -> super::vals::Awd {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Awd::from_bits(val as u8)
        }
        #[doc = "Analog watchdog flag"]
        #[inline(always)]
        pub fn set_awd(&mut self, val: super::vals::Awd) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Regular channel end of conversion"]
        #[inline(always)]
        pub const fn eoc(&self) -> super::vals::Eoc {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Eoc::from_bits(val as u8)
        }
        #[doc = "Regular channel end of conversion"]
        #[inline(always)]
        pub fn set_eoc(&mut self, val: super::vals::Eoc) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "Injected channel end of conversion"]
        #[inline(always)]
        pub const fn jeoc(&self) -> super::vals::Jeoc {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Jeoc::from_bits(val as u8)
        }
        #[doc = "Injected channel end of conversion"]
        #[inline(always)]
        pub fn set_jeoc(&mut self, val: super::vals::Jeoc) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "Injected channel start flag"]
        #[inline(always)]
        pub const fn jstrt(&self) -> super::vals::Jstrt {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Jstrt::from_bits(val as u8)
        }
        #[doc = "Injected channel start flag"]
        #[inline(always)]
        pub fn set_jstrt(&mut self, val: super::vals::Jstrt) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Regular channel start flag"]
        #[inline(always)]
        pub const fn strt(&self) -> super::vals::Strt {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Strt::from_bits(val as u8)
        }
        #[doc = "Regular channel start flag"]
        #[inline(always)]
        pub fn set_strt(&mut self, val: super::vals::Strt) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Overrun"]
        #[inline(always)]
        pub const fn ovr(&self) -> super::vals::Ovr {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Ovr::from_bits(val as u8)
        }
        #[doc = "Overrun"]
        #[inline(always)]
        pub fn set_ovr(&mut self, val: super::vals::Ovr) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Sr {
        #[inline(always)]
        fn default() -> Sr {
            Sr(0)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Adon {
        #[doc = "Disable ADC conversion and go to power down mode"]
        DISABLED = 0,
        #[doc = "Enable ADC"]
        ENABLED = 0x01,
    }
    impl Adon {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Adon {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Adon {
        #[inline(always)]
        fn from(val: u8) -> Adon {
            Adon::from_bits(val)
        }
    }
    impl From<Adon> for u8 {
        #[inline(always)]
        fn from(val: Adon) -> u8 {
            Adon::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Align {
        #[doc = "Right alignment"]
        RIGHT = 0,
        #[doc = "Left alignment"]
        LEFT = 0x01,
    }
    impl Align {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Align {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Align {
        #[inline(always)]
        fn from(val: u8) -> Align {
            Align::from_bits(val)
        }
    }
    impl From<Align> for u8 {
        #[inline(always)]
        fn from(val: Align) -> u8 {
            Align::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Awd {
        #[doc = "No analog watchdog event occurred"]
        NOEVENT = 0,
        #[doc = "Analog watchdog event occurred"]
        EVENT = 0x01,
    }
    impl Awd {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Awd {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Awd {
        #[inline(always)]
        fn from(val: u8) -> Awd {
            Awd::from_bits(val)
        }
    }
    impl From<Awd> for u8 {
        #[inline(always)]
        fn from(val: Awd) -> u8 {
            Awd::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Awden {
        #[doc = "Analog watchdog disabled on regular channels"]
        DISABLED = 0,
        #[doc = "Analog watchdog enabled on regular channels"]
        ENABLED = 0x01,
    }
    impl Awden {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Awden {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Awden {
        #[inline(always)]
        fn from(val: u8) -> Awden {
            Awden::from_bits(val)
        }
    }
    impl From<Awden> for u8 {
        #[inline(always)]
        fn from(val: Awden) -> u8 {
            Awden::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Awdie {
        #[doc = "Analogue watchdog interrupt disabled"]
        DISABLED = 0,
        #[doc = "Analogue watchdog interrupt enabled"]
        ENABLED = 0x01,
    }
    impl Awdie {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Awdie {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Awdie {
        #[inline(always)]
        fn from(val: u8) -> Awdie {
            Awdie::from_bits(val)
        }
    }
    impl From<Awdie> for u8 {
        #[inline(always)]
        fn from(val: Awdie) -> u8 {
            Awdie::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Awdsgl {
        #[doc = "Analog watchdog enabled on all channels"]
        ALLCHANNELS = 0,
        #[doc = "Analog watchdog enabled on a single channel"]
        SINGLECHANNEL = 0x01,
    }
    impl Awdsgl {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Awdsgl {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Awdsgl {
        #[inline(always)]
        fn from(val: u8) -> Awdsgl {
            Awdsgl::from_bits(val)
        }
    }
    impl From<Awdsgl> for u8 {
        #[inline(always)]
        fn from(val: Awdsgl) -> u8 {
            Awdsgl::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Cont {
        #[doc = "Single conversion mode"]
        SINGLE = 0,
        #[doc = "Continuous conversion mode"]
        CONTINUOUS = 0x01,
    }
    impl Cont {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cont {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cont {
        #[inline(always)]
        fn from(val: u8) -> Cont {
            Cont::from_bits(val)
        }
    }
    impl From<Cont> for u8 {
        #[inline(always)]
        fn from(val: Cont) -> u8 {
            Cont::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Dds {
        #[doc = "No new DMA request is issued after the last transfer"]
        SINGLE = 0,
        #[doc = "DMA requests are issued as long as data are converted and DMA=1"]
        CONTINUOUS = 0x01,
    }
    impl Dds {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dds {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dds {
        #[inline(always)]
        fn from(val: u8) -> Dds {
            Dds::from_bits(val)
        }
    }
    impl From<Dds> for u8 {
        #[inline(always)]
        fn from(val: Dds) -> u8 {
            Dds::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Discen {
        #[doc = "Discontinuous mode on regular channels disabled"]
        DISABLED = 0,
        #[doc = "Discontinuous mode on regular channels enabled"]
        ENABLED = 0x01,
    }
    impl Discen {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Discen {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Discen {
        #[inline(always)]
        fn from(val: u8) -> Discen {
            Discen::from_bits(val)
        }
    }
    impl From<Discen> for u8 {
        #[inline(always)]
        fn from(val: Discen) -> u8 {
            Discen::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Dma {
        #[doc = "DMA mode disabled"]
        DISABLED = 0,
        #[doc = "DMA mode enabled"]
        ENABLED = 0x01,
    }
    impl Dma {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dma {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dma {
        #[inline(always)]
        fn from(val: u8) -> Dma {
            Dma::from_bits(val)
        }
    }
    impl From<Dma> for u8 {
        #[inline(always)]
        fn from(val: Dma) -> u8 {
            Dma::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Eoc {
        #[doc = "Conversion is not complete"]
        NOTCOMPLETE = 0,
        #[doc = "Conversion complete"]
        COMPLETE = 0x01,
    }
    impl Eoc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Eoc {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Eoc {
        #[inline(always)]
        fn from(val: u8) -> Eoc {
            Eoc::from_bits(val)
        }
    }
    impl From<Eoc> for u8 {
        #[inline(always)]
        fn from(val: Eoc) -> u8 {
            Eoc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Eocie {
        #[doc = "EOC interrupt disabled"]
        DISABLED = 0,
        #[doc = "EOC interrupt enabled"]
        ENABLED = 0x01,
    }
    impl Eocie {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Eocie {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Eocie {
        #[inline(always)]
        fn from(val: u8) -> Eocie {
            Eocie::from_bits(val)
        }
    }
    impl From<Eocie> for u8 {
        #[inline(always)]
        fn from(val: Eocie) -> u8 {
            Eocie::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Eocs {
        #[doc = "The EOC bit is set at the end of each sequence of regular conversions"]
        EACHSEQUENCE = 0,
        #[doc = "The EOC bit is set at the end of each regular conversion"]
        EACHCONVERSION = 0x01,
    }
    impl Eocs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Eocs {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Eocs {
        #[inline(always)]
        fn from(val: u8) -> Eocs {
            Eocs::from_bits(val)
        }
    }
    impl From<Eocs> for u8 {
        #[inline(always)]
        fn from(val: Eocs) -> u8 {
            Eocs::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Exten {
        #[doc = "Trigger detection disabled"]
        DISABLED = 0,
        #[doc = "Trigger detection on the rising edge"]
        RISINGEDGE = 0x01,
        #[doc = "Trigger detection on the falling edge"]
        FALLINGEDGE = 0x02,
        #[doc = "Trigger detection on both the rising and falling edges"]
        BOTHEDGES = 0x03,
    }
    impl Exten {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Exten {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Exten {
        #[inline(always)]
        fn from(val: u8) -> Exten {
            Exten::from_bits(val)
        }
    }
    impl From<Exten> for u8 {
        #[inline(always)]
        fn from(val: Exten) -> u8 {
            Exten::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Extsel {
        #[doc = "Timer 1 CC1 event"]
        TIM1CC1 = 0,
        #[doc = "Timer 1 CC2 event"]
        TIM1CC2 = 0x01,
        #[doc = "Timer 1 CC3 event"]
        TIM1CC3 = 0x02,
        #[doc = "Timer 2 CC2 event"]
        TIM2CC2 = 0x03,
        #[doc = "Timer 2 CC3 event"]
        TIM2CC3 = 0x04,
        #[doc = "Timer 2 CC4 event"]
        TIM2CC4 = 0x05,
        #[doc = "Timer 2 TRGO event"]
        TIM2TRGO = 0x06,
        _RESERVED_7 = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Extsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Extsel {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Extsel {
        #[inline(always)]
        fn from(val: u8) -> Extsel {
            Extsel::from_bits(val)
        }
    }
    impl From<Extsel> for u8 {
        #[inline(always)]
        fn from(val: Extsel) -> u8 {
            Extsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Jauto {
        #[doc = "Automatic injected group conversion disabled"]
        DISABLED = 0,
        #[doc = "Automatic injected group conversion enabled"]
        ENABLED = 0x01,
    }
    impl Jauto {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Jauto {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Jauto {
        #[inline(always)]
        fn from(val: u8) -> Jauto {
            Jauto::from_bits(val)
        }
    }
    impl From<Jauto> for u8 {
        #[inline(always)]
        fn from(val: Jauto) -> u8 {
            Jauto::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Jawden {
        #[doc = "Analog watchdog disabled on injected channels"]
        DISABLED = 0,
        #[doc = "Analog watchdog enabled on injected channels"]
        ENABLED = 0x01,
    }
    impl Jawden {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Jawden {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Jawden {
        #[inline(always)]
        fn from(val: u8) -> Jawden {
            Jawden::from_bits(val)
        }
    }
    impl From<Jawden> for u8 {
        #[inline(always)]
        fn from(val: Jawden) -> u8 {
            Jawden::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Jdiscen {
        #[doc = "Discontinuous mode on injected channels disabled"]
        DISABLED = 0,
        #[doc = "Discontinuous mode on injected channels enabled"]
        ENABLED = 0x01,
    }
    impl Jdiscen {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Jdiscen {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Jdiscen {
        #[inline(always)]
        fn from(val: u8) -> Jdiscen {
            Jdiscen::from_bits(val)
        }
    }
    impl From<Jdiscen> for u8 {
        #[inline(always)]
        fn from(val: Jdiscen) -> u8 {
            Jdiscen::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Jeoc {
        #[doc = "Conversion is not complete"]
        NOTCOMPLETE = 0,
        #[doc = "Conversion complete"]
        COMPLETE = 0x01,
    }
    impl Jeoc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Jeoc {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Jeoc {
        #[inline(always)]
        fn from(val: u8) -> Jeoc {
            Jeoc::from_bits(val)
        }
    }
    impl From<Jeoc> for u8 {
        #[inline(always)]
        fn from(val: Jeoc) -> u8 {
            Jeoc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Jeocie {
        #[doc = "JEOC interrupt disabled"]
        DISABLED = 0,
        #[doc = "JEOC interrupt enabled"]
        ENABLED = 0x01,
    }
    impl Jeocie {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Jeocie {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Jeocie {
        #[inline(always)]
        fn from(val: u8) -> Jeocie {
            Jeocie::from_bits(val)
        }
    }
    impl From<Jeocie> for u8 {
        #[inline(always)]
        fn from(val: Jeocie) -> u8 {
            Jeocie::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Jexten {
        #[doc = "Trigger detection disabled"]
        DISABLED = 0,
        #[doc = "Trigger detection on the rising edge"]
        RISINGEDGE = 0x01,
        #[doc = "Trigger detection on the falling edge"]
        FALLINGEDGE = 0x02,
        #[doc = "Trigger detection on both the rising and falling edges"]
        BOTHEDGES = 0x03,
    }
    impl Jexten {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Jexten {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Jexten {
        #[inline(always)]
        fn from(val: u8) -> Jexten {
            Jexten::from_bits(val)
        }
    }
    impl From<Jexten> for u8 {
        #[inline(always)]
        fn from(val: Jexten) -> u8 {
            Jexten::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Jextsel {
        #[doc = "Timer 1 TRGO event"]
        TIM1TRGO = 0,
        #[doc = "Timer 1 CC4 event"]
        TIM1CC4 = 0x01,
        #[doc = "Timer 2 TRGO event"]
        TIM2TRGO = 0x02,
        #[doc = "Timer 2 CC1 event"]
        TIM2CC1 = 0x03,
        #[doc = "Timer 3 CC4 event"]
        TIM3CC4 = 0x04,
        #[doc = "Timer 4 TRGO event"]
        TIM4TRGO = 0x05,
        _RESERVED_6 = 0x06,
        #[doc = "Timer 8 CC4 event"]
        TIM8CC4 = 0x07,
        #[doc = "Timer 1 TRGO(2) event"]
        TIM1TRGO2 = 0x08,
        #[doc = "Timer 8 TRGO event"]
        TIM8TRGO = 0x09,
        #[doc = "Timer 8 TRGO(2) event"]
        TIM8TRGO2 = 0x0a,
        #[doc = "Timer 3 CC3 event"]
        TIM3CC3 = 0x0b,
        #[doc = "Timer 5 TRGO event"]
        TIM5TRGO = 0x0c,
        #[doc = "Timer 3 CC1 event"]
        TIM3CC1 = 0x0d,
        #[doc = "Timer 6 TRGO event"]
        TIM6TRGO = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Jextsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Jextsel {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Jextsel {
        #[inline(always)]
        fn from(val: u8) -> Jextsel {
            Jextsel::from_bits(val)
        }
    }
    impl From<Jextsel> for u8 {
        #[inline(always)]
        fn from(val: Jextsel) -> u8 {
            Jextsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Jstrt {
        #[doc = "No injected channel conversion started"]
        NOTSTARTED = 0,
        #[doc = "Injected channel conversion has started"]
        STARTED = 0x01,
    }
    impl Jstrt {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Jstrt {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Jstrt {
        #[inline(always)]
        fn from(val: u8) -> Jstrt {
            Jstrt::from_bits(val)
        }
    }
    impl From<Jstrt> for u8 {
        #[inline(always)]
        fn from(val: Jstrt) -> u8 {
            Jstrt::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Ovr {
        #[doc = "No overrun occurred"]
        NOOVERRUN = 0,
        #[doc = "Overrun occurred"]
        OVERRUN = 0x01,
    }
    impl Ovr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ovr {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ovr {
        #[inline(always)]
        fn from(val: u8) -> Ovr {
            Ovr::from_bits(val)
        }
    }
    impl From<Ovr> for u8 {
        #[inline(always)]
        fn from(val: Ovr) -> u8 {
            Ovr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Ovrie {
        #[doc = "Overrun interrupt disabled"]
        DISABLED = 0,
        #[doc = "Overrun interrupt enabled"]
        ENABLED = 0x01,
    }
    impl Ovrie {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ovrie {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ovrie {
        #[inline(always)]
        fn from(val: u8) -> Ovrie {
            Ovrie::from_bits(val)
        }
    }
    impl From<Ovrie> for u8 {
        #[inline(always)]
        fn from(val: Ovrie) -> u8 {
            Ovrie::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Res {
        #[doc = "12-bit (15 ADCCLK cycles)"]
        TWELVEBIT = 0,
        #[doc = "10-bit (13 ADCCLK cycles)"]
        TENBIT = 0x01,
        #[doc = "8-bit (11 ADCCLK cycles)"]
        EIGHTBIT = 0x02,
        #[doc = "6-bit (9 ADCCLK cycles)"]
        SIXBIT = 0x03,
    }
    impl Res {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Res {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Res {
        #[inline(always)]
        fn from(val: u8) -> Res {
            Res::from_bits(val)
        }
    }
    impl From<Res> for u8 {
        #[inline(always)]
        fn from(val: Res) -> u8 {
            Res::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum SampleTime {
        #[doc = "3 cycles"]
        CYCLES3 = 0,
        #[doc = "15 cycles"]
        CYCLES15 = 0x01,
        #[doc = "28 cycles"]
        CYCLES28 = 0x02,
        #[doc = "56 cycles"]
        CYCLES56 = 0x03,
        #[doc = "84 cycles"]
        CYCLES84 = 0x04,
        #[doc = "112 cycles"]
        CYCLES112 = 0x05,
        #[doc = "144 cycles"]
        CYCLES144 = 0x06,
        #[doc = "480 cycles"]
        CYCLES480 = 0x07,
    }
    impl SampleTime {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> SampleTime {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for SampleTime {
        #[inline(always)]
        fn from(val: u8) -> SampleTime {
            SampleTime::from_bits(val)
        }
    }
    impl From<SampleTime> for u8 {
        #[inline(always)]
        fn from(val: SampleTime) -> u8 {
            SampleTime::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Scan {
        #[doc = "Scan mode disabled"]
        DISABLED = 0,
        #[doc = "Scan mode enabled"]
        ENABLED = 0x01,
    }
    impl Scan {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scan {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scan {
        #[inline(always)]
        fn from(val: u8) -> Scan {
            Scan::from_bits(val)
        }
    }
    impl From<Scan> for u8 {
        #[inline(always)]
        fn from(val: Scan) -> u8 {
            Scan::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct SmprSmpxX(pub u32);
    impl SmprSmpxX {
        #[doc = "3 cycles"]
        pub const CYCLES3: Self = Self(0);
        #[doc = "15 cycles"]
        pub const CYCLES15: Self = Self(0x01);
        #[doc = "28 cycles"]
        pub const CYCLES28: Self = Self(0x02);
        #[doc = "56 cycles"]
        pub const CYCLES56: Self = Self(0x03);
        #[doc = "84 cycles"]
        pub const CYCLES84: Self = Self(0x04);
        #[doc = "112 cycles"]
        pub const CYCLES112: Self = Self(0x05);
        #[doc = "144 cycles"]
        pub const CYCLES144: Self = Self(0x06);
        #[doc = "480 cycles"]
        pub const CYCLES480: Self = Self(0x07);
    }
    impl SmprSmpxX {
        pub const fn from_bits(val: u32) -> SmprSmpxX {
            Self(val & 0xffff_ffff)
        }
        pub const fn to_bits(self) -> u32 {
            self.0
        }
    }
    impl From<u32> for SmprSmpxX {
        #[inline(always)]
        fn from(val: u32) -> SmprSmpxX {
            SmprSmpxX::from_bits(val)
        }
    }
    impl From<SmprSmpxX> for u32 {
        #[inline(always)]
        fn from(val: SmprSmpxX) -> u32 {
            SmprSmpxX::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Strt {
        #[doc = "No regular channel conversion started"]
        NOTSTARTED = 0,
        #[doc = "Regular channel conversion has started"]
        STARTED = 0x01,
    }
    impl Strt {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Strt {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Strt {
        #[inline(always)]
        fn from(val: u8) -> Strt {
            Strt::from_bits(val)
        }
    }
    impl From<Strt> for u8 {
        #[inline(always)]
        fn from(val: Strt) -> u8 {
            Strt::to_bits(val)
        }
    }
}
