#![doc = "Peripheral access API for EFM32TG11B140F64GM32 microcontrollers (generated using svd2rust v0.28.0 ( ))\n\nYou can find an overview of the generated API [here].\n\nAPI features to be included in the [next]
svd2rust release can be generated by cloning the svd2rust [repository], checking out the above commit, and running `cargo doc --open`.\n\n[here]: https://docs.rs/svd2rust/0.28.0/svd2rust/#peripheral-api\n[next]: https://github.com/rust-embedded/svd2rust/blob/master/CHANGELOG.md#unreleased\n[repository]: https://github.com/rust-embedded/svd2rust"]
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
extern "C" {
    fn EMU();
    fn WDOG0();
    fn LDMA();
    fn GPIO_EVEN();
    fn SMU();
    fn TIMER0();
    fn USART0();
    fn ACMP0();
    fn ADC0();
    fn I2C0();
    fn I2C1();
    fn GPIO_ODD();
    fn TIMER1();
    fn USART1();
    fn USART2();
    fn UART0();
    fn LEUART0();
    fn LETIMER0();
    fn PCNT0();
    fn RTCC();
    fn CMU();
    fn MSC();
    fn CRYPTO0();
    fn CRYOTIMER();
    fn USART3();
    fn WTIMER0();
    fn WTIMER1();
    fn VDAC0();
    fn CSEN();
    fn LESENSE();
    fn CAN0();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 32] = [
    Vector { _handler: EMU },
    Vector { _handler: WDOG0 },
    Vector { _handler: LDMA },
    Vector {
        _handler: GPIO_EVEN,
    },
    Vector { _handler: SMU },
    Vector { _handler: TIMER0 },
    Vector { _handler: USART0 },
    Vector { _handler: ACMP0 },
    Vector { _handler: ADC0 },
    Vector { _handler: I2C0 },
    Vector { _handler: I2C1 },
    Vector { _handler: GPIO_ODD },
    Vector { _handler: TIMER1 },
    Vector { _handler: USART1 },
    Vector { _handler: USART2 },
    Vector { _handler: UART0 },
    Vector { _handler: LEUART0 },
    Vector { _handler: LETIMER0 },
    Vector { _handler: PCNT0 },
    Vector { _handler: RTCC },
    Vector { _handler: CMU },
    Vector { _handler: MSC },
    Vector { _handler: CRYPTO0 },
    Vector {
        _handler: CRYOTIMER,
    },
    Vector { _handler: USART3 },
    Vector { _handler: WTIMER0 },
    Vector { _handler: WTIMER1 },
    Vector { _handler: VDAC0 },
    Vector { _handler: CSEN },
    Vector { _handler: LESENSE },
    Vector { _reserved: 0 },
    Vector { _handler: CAN0 },
];
#[doc = r"Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "0 - EMU"]
    EMU = 0,
    #[doc = "1 - WDOG0"]
    WDOG0 = 1,
    #[doc = "2 - LDMA"]
    LDMA = 2,
    #[doc = "3 - GPIO_EVEN"]
    GPIO_EVEN = 3,
    #[doc = "4 - SMU"]
    SMU = 4,
    #[doc = "5 - TIMER0"]
    TIMER0 = 5,
    #[doc = "6 - USART0"]
    USART0 = 6,
    #[doc = "7 - ACMP0"]
    ACMP0 = 7,
    #[doc = "8 - ADC0"]
    ADC0 = 8,
    #[doc = "9 - I2C0"]
    I2C0 = 9,
    #[doc = "10 - I2C1"]
    I2C1 = 10,
    #[doc = "11 - GPIO_ODD"]
    GPIO_ODD = 11,
    #[doc = "12 - TIMER1"]
    TIMER1 = 12,
    #[doc = "13 - USART1"]
    USART1 = 13,
    #[doc = "14 - USART2"]
    USART2 = 14,
    #[doc = "15 - UART0"]
    UART0 = 15,
    #[doc = "16 - LEUART0"]
    LEUART0 = 16,
    #[doc = "17 - LETIMER0"]
    LETIMER0 = 17,
    #[doc = "18 - PCNT0"]
    PCNT0 = 18,
    #[doc = "19 - RTCC"]
    RTCC = 19,
    #[doc = "20 - CMU"]
    CMU = 20,
    #[doc = "21 - MSC"]
    MSC = 21,
    #[doc = "22 - CRYPTO0"]
    CRYPTO0 = 22,
    #[doc = "23 - CRYOTIMER"]
    CRYOTIMER = 23,
    #[doc = "24 - USART3"]
    USART3 = 24,
    #[doc = "25 - WTIMER0"]
    WTIMER0 = 25,
    #[doc = "26 - WTIMER1"]
    WTIMER1 = 26,
    #[doc = "27 - VDAC0"]
    VDAC0 = 27,
    #[doc = "28 - CSEN"]
    CSEN = 28,
    #[doc = "29 - LESENSE"]
    LESENSE = 29,
    #[doc = "31 - CAN0"]
    CAN0 = 31,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[doc = "MSC"]
