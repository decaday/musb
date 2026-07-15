
#[doc = "USB control and status registers for managing USB operations."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb {
    ptr: *mut u8,
}
unsafe impl Send for Usb {}
unsafe impl Sync for Usb {}
impl Usb {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Index register for selecting the endpoint status and control registers."]
    #[inline(always)]
    pub const fn index(self) -> crate::common::Reg<regs::Index, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0eusize) as _) }
    }
    #[doc = "Returns details of core configuration."]
    #[inline(always)]
    pub const fn configdata(self) -> crate::common::Reg<regs::Configdata, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1fusize) as _) }
    }
    #[doc = "FIFO Size Register for TX and RX Endpoints"]
    #[inline(always)]
    pub const fn fifosize(self) -> crate::common::Reg<regs::Fifosize, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1fusize) as _) }
    }
    #[doc = "Read-back of the number of TX and Rx endpoints."]
    #[inline(always)]
    pub const fn epinfo(self) -> crate::common::Reg<regs::Epinfo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "Provides information about the width of the RAM."]
    #[inline(always)]
    pub const fn raminfo(self) -> crate::common::Reg<regs::Raminfo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x79usize) as _) }
    }
    #[doc = "Allows some delays to be specified."]
    #[inline(always)]
    pub const fn linkinfo(self) -> crate::common::Reg<regs::Linkinfo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7ausize) as _) }
    }
    #[doc = "Sets the duration of the VBus pulsing charge."]
    #[inline(always)]
    pub const fn vplen(self) -> crate::common::Reg<regs::Vplen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7busize) as _) }
    }
    #[doc = "Sets the minimum time gap before EOF for High-speed transactions."]
    #[inline(always)]
    pub const fn hs_eof1(self) -> crate::common::Reg<regs::HsEof1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "Sets the minimum time gap before EOF for Full-speed transactions."]
    #[inline(always)]
    pub const fn fs_eof1(self) -> crate::common::Reg<regs::FsEof1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7dusize) as _) }
    }
    #[doc = "Sets the minimum time gap before EOF for Low-speed transactions."]
    #[inline(always)]
    pub const fn ls_eof1(self) -> crate::common::Reg<regs::LsEof1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7eusize) as _) }
    }
    #[doc = "Asserts LOW the output reset signals NRSTO and NRSTOX."]
    #[inline(always)]
    pub const fn soft_rst(self) -> crate::common::Reg<regs::SoftRst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7fusize) as _) }
    }
}
pub mod regs {
    #[doc = "Core configuration information register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Configdata(pub u8);
    impl Configdata {
        #[doc = "UTMI+ data width selection"]
        #[must_use]
        #[inline(always)]
        pub const fn utmi_data_width(&self) -> super::vals::UtmiWidth {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::UtmiWidth::from_bits(val as u8)
        }
        #[doc = "UTMI+ data width selection"]
        #[inline(always)]
        pub const fn set_utmi_data_width(&mut self, val: super::vals::UtmiWidth) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
        }
        #[doc = "Soft Connect/Disconnect feature"]
        #[must_use]
        #[inline(always)]
        pub const fn soft_con_e(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Soft Connect/Disconnect feature"]
        #[inline(always)]
        pub const fn set_soft_con_e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "Dynamic FIFO Sizing option"]
        #[must_use]
        #[inline(always)]
        pub const fn dyn_fifo_sizing(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Dynamic FIFO Sizing option"]
        #[inline(always)]
        pub const fn set_dyn_fifo_sizing(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "High-bandwidth TX ISO Endpoint Support"]
        #[must_use]
        #[inline(always)]
        pub const fn hbtxe(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "High-bandwidth TX ISO Endpoint Support"]
        #[inline(always)]
        pub const fn set_hbtxe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "High-bandwidth Rx ISO Endpoint Support"]
        #[must_use]
        #[inline(always)]
        pub const fn hbrxe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "High-bandwidth Rx ISO Endpoint Support"]
        #[inline(always)]
        pub const fn set_hbrxe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Endian ordering indicator"]
        #[must_use]
        #[inline(always)]
        pub const fn big_endian(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Endian ordering indicator"]
        #[inline(always)]
        pub const fn set_big_endian(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Automatic bulk packet splitting"]
        #[must_use]
        #[inline(always)]
        pub const fn mptxe(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic bulk packet splitting"]
        #[inline(always)]
        pub const fn set_mptxe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Automatic bulk packet amalgamation"]
        #[must_use]
        #[inline(always)]
        pub const fn mprxe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic bulk packet amalgamation"]
        #[inline(always)]
        pub const fn set_mprxe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Configdata {
        #[inline(always)]
        fn default() -> Configdata {
            Configdata(0)
        }
    }
    impl core::fmt::Debug for Configdata {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Configdata")
                .field("utmi_data_width", &self.utmi_data_width())
                .field("soft_con_e", &self.soft_con_e())
                .field("dyn_fifo_sizing", &self.dyn_fifo_sizing())
                .field("hbtxe", &self.hbtxe())
                .field("hbrxe", &self.hbrxe())
                .field("big_endian", &self.big_endian())
                .field("mptxe", &self.mptxe())
                .field("mprxe", &self.mprxe())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Configdata {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Configdata {{ utmi_data_width: {:?}, soft_con_e: {=bool:?}, dyn_fifo_sizing: {=bool:?}, hbtxe: {=bool:?}, hbrxe: {=bool:?}, big_endian: {=bool:?}, mptxe: {=bool:?}, mprxe: {=bool:?} }}" , self . utmi_data_width () , self . soft_con_e () , self . dyn_fifo_sizing () , self . hbtxe () , self . hbrxe () , self . big_endian () , self . mptxe () , self . mprxe ())
        }
    }
    #[doc = "Endpoint information register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Epinfo(pub u8);
    impl Epinfo {
        #[doc = "The number of TX endpoints implemented."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_end_points(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "The number of TX endpoints implemented."]
        #[inline(always)]
        pub const fn set_tx_end_points(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "The number of Rx endpoints implemented."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_end_points(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "The number of Rx endpoints implemented."]
        #[inline(always)]
        pub const fn set_rx_end_points(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
        }
    }
    impl Default for Epinfo {
        #[inline(always)]
        fn default() -> Epinfo {
            Epinfo(0)
        }
    }
    impl core::fmt::Debug for Epinfo {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Epinfo")
                .field("tx_end_points", &self.tx_end_points())
                .field("rx_end_points", &self.rx_end_points())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Epinfo {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Epinfo {{ tx_end_points: {=u8:?}, rx_end_points: {=u8:?} }}",
                self.tx_end_points(),
                self.rx_end_points()
            )
        }
    }
    #[doc = "FIFO Size Register for TX and RX Endpoints"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fifosize(pub u8);
    impl Fifosize {
        #[doc = "Size of the selected Tx endpoint FIFO (2^n bytes, 0 if not configured)"]
        #[must_use]
        #[inline(always)]
        pub const fn tx_fifo_size(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Size of the selected Tx endpoint FIFO (2^n bytes, 0 if not configured)"]
        #[inline(always)]
        pub const fn set_tx_fifo_size(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "Size of the selected Rx endpoint FIFO (2^n bytes, 0 if not configured)"]
        #[must_use]
        #[inline(always)]
        pub const fn rx_fifo_size(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Size of the selected Rx endpoint FIFO (2^n bytes, 0 if not configured)"]
        #[inline(always)]
        pub const fn set_rx_fifo_size(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
        }
    }
    impl Default for Fifosize {
        #[inline(always)]
        fn default() -> Fifosize {
            Fifosize(0)
        }
    }
    impl core::fmt::Debug for Fifosize {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fifosize")
                .field("tx_fifo_size", &self.tx_fifo_size())
                .field("rx_fifo_size", &self.rx_fifo_size())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fifosize {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Fifosize {{ tx_fifo_size: {=u8:?}, rx_fifo_size: {=u8:?} }}",
                self.tx_fifo_size(),
                self.rx_fifo_size()
            )
        }
    }
    #[doc = "Full-speed end of frame time gap."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FsEof1(pub u8);
    impl FsEof1 {
        #[doc = "Time before EOF to stop beginning new transactions for Full-speed, in units of 533.3ns."]
        #[must_use]
        #[inline(always)]
        pub const fn fs_eof1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Time before EOF to stop beginning new transactions for Full-speed, in units of 533.3ns."]
        #[inline(always)]
        pub const fn set_fs_eof1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for FsEof1 {
        #[inline(always)]
        fn default() -> FsEof1 {
            FsEof1(0)
        }
    }
    impl core::fmt::Debug for FsEof1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FsEof1")
                .field("fs_eof1", &self.fs_eof1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FsEof1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FsEof1 {{ fs_eof1: {=u8:?} }}", self.fs_eof1())
        }
    }
    #[doc = "High-speed end of frame time gap."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HsEof1(pub u8);
    impl HsEof1 {
        #[doc = "Time before EOF to stop beginning new transactions for High-speed, in units of 133.3ns."]
        #[must_use]
        #[inline(always)]
        pub const fn hs_eof1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Time before EOF to stop beginning new transactions for High-speed, in units of 133.3ns."]
        #[inline(always)]
        pub const fn set_hs_eof1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for HsEof1 {
        #[inline(always)]
        fn default() -> HsEof1 {
            HsEof1(0)
        }
    }
    impl core::fmt::Debug for HsEof1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HsEof1")
                .field("hs_eof1", &self.hs_eof1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HsEof1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HsEof1 {{ hs_eof1: {=u8:?} }}", self.hs_eof1())
        }
    }
    #[doc = "Endpoint index selection register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Index(pub u8);
    impl Index {
        #[doc = "Selects which endpoint control/status registers are accessed"]
        #[must_use]
        #[inline(always)]
        pub const fn index(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Selects which endpoint control/status registers are accessed"]
        #[inline(always)]
        pub const fn set_index(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
    }
    impl Default for Index {
        #[inline(always)]
        fn default() -> Index {
            Index(0)
        }
    }
    impl core::fmt::Debug for Index {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Index")
                .field("index", &self.index())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Index {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Index {{ index: {=u8:?} }}", self.index())
        }
    }
    #[doc = "Link information and delay specification."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Linkinfo(pub u8);
    impl Linkinfo {
        #[doc = "Sets the delay from IDPULLUP assertion to IDDIG valid in units of 4.369ms."]
        #[must_use]
        #[inline(always)]
        pub const fn wtid(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Sets the delay from IDPULLUP assertion to IDDIG valid in units of 4.369ms."]
        #[inline(always)]
        pub const fn set_wtid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "Sets the wait for connect/disconnect filter in units of 533.3ns."]
        #[must_use]
        #[inline(always)]
        pub const fn wtcon(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Sets the wait for connect/disconnect filter in units of 533.3ns."]
        #[inline(always)]
        pub const fn set_wtcon(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
        }
    }
    impl Default for Linkinfo {
        #[inline(always)]
        fn default() -> Linkinfo {
            Linkinfo(0)
        }
    }
    impl core::fmt::Debug for Linkinfo {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Linkinfo")
                .field("wtid", &self.wtid())
                .field("wtcon", &self.wtcon())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Linkinfo {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Linkinfo {{ wtid: {=u8:?}, wtcon: {=u8:?} }}",
                self.wtid(),
                self.wtcon()
            )
        }
    }
    #[doc = "Low-speed end of frame time gap."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LsEof1(pub u8);
    impl LsEof1 {
        #[doc = "Time before EOF to stop beginning new transactions for Low-speed, in units of 1.067us."]
        #[must_use]
        #[inline(always)]
        pub const fn ls_eof1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Time before EOF to stop beginning new transactions for Low-speed, in units of 1.067us."]
        #[inline(always)]
        pub const fn set_ls_eof1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for LsEof1 {
        #[inline(always)]
        fn default() -> LsEof1 {
            LsEof1(0)
        }
    }
    impl core::fmt::Debug for LsEof1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LsEof1")
                .field("ls_eof1", &self.ls_eof1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LsEof1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "LsEof1 {{ ls_eof1: {=u8:?} }}", self.ls_eof1())
        }
    }
    #[doc = "Provides information about the RAM."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Raminfo(pub u8);
    impl Raminfo {
        #[doc = "The width of the RAM address bus."]
        #[must_use]
        #[inline(always)]
        pub const fn ram_bits(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "The width of the RAM address bus."]
        #[inline(always)]
        pub const fn set_ram_bits(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "The number of DMA channels implemented."]
        #[must_use]
        #[inline(always)]
        pub const fn dmachans(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "The number of DMA channels implemented."]
        #[inline(always)]
        pub const fn set_dmachans(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
        }
    }
    impl Default for Raminfo {
        #[inline(always)]
        fn default() -> Raminfo {
            Raminfo(0)
        }
    }
    impl core::fmt::Debug for Raminfo {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Raminfo")
                .field("ram_bits", &self.ram_bits())
                .field("dmachans", &self.dmachans())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Raminfo {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Raminfo {{ ram_bits: {=u8:?}, dmachans: {=u8:?} }}",
                self.ram_bits(),
                self.dmachans()
            )
        }
    }
    #[doc = "Software reset control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SoftRst(pub u8);
    impl SoftRst {
        #[doc = "Writing 1 asserts the NRSTO output LOW."]
        #[must_use]
        #[inline(always)]
        pub const fn nrst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Writing 1 asserts the NRSTO output LOW."]
        #[inline(always)]
        pub const fn set_nrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Writing 1 asserts the NRSTXO output LOW."]
        #[must_use]
        #[inline(always)]
        pub const fn nrstx(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Writing 1 asserts the NRSTXO output LOW."]
        #[inline(always)]
        pub const fn set_nrstx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
    }
    impl Default for SoftRst {
        #[inline(always)]
        fn default() -> SoftRst {
            SoftRst(0)
        }
    }
    impl core::fmt::Debug for SoftRst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SoftRst")
                .field("nrst", &self.nrst())
                .field("nrstx", &self.nrstx())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SoftRst {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SoftRst {{ nrst: {=bool:?}, nrstx: {=bool:?} }}",
                self.nrst(),
                self.nrstx()
            )
        }
    }
    #[doc = "VBus pulsing charge duration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vplen(pub u8);
    impl Vplen {
        #[doc = "Sets the duration of VBus pulsing charge in units of 546.1 µs."]
        #[must_use]
        #[inline(always)]
        pub const fn vplen(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Sets the duration of VBus pulsing charge in units of 546.1 µs."]
        #[inline(always)]
        pub const fn set_vplen(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Vplen {
        #[inline(always)]
        fn default() -> Vplen {
            Vplen(0)
        }
    }
    impl core::fmt::Debug for Vplen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vplen")
                .field("vplen", &self.vplen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vplen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Vplen {{ vplen: {=u8:?} }}", self.vplen())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum UtmiWidth {
        EightBit = 0x0,
        SixteenBit = 0x01,
    }
    impl UtmiWidth {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> UtmiWidth {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for UtmiWidth {
        #[inline(always)]
        fn from(val: u8) -> UtmiWidth {
            UtmiWidth::from_bits(val)
        }
    }
    impl From<UtmiWidth> for u8 {
        #[inline(always)]
        fn from(val: UtmiWidth) -> u8 {
            UtmiWidth::to_bits(val)
        }
    }
}