pub struct MSC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MSC {}
impl MSC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const msc::RegisterBlock = 0x4000_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const msc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for MSC {
    type Target = msc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for MSC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MSC").finish()
    }
}
#[doc = "MSC"]
pub mod msc;
#[doc = "EMU"]
pub struct EMU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EMU {}
impl EMU {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const emu::RegisterBlock = 0x400e_3000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const emu::RegisterBlock {
        Self::PTR
    }
}
impl Deref for EMU {
    type Target = emu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for EMU {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EMU").finish()
    }
}
#[doc = "EMU"]
pub mod emu;
#[doc = "RMU"]
pub struct RMU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RMU {}
impl RMU {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const rmu::RegisterBlock = 0x400e_5000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rmu::RegisterBlock {
        Self::PTR
    }
}
impl Deref for RMU {
    type Target = rmu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for RMU {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RMU").finish()
    }
}
#[doc = "RMU"]
pub mod rmu;
#[doc = "CMU"]
pub struct CMU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CMU {}
impl CMU {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const cmu::RegisterBlock = 0x400e_4000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cmu::RegisterBlock {
        Self::PTR
    }
}
impl Deref for CMU {
    type Target = cmu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for CMU {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMU").finish()
    }
}
#[doc = "CMU"]
pub mod cmu;
#[doc = "CRYPTO0"]
pub struct CRYPTO0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRYPTO0 {}
impl CRYPTO0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const crypto0::RegisterBlock = 0x400f_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const crypto0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for CRYPTO0 {
    type Target = crypto0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for CRYPTO0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRYPTO0").finish()
    }
}
#[doc = "CRYPTO0"]
pub mod crypto0;
#[doc = "LESENSE"]
pub struct LESENSE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LESENSE {}
impl LESENSE {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const lesense::RegisterBlock = 0x4005_5000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lesense::RegisterBlock {
        Self::PTR
    }
}
impl Deref for LESENSE {
    type Target = lesense::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for LESENSE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LESENSE").finish()
    }
}
#[doc = "LESENSE"]
pub mod lesense;
#[doc = "GPIO"]
pub struct GPIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO {}
impl GPIO {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const gpio::RegisterBlock = 0x4008_8000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio::RegisterBlock {
        Self::PTR
    }
}
impl Deref for GPIO {
    type Target = gpio::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for GPIO {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO").finish()
    }
}
#[doc = "GPIO"]
pub mod gpio;
#[doc = "PRS"]
pub struct PRS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PRS {}
impl PRS {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const prs::RegisterBlock = 0x400e_6000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const prs::RegisterBlock {
        Self::PTR
    }
}
impl Deref for PRS {
    type Target = prs::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for PRS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRS").finish()
    }
}
#[doc = "PRS"]
pub mod prs;
#[doc = "LDMA"]
pub struct LDMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LDMA {}
impl LDMA {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const ldma::RegisterBlock = 0x4000_2000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ldma::RegisterBlock {
        Self::PTR
    }
}
impl Deref for LDMA {
    type Target = ldma::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for LDMA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LDMA").finish()
    }
}
#[doc = "LDMA"]
pub mod ldma;
#[doc = "GPCRC"]
pub struct GPCRC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPCRC {}
impl GPCRC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const gpcrc::RegisterBlock = 0x4001_c000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpcrc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for GPCRC {
    type Target = gpcrc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for GPCRC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPCRC").finish()
    }
}
#[doc = "GPCRC"]
pub mod gpcrc;
#[doc = "CAN0"]
pub struct CAN0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN0 {}
impl CAN0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const can0::RegisterBlock = 0x4000_4000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for CAN0 {
    type Target = can0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for CAN0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN0").finish()
    }
}
#[doc = "CAN0"]
pub mod can0;
#[doc = "TIMER0"]
pub struct TIMER0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER0 {}
impl TIMER0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const timer0::RegisterBlock = 0x4001_8000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TIMER0 {
    type Target = timer0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TIMER0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER0").finish()
    }
}
#[doc = "TIMER0"]
pub mod timer0;
#[doc = "TIMER1"]
pub struct TIMER1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER1 {}
impl TIMER1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const timer1::RegisterBlock = 0x4001_8400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TIMER1 {
    type Target = timer1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TIMER1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER1").finish()
    }
}
#[doc = "TIMER1"]
pub mod timer1;
#[doc = "WTIMER0"]
pub struct WTIMER0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WTIMER0 {}
impl WTIMER0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const wtimer0::RegisterBlock = 0x4001_a000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wtimer0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for WTIMER0 {
    type Target = wtimer0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for WTIMER0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WTIMER0").finish()
    }
}
#[doc = "WTIMER0"]
pub mod wtimer0;
#[doc = "WTIMER1"]
pub struct WTIMER1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WTIMER1 {}
impl WTIMER1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const wtimer1::RegisterBlock = 0x4001_a400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wtimer1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for WTIMER1 {
    type Target = wtimer1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for WTIMER1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WTIMER1").finish()
    }
}
#[doc = "WTIMER1"]
pub mod wtimer1;
#[doc = "USART0"]
pub struct USART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART0 {}
impl USART0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const usart0::RegisterBlock = 0x4001_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for USART0 {
    type Target = usart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for USART0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USART0").finish()
    }
}
#[doc = "USART0"]
pub mod usart0;
#[doc = "USART1"]
pub struct USART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART1 {}
impl USART1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const usart1::RegisterBlock = 0x4001_0400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for USART1 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for USART1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USART1").finish()
    }
}
#[doc = "USART1"]
pub mod usart1;
#[doc = "USART2"]
pub struct USART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART2 {}
impl USART2 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const usart2::RegisterBlock = 0x4001_0800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart2::RegisterBlock {
        Self::PTR
    }
}
impl Deref for USART2 {
    type Target = usart2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for USART2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USART2").finish()
    }
}
#[doc = "USART2"]
pub mod usart2;
#[doc = "USART3"]
pub struct USART3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART3 {}
impl USART3 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const usart3::RegisterBlock = 0x4001_0c00 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart3::RegisterBlock {
        Self::PTR
    }
}
impl Deref for USART3 {
    type Target = usart3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for USART3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USART3").finish()
    }
}
#[doc = "USART3"]
pub mod usart3;
#[doc = "UART0"]
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART0 {}
impl UART0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const uart0::RegisterBlock = 0x4001_4000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for UART0 {
    type Target = uart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for UART0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART0").finish()
    }
}
#[doc = "UART0"]
pub mod uart0;
#[doc = "LEUART0"]
pub struct LEUART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LEUART0 {}
impl LEUART0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const leuart0::RegisterBlock = 0x4006_a000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const leuart0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for LEUART0 {
    type Target = leuart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for LEUART0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LEUART0").finish()
    }
}
#[doc = "LEUART0"]
pub mod leuart0;
#[doc = "LETIMER0"]
pub struct LETIMER0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LETIMER0 {}
impl LETIMER0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const letimer0::RegisterBlock = 0x4006_6000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const letimer0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for LETIMER0 {
    type Target = letimer0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for LETIMER0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LETIMER0").finish()
    }
}
#[doc = "LETIMER0"]
pub mod letimer0;
#[doc = "CRYOTIMER"]
pub struct CRYOTIMER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRYOTIMER {}
impl CRYOTIMER {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const cryotimer::RegisterBlock = 0x4008_f000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cryotimer::RegisterBlock {
        Self::PTR
    }
}
impl Deref for CRYOTIMER {
    type Target = cryotimer::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for CRYOTIMER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRYOTIMER").finish()
    }
}
#[doc = "CRYOTIMER"]
pub mod cryotimer;
#[doc = "PCNT0"]
pub struct PCNT0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PCNT0 {}
impl PCNT0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const pcnt0::RegisterBlock = 0x4006_e000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pcnt0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for PCNT0 {
    type Target = pcnt0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for PCNT0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCNT0").finish()
    }
}
#[doc = "PCNT0"]
pub mod pcnt0;
#[doc = "I2C0"]
pub struct I2C0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C0 {}
impl I2C0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const i2c0::RegisterBlock = 0x4008_9000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for I2C0 {
    type Target = i2c0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for I2C0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C0").finish()
    }
}
#[doc = "I2C0"]
pub mod i2c0;
#[doc = "I2C1"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const i2c1::RegisterBlock = 0x4008_9400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for I2C1 {
    type Target = i2c1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for I2C1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C1").finish()
    }
}
#[doc = "I2C1"]
pub mod i2c1;
#[doc = "ADC0"]
pub struct ADC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC0 {}
impl ADC0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const adc0::RegisterBlock = 0x4008_2000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for ADC0 {
    type Target = adc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for ADC0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC0").finish()
    }
}
#[doc = "ADC0"]
pub mod adc0;
#[doc = "ACMP0"]
pub struct ACMP0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ACMP0 {}
impl ACMP0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const acmp0::RegisterBlock = 0x4008_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const acmp0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for ACMP0 {
    type Target = acmp0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for ACMP0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACMP0").finish()
    }
}
#[doc = "ACMP0"]
pub mod acmp0;
#[doc = "ACMP1"]
pub struct ACMP1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ACMP1 {}
impl ACMP1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const acmp1::RegisterBlock = 0x4008_0400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const acmp1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for ACMP1 {
    type Target = acmp1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for ACMP1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACMP1").finish()
    }
}
#[doc = "ACMP1"]
pub mod acmp1;
#[doc = "VDAC0"]
pub struct VDAC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VDAC0 {}
impl VDAC0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const vdac0::RegisterBlock = 0x4008_6000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const vdac0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for VDAC0 {
    type Target = vdac0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for VDAC0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VDAC0").finish()
    }
}
#[doc = "VDAC0"]
pub mod vdac0;
#[doc = "CSEN"]
pub struct CSEN {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CSEN {}
impl CSEN {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const csen::RegisterBlock = 0x4008_e000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const csen::RegisterBlock {
        Self::PTR
    }
}
impl Deref for CSEN {
    type Target = csen::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for CSEN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSEN").finish()
    }
}
#[doc = "CSEN"]
pub mod csen;
#[doc = "RTCC"]
pub struct RTCC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTCC {}
impl RTCC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const rtcc::RegisterBlock = 0x4006_2000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtcc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for RTCC {
    type Target = rtcc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for RTCC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTCC").finish()
    }
}
#[doc = "RTCC"]
pub mod rtcc;
#[doc = "WDOG0"]
pub struct WDOG0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDOG0 {}
impl WDOG0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const wdog0::RegisterBlock = 0x4005_2000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdog0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for WDOG0 {
    type Target = wdog0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for WDOG0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDOG0").finish()
    }
}
#[doc = "WDOG0"]
pub mod wdog0;
#[doc = "SMU"]
pub struct SMU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SMU {}
impl SMU {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const smu::RegisterBlock = 0x4002_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const smu::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SMU {
    type Target = smu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SMU {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMU").finish()
    }
}
#[doc = "SMU"]
pub mod smu;
#[doc = "TRNG0"]
pub struct TRNG0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TRNG0 {}
impl TRNG0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const trng0::RegisterBlock = 0x4001_d000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const trng0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TRNG0 {
    type Target = trng0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TRNG0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TRNG0").finish()
    }
}
#[doc = "TRNG0"]
pub mod trng0;
#[doc = "MTB"]
pub struct MTB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MTB {}
impl MTB {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const mtb::RegisterBlock = 0xf004_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mtb::RegisterBlock {
        Self::PTR
    }
}
impl Deref for MTB {
    type Target = mtb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for MTB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTB").finish()
    }
}
#[doc = "MTB"]
pub mod mtb;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals."]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "MSC"]
    pub MSC: MSC,
    #[doc = "EMU"]
    pub EMU: EMU,
    #[doc = "RMU"]
    pub RMU: RMU,
    #[doc = "CMU"]
    pub CMU: CMU,
    #[doc = "CRYPTO0"]
    pub CRYPTO0: CRYPTO0,
    #[doc = "LESENSE"]
    pub LESENSE: LESENSE,
    #[doc = "GPIO"]
    pub GPIO: GPIO,
    #[doc = "PRS"]
    pub PRS: PRS,
    #[doc = "LDMA"]
    pub LDMA: LDMA,
    #[doc = "GPCRC"]
    pub GPCRC: GPCRC,
    #[doc = "CAN0"]
    pub CAN0: CAN0,
    #[doc = "TIMER0"]
    pub TIMER0: TIMER0,
    #[doc = "TIMER1"]
    pub TIMER1: TIMER1,
    #[doc = "WTIMER0"]
    pub WTIMER0: WTIMER0,
    #[doc = "WTIMER1"]
    pub WTIMER1: WTIMER1,
    #[doc = "USART0"]
    pub USART0: USART0,
    #[doc = "USART1"]
    pub USART1: USART1,
    #[doc = "USART2"]
    pub USART2: USART2,
    #[doc = "USART3"]
    pub USART3: USART3,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "LEUART0"]
    pub LEUART0: LEUART0,
    #[doc = "LETIMER0"]
    pub LETIMER0: LETIMER0,
    #[doc = "CRYOTIMER"]
    pub CRYOTIMER: CRYOTIMER,
    #[doc = "PCNT0"]
    pub PCNT0: PCNT0,
    #[doc = "I2C0"]
    pub I2C0: I2C0,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "ADC0"]
    pub ADC0: ADC0,
    #[doc = "ACMP0"]
    pub ACMP0: ACMP0,
    #[doc = "ACMP1"]
    pub ACMP1: ACMP1,
    #[doc = "VDAC0"]
    pub VDAC0: VDAC0,
    #[doc = "CSEN"]
    pub CSEN: CSEN,
    #[doc = "RTCC"]
    pub RTCC: RTCC,
    #[doc = "WDOG0"]
    pub WDOG0: WDOG0,
    #[doc = "SMU"]
    pub SMU: SMU,
    #[doc = "TRNG0"]
    pub TRNG0: TRNG0,
    #[doc = "MTB"]
    pub MTB: MTB,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*."]
    #[cfg(feature = "critical-section")]
    #[inline]
    pub fn take() -> Option<Self> {
        critical_section::with(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                return None;
            }
            Some(unsafe { Peripherals::steal() })
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Each of the returned peripherals must be used at most once."]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            MSC: MSC {
                _marker: PhantomData,
            },
            EMU: EMU {
                _marker: PhantomData,
            },
            RMU: RMU {
                _marker: PhantomData,
            },
            CMU: CMU {
                _marker: PhantomData,
            },
            CRYPTO0: CRYPTO0 {
                _marker: PhantomData,
            },
            LESENSE: LESENSE {
                _marker: PhantomData,
            },
            GPIO: GPIO {
                _marker: PhantomData,
            },
            PRS: PRS {
                _marker: PhantomData,
            },
            LDMA: LDMA {
                _marker: PhantomData,
            },
            GPCRC: GPCRC {
                _marker: PhantomData,
            },
            CAN0: CAN0 {
                _marker: PhantomData,
            },
            TIMER0: TIMER0 {
                _marker: PhantomData,
            },
            TIMER1: TIMER1 {
                _marker: PhantomData,
            },
            WTIMER0: WTIMER0 {
                _marker: PhantomData,
            },
            WTIMER1: WTIMER1 {
                _marker: PhantomData,
            },
            USART0: USART0 {
                _marker: PhantomData,
            },
            USART1: USART1 {
                _marker: PhantomData,
            },
            USART2: USART2 {
                _marker: PhantomData,
            },
            USART3: USART3 {
                _marker: PhantomData,
            },
            UART0: UART0 {
                _marker: PhantomData,
            },
            LEUART0: LEUART0 {
                _marker: PhantomData,
            },
            LETIMER0: LETIMER0 {
                _marker: PhantomData,
            },
            CRYOTIMER: CRYOTIMER {
                _marker: PhantomData,
            },
            PCNT0: PCNT0 {
                _marker: PhantomData,
            },
            I2C0: I2C0 {
                _marker: PhantomData,
            },
            I2C1: I2C1 {
                _marker: PhantomData,
            },
            ADC0: ADC0 {
                _marker: PhantomData,
            },
            ACMP0: ACMP0 {
                _marker: PhantomData,
            },
            ACMP1: ACMP1 {
                _marker: PhantomData,
            },
            VDAC0: VDAC0 {
                _marker: PhantomData,
            },
            CSEN: CSEN {
                _marker: PhantomData,
            },
            RTCC: RTCC {
                _marker: PhantomData,
            },
            WDOG0: WDOG0 {
                _marker: PhantomData,
            },
            SMU: SMU {
                _marker: PhantomData,
            },
            TRNG0: TRNG0 {
                _marker: PhantomData,
            },
            MTB: MTB {
                _marker: PhantomData,
            },
        }
    }
}
