
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
    #[doc = "Function address register."]
    #[inline(always)]
    pub const fn faddr(self) -> crate::common::Reg<regs::Faddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Power management register."]
    #[inline(always)]
    pub const fn power(self) -> crate::common::Reg<regs::Power, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01usize) as _) }
    }
    #[doc = "Interrupt register for Endpoint 0 plus TX Endpoints 1 to 15."]
    #[inline(always)]
    pub const fn intrtx(self) -> crate::common::Reg<regs::Intrtx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "Interrupt register for Rx Endpoints 1 to 15."]
    #[inline(always)]
    pub const fn intrrx(self) -> crate::common::Reg<regs::Intrrx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Interrupt enable register for INTRTX."]
    #[inline(always)]
    pub const fn intrtxe(self) -> crate::common::Reg<regs::Intrtxe, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "Interrupt enable register for INTRRX."]
    #[inline(always)]
    pub const fn intrrxe(self) -> crate::common::Reg<regs::Intrrxe, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Interrupt register for common USB interrupts."]
    #[inline(always)]
    pub const fn intrusb(self) -> crate::common::Reg<regs::Intrusb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "Interrupt enable register for INTRUSB."]
    #[inline(always)]
    pub const fn intrusbe(self) -> crate::common::Reg<regs::Intrusbe, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0busize) as _) }
    }
    #[doc = "Frame number."]
    #[inline(always)]
    pub const fn frame(self) -> crate::common::Reg<regs::Frame, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Index register for selecting the endpoint status and control registers."]
    #[inline(always)]
    pub const fn index(self) -> crate::common::Reg<regs::Index, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0eusize) as _) }
    }
    #[doc = "Enables the USB 2.0 test modes."]
    #[inline(always)]
    pub const fn testmode(self) -> crate::common::Reg<regs::Testmode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fusize) as _) }
    }
    #[doc = "Maximum packet size for peripheral TX endpoint."]
    #[inline(always)]
    pub const fn txmaxp(self) -> crate::common::Reg<regs::Maxp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Control Status register lower byte for Endpoint 0."]
    #[inline(always)]
    pub const fn csr0l(self) -> crate::common::Reg<regs::Csr0l, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12usize) as _) }
    }
    #[doc = "Control Status register lower byte for peripheral TX endpoint."]
    #[inline(always)]
    pub const fn txcsrl(self) -> crate::common::Reg<regs::Txcsrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12usize) as _) }
    }
    #[doc = "Control Status register higher byte for Endpoint 0."]
    #[inline(always)]
    pub const fn csr0h(self) -> crate::common::Reg<regs::Csr0h, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13usize) as _) }
    }
    #[doc = "Control Status register higher byte for peripheral TX endpoint."]
    #[inline(always)]
    pub const fn txcsrh(self) -> crate::common::Reg<regs::Txcsrh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13usize) as _) }
    }
    #[doc = "Maximum packet size for peripheral Rx endpoint."]
    #[inline(always)]
    pub const fn rxmaxp(self) -> crate::common::Reg<regs::Maxp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Control Status register lower byte for peripheral Rx endpoint."]
    #[inline(always)]
    pub const fn rxcsrl(self) -> crate::common::Reg<regs::Rxcsrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x16usize) as _) }
    }
    #[doc = "Control Status register higher byte for peripheral Rx endpoint."]
    #[inline(always)]
    pub const fn rxcsrh(self) -> crate::common::Reg<regs::Rxcsrh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x17usize) as _) }
    }
    #[doc = "Number of received bytes in Endpoint 0 FIFO."]
    #[inline(always)]
    pub const fn count0(self) -> crate::common::Reg<regs::Count0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Number of bytes to be read from peripheral Rx endpoint FIFO."]
    #[inline(always)]
    pub const fn rxcount(self) -> crate::common::Reg<regs::Rxcount, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
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
    #[doc = "FIFO for endpoints."]
    #[inline(always)]
    pub const fn fifo(self, n: usize) -> crate::common::Reg<regs::Fifo, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize + n * 4usize) as _) }
    }
    #[doc = "used to select whether the MUSBMHDRC is operating in Peripheral mode or in Host mode, and for controlling and monitoring the USB VBus line."]
    #[inline(always)]
    pub const fn devctl(self) -> crate::common::Reg<regs::Devctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "controls the size of the selected TX endpoint FIFO"]
    #[inline(always)]
    pub const fn tx_fifo_sz(self) -> crate::common::Reg<regs::FifoSz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x62usize) as _) }
    }
    #[doc = "controls the size of the selected Rx endpoint FIFO"]
    #[inline(always)]
    pub const fn rx_fifo_sz(self) -> crate::common::Reg<regs::FifoSz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x63usize) as _) }
    }
    #[doc = "controls the start address of the selected Tx endpoint FIFO"]
    #[inline(always)]
    pub const fn tx_fifo_add(self) -> crate::common::Reg<regs::FifoAdd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "controls the start address of the selected Rx endpoint FIFO"]
    #[inline(always)]
    pub const fn rx_fifo_add(self) -> crate::common::Reg<regs::FifoAdd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x66usize) as _) }
    }
    #[doc = "Double Packet Buffer Disable register."]
    #[inline(always)]
    pub const fn tx_dpktbufdis(self) -> crate::common::Reg<regs::Dpktbufdis, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0342usize) as _) }
    }
    #[doc = "Double Packet Buffer Disable register."]
    #[inline(always)]
    pub const fn rx_dpktbufdis(self) -> crate::common::Reg<regs::Dpktbufdis, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0344usize) as _) }
    }
    #[doc = "Vender-specified USB configuration register."]
    #[inline(always)]
    pub const fn usbcfg(self) -> crate::common::Reg<regs::Usbcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0370usize) as _) }
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
    #[doc = "USB Endpoint 0 Received Data Byte Count"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Count0(pub u8);
    impl Count0 {
        #[doc = "Number of received data bytes in FIFO"]
        #[must_use]
        #[inline(always)]
        pub const fn count(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Number of received data bytes in FIFO"]
        #[inline(always)]
        pub const fn set_count(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u8) & 0x7f) << 0usize);
        }
    }
    impl Default for Count0 {
        #[inline(always)]
        fn default() -> Count0 {
            Count0(0)
        }
    }
    impl core::fmt::Debug for Count0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Count0")
                .field("count", &self.count())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Count0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Count0 {{ count: {=u8:?} }}", self.count())
        }
    }
    #[doc = "USB Endpoint 0 Control and Status Register High"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr0h(pub u8);
    impl Csr0h {
        #[doc = "Reset FIFO pointer and clear packet ready status"]
        #[must_use]
        #[inline(always)]
        pub const fn flush_fifo(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Reset FIFO pointer and clear packet ready status"]
        #[inline(always)]
        pub const fn set_flush_fifo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
    }
    impl Default for Csr0h {
        #[inline(always)]
        fn default() -> Csr0h {
            Csr0h(0)
        }
    }
    impl core::fmt::Debug for Csr0h {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csr0h")
                .field("flush_fifo", &self.flush_fifo())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csr0h {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Csr0h {{ flush_fifo: {=bool:?} }}", self.flush_fifo())
        }
    }
    #[doc = "USB Endpoint 0 Control and Status Register Low"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr0l(pub u8);
    impl Csr0l {
        #[doc = "Indicates received data packet ready for processing"]
        #[must_use]
        #[inline(always)]
        pub const fn rx_pkt_rdy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates received data packet ready for processing"]
        #[inline(always)]
        pub const fn set_rx_pkt_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Indicates data packet loaded in FIFO ready for transmission"]
        #[must_use]
        #[inline(always)]
        pub const fn tx_pkt_rdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates data packet loaded in FIFO ready for transmission"]
        #[inline(always)]
        pub const fn set_tx_pkt_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "Set when STALL handshake is transmitted"]
        #[must_use]
        #[inline(always)]
        pub const fn sent_stall(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Set when STALL handshake is transmitted"]
        #[inline(always)]
        pub const fn set_sent_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "Marks the end of data transfer"]
        #[must_use]
        #[inline(always)]
        pub const fn data_end(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Marks the end of data transfer"]
        #[inline(always)]
        pub const fn set_data_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Control transaction ended prematurely"]
        #[must_use]
        #[inline(always)]
        pub const fn setup_end(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Control transaction ended prematurely"]
        #[inline(always)]
        pub const fn set_setup_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Terminate current transaction with STALL handshake"]
        #[must_use]
        #[inline(always)]
        pub const fn send_stall(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Terminate current transaction with STALL handshake"]
        #[inline(always)]
        pub const fn set_send_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Clear RxPktRdy bit"]
        #[must_use]
        #[inline(always)]
        pub const fn serviced_rx_pkt_rdy(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Clear RxPktRdy bit"]
        #[inline(always)]
        pub const fn set_serviced_rx_pkt_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Clear SetupEnd bit"]
        #[must_use]
        #[inline(always)]
        pub const fn serviced_setup_end(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Clear SetupEnd bit"]
        #[inline(always)]
        pub const fn set_serviced_setup_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Csr0l {
        #[inline(always)]
        fn default() -> Csr0l {
            Csr0l(0)
        }
    }
    impl core::fmt::Debug for Csr0l {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csr0l")
                .field("rx_pkt_rdy", &self.rx_pkt_rdy())
                .field("tx_pkt_rdy", &self.tx_pkt_rdy())
                .field("sent_stall", &self.sent_stall())
                .field("data_end", &self.data_end())
                .field("setup_end", &self.setup_end())
                .field("send_stall", &self.send_stall())
                .field("serviced_rx_pkt_rdy", &self.serviced_rx_pkt_rdy())
                .field("serviced_setup_end", &self.serviced_setup_end())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csr0l {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Csr0l {{ rx_pkt_rdy: {=bool:?}, tx_pkt_rdy: {=bool:?}, sent_stall: {=bool:?}, data_end: {=bool:?}, setup_end: {=bool:?}, send_stall: {=bool:?}, serviced_rx_pkt_rdy: {=bool:?}, serviced_setup_end: {=bool:?} }}" , self . rx_pkt_rdy () , self . tx_pkt_rdy () , self . sent_stall () , self . data_end () , self . setup_end () , self . send_stall () , self . serviced_rx_pkt_rdy () , self . serviced_setup_end ())
        }
    }
    #[doc = "Device Control Register for USB mode and VBus monitoring"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Devctl(pub u8);
    impl Devctl {
        #[doc = "Control or monitor USB session state"]
        #[must_use]
        #[inline(always)]
        pub const fn session(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Control or monitor USB session state"]
        #[inline(always)]
        pub const fn set_session(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Initiate Host Negotiation Protocol"]
        #[must_use]
        #[inline(always)]
        pub const fn host_req(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Initiate Host Negotiation Protocol"]
        #[inline(always)]
        pub const fn set_host_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "Indicates USB Host mode operation"]
        #[must_use]
        #[inline(always)]
        pub const fn host_mode(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates USB Host mode operation"]
        #[inline(always)]
        pub const fn set_host_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "VBus voltage level indication"]
        #[must_use]
        #[inline(always)]
        pub const fn vbus(&self) -> super::vals::VbusLevel {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::VbusLevel::from_bits(val as u8)
        }
        #[doc = "VBus voltage level indication"]
        #[inline(always)]
        pub const fn set_vbus(&mut self, val: super::vals::VbusLevel) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u8) & 0x03) << 3usize);
        }
        #[doc = "Low-speed device detection"]
        #[must_use]
        #[inline(always)]
        pub const fn ls_dev(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Low-speed device detection"]
        #[inline(always)]
        pub const fn set_ls_dev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Full-speed or high-speed device detection"]
        #[must_use]
        #[inline(always)]
        pub const fn fs_dev(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Full-speed or high-speed device detection"]
        #[inline(always)]
        pub const fn set_fs_dev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Indicates device type in USB session"]
        #[must_use]
        #[inline(always)]
        pub const fn b_device(&self) -> super::vals::DeviceType {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::DeviceType::from_bits(val as u8)
        }
        #[doc = "Indicates device type in USB session"]
        #[inline(always)]
        pub const fn set_b_device(&mut self, val: super::vals::DeviceType) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Devctl {
        #[inline(always)]
        fn default() -> Devctl {
            Devctl(0)
        }
    }
    impl core::fmt::Debug for Devctl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Devctl")
                .field("session", &self.session())
                .field("host_req", &self.host_req())
                .field("host_mode", &self.host_mode())
                .field("vbus", &self.vbus())
                .field("ls_dev", &self.ls_dev())
                .field("fs_dev", &self.fs_dev())
                .field("b_device", &self.b_device())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Devctl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Devctl {{ session: {=bool:?}, host_req: {=bool:?}, host_mode: {=bool:?}, vbus: {:?}, ls_dev: {=bool:?}, fs_dev: {=bool:?}, b_device: {:?} }}" , self . session () , self . host_req () , self . host_mode () , self . vbus () , self . ls_dev () , self . fs_dev () , self . b_device ())
        }
    }
    #[doc = "Indicates which of the endpoints have disabled the double packet buffer functionality described in section 8.4.2.2 of the MUSBMHDRC Product Specification"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dpktbufdis(pub u16);
    impl Dpktbufdis {
        #[doc = "Double Packet Buffer Disable for Tx/Rx Endpoint x (except EP0)"]
        #[must_use]
        #[inline(always)]
        pub const fn dis(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Double Packet Buffer Disable for Tx/Rx Endpoint x (except EP0)"]
        #[inline(always)]
        pub const fn set_dis(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u16) & 0x01) << offs);
        }
    }
    impl Default for Dpktbufdis {
        #[inline(always)]
        fn default() -> Dpktbufdis {
            Dpktbufdis(0)
        }
    }
    impl core::fmt::Debug for Dpktbufdis {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dpktbufdis")
                .field("dis[0]", &self.dis(0usize))
                .field("dis[1]", &self.dis(1usize))
                .field("dis[2]", &self.dis(2usize))
                .field("dis[3]", &self.dis(3usize))
                .field("dis[4]", &self.dis(4usize))
                .field("dis[5]", &self.dis(5usize))
                .field("dis[6]", &self.dis(6usize))
                .field("dis[7]", &self.dis(7usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dpktbufdis {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Dpktbufdis {{ dis[0]: {=bool:?}, dis[1]: {=bool:?}, dis[2]: {=bool:?}, dis[3]: {=bool:?}, dis[4]: {=bool:?}, dis[5]: {=bool:?}, dis[6]: {=bool:?}, dis[7]: {=bool:?} }}" , self . dis (0usize) , self . dis (1usize) , self . dis (2usize) , self . dis (3usize) , self . dis (4usize) , self . dis (5usize) , self . dis (6usize) , self . dis (7usize))
        }
    }
    #[doc = "Function Address Register for USB device addressing"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Faddr(pub u8);
    impl Faddr {
        #[doc = "USB device function address"]
        #[must_use]
        #[inline(always)]
        pub const fn func_addr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "USB device function address"]
        #[inline(always)]
        pub const fn set_func_addr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u8) & 0x7f) << 0usize);
        }
    }
    impl Default for Faddr {
        #[inline(always)]
        fn default() -> Faddr {
            Faddr(0)
        }
    }
    impl core::fmt::Debug for Faddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Faddr")
                .field("func_addr", &self.func_addr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Faddr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Faddr {{ func_addr: {=u8:?} }}", self.func_addr())
        }
    }
    #[doc = "FIFO Data Access Register for Endpoints"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fifo(pub u8);
    impl Fifo {
        #[doc = "Data byte for FIFO read/write operation"]
        #[must_use]
        #[inline(always)]
        pub const fn data(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Data byte for FIFO read/write operation"]
        #[inline(always)]
        pub const fn set_data(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Fifo {
        #[inline(always)]
        fn default() -> Fifo {
            Fifo(0)
        }
    }
    impl core::fmt::Debug for Fifo {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fifo").field("data", &self.data()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fifo {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Fifo {{ data: {=u8:?} }}", self.data())
        }
    }
    #[doc = "controls the start address of the selected endpoint FIFO"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FifoAdd(pub u16);
    impl FifoAdd {
        #[doc = "Start address of the endpoint FIFO in units of 8 bytes"]
        #[must_use]
        #[inline(always)]
        pub const fn add(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "Start address of the endpoint FIFO in units of 8 bytes"]
        #[inline(always)]
        pub const fn set_add(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u16) & 0x1fff) << 0usize);
        }
    }
    impl Default for FifoAdd {
        #[inline(always)]
        fn default() -> FifoAdd {
            FifoAdd(0)
        }
    }
    impl core::fmt::Debug for FifoAdd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FifoAdd").field("add", &self.add()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FifoAdd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FifoAdd {{ add: {=u16:?} }}", self.add())
        }
    }
    #[doc = "controls the size of the selected endpoint FIFO"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FifoSz(pub u8);
    impl FifoSz {
        #[doc = "Maximum packet size to be allowed for (before any splitting within the FIFO of Bulk/High Bandwidth packets prior to transmission – see Sections 8.4.1.3, 8.4.1.4 and 8.5.3)"]
        #[must_use]
        #[inline(always)]
        pub const fn sz(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Maximum packet size to be allowed for (before any splitting within the FIFO of Bulk/High Bandwidth packets prior to transmission – see Sections 8.4.1.3, 8.4.1.4 and 8.5.3)"]
        #[inline(always)]
        pub const fn set_sz(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u8) & 0x07) << 0usize);
        }
        #[doc = "Defines whether double-packet buffering supported"]
        #[must_use]
        #[inline(always)]
        pub const fn dpb(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Defines whether double-packet buffering supported"]
        #[inline(always)]
        pub const fn set_dpb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
    }
    impl Default for FifoSz {
        #[inline(always)]
        fn default() -> FifoSz {
            FifoSz(0)
        }
    }
    impl core::fmt::Debug for FifoSz {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FifoSz")
                .field("sz", &self.sz())
                .field("dpb", &self.dpb())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FifoSz {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FifoSz {{ sz: {=u8:?}, dpb: {=bool:?} }}",
                self.sz(),
                self.dpb()
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
    #[doc = "Last received USB frame number"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Frame(pub u16);
    impl Frame {
        #[doc = "USB frame number"]
        #[must_use]
        #[inline(always)]
        pub const fn frame(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "USB frame number"]
        #[inline(always)]
        pub const fn set_frame(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
        }
    }
    impl Default for Frame {
        #[inline(always)]
        fn default() -> Frame {
            Frame(0)
        }
    }
    impl core::fmt::Debug for Frame {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Frame")
                .field("frame", &self.frame())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Frame {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Frame {{ frame: {=u16:?} }}", self.frame())
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
    #[doc = "Receive Endpoint Interrupt Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intrrx(pub u16);
    impl Intrrx {
        #[doc = "Receive endpoint interrupt (except EP0)"]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Receive endpoint interrupt (except EP0)"]
        #[inline(always)]
        pub const fn set_ep_rx(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u16) & 0x01) << offs);
        }
    }
    impl Default for Intrrx {
        #[inline(always)]
        fn default() -> Intrrx {
            Intrrx(0)
        }
    }
    impl core::fmt::Debug for Intrrx {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intrrx")
                .field("ep_rx[0]", &self.ep_rx(0usize))
                .field("ep_rx[1]", &self.ep_rx(1usize))
                .field("ep_rx[2]", &self.ep_rx(2usize))
                .field("ep_rx[3]", &self.ep_rx(3usize))
                .field("ep_rx[4]", &self.ep_rx(4usize))
                .field("ep_rx[5]", &self.ep_rx(5usize))
                .field("ep_rx[6]", &self.ep_rx(6usize))
                .field("ep_rx[7]", &self.ep_rx(7usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Intrrx {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Intrrx {{ ep_rx[0]: {=bool:?}, ep_rx[1]: {=bool:?}, ep_rx[2]: {=bool:?}, ep_rx[3]: {=bool:?}, ep_rx[4]: {=bool:?}, ep_rx[5]: {=bool:?}, ep_rx[6]: {=bool:?}, ep_rx[7]: {=bool:?} }}" , self . ep_rx (0usize) , self . ep_rx (1usize) , self . ep_rx (2usize) , self . ep_rx (3usize) , self . ep_rx (4usize) , self . ep_rx (5usize) , self . ep_rx (6usize) , self . ep_rx (7usize))
        }
    }
    #[doc = "Receive Endpoint Interrupt Enable Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intrrxe(pub u16);
    impl Intrrxe {
        #[doc = "Endpoint transmit interrupt enable (except EP0)"]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rxe(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Endpoint transmit interrupt enable (except EP0)"]
        #[inline(always)]
        pub const fn set_ep_rxe(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u16) & 0x01) << offs);
        }
    }
    impl Default for Intrrxe {
        #[inline(always)]
        fn default() -> Intrrxe {
            Intrrxe(0)
        }
    }
    impl core::fmt::Debug for Intrrxe {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intrrxe")
                .field("ep_rxe[0]", &self.ep_rxe(0usize))
                .field("ep_rxe[1]", &self.ep_rxe(1usize))
                .field("ep_rxe[2]", &self.ep_rxe(2usize))
                .field("ep_rxe[3]", &self.ep_rxe(3usize))
                .field("ep_rxe[4]", &self.ep_rxe(4usize))
                .field("ep_rxe[5]", &self.ep_rxe(5usize))
                .field("ep_rxe[6]", &self.ep_rxe(6usize))
                .field("ep_rxe[7]", &self.ep_rxe(7usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Intrrxe {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Intrrxe {{ ep_rxe[0]: {=bool:?}, ep_rxe[1]: {=bool:?}, ep_rxe[2]: {=bool:?}, ep_rxe[3]: {=bool:?}, ep_rxe[4]: {=bool:?}, ep_rxe[5]: {=bool:?}, ep_rxe[6]: {=bool:?}, ep_rxe[7]: {=bool:?} }}" , self . ep_rxe (0usize) , self . ep_rxe (1usize) , self . ep_rxe (2usize) , self . ep_rxe (3usize) , self . ep_rxe (4usize) , self . ep_rxe (5usize) , self . ep_rxe (6usize) , self . ep_rxe (7usize))
        }
    }
    #[doc = "Transmit Endpoint Interrupt Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intrtx(pub u16);
    impl Intrtx {
        #[doc = "Endpoint 0 and transmit endpoints interrupt"]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Endpoint 0 and transmit endpoints interrupt"]
        #[inline(always)]
        pub const fn set_ep_tx(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u16) & 0x01) << offs);
        }
    }
    impl Default for Intrtx {
        #[inline(always)]
        fn default() -> Intrtx {
            Intrtx(0)
        }
    }
    impl core::fmt::Debug for Intrtx {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intrtx")
                .field("ep_tx[0]", &self.ep_tx(0usize))
                .field("ep_tx[1]", &self.ep_tx(1usize))
                .field("ep_tx[2]", &self.ep_tx(2usize))
                .field("ep_tx[3]", &self.ep_tx(3usize))
                .field("ep_tx[4]", &self.ep_tx(4usize))
                .field("ep_tx[5]", &self.ep_tx(5usize))
                .field("ep_tx[6]", &self.ep_tx(6usize))
                .field("ep_tx[7]", &self.ep_tx(7usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Intrtx {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Intrtx {{ ep_tx[0]: {=bool:?}, ep_tx[1]: {=bool:?}, ep_tx[2]: {=bool:?}, ep_tx[3]: {=bool:?}, ep_tx[4]: {=bool:?}, ep_tx[5]: {=bool:?}, ep_tx[6]: {=bool:?}, ep_tx[7]: {=bool:?} }}" , self . ep_tx (0usize) , self . ep_tx (1usize) , self . ep_tx (2usize) , self . ep_tx (3usize) , self . ep_tx (4usize) , self . ep_tx (5usize) , self . ep_tx (6usize) , self . ep_tx (7usize))
        }
    }
    #[doc = "Transmit Endpoint Interrupt Enable Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intrtxe(pub u16);
    impl Intrtxe {
        #[doc = "Endpoint transmit interrupt enable (EP0:TXE_RXE)"]
        #[must_use]
        #[inline(always)]
        pub const fn ep_txe(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Endpoint transmit interrupt enable (EP0:TXE_RXE)"]
        #[inline(always)]
        pub const fn set_ep_txe(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u16) & 0x01) << offs);
        }
    }
    impl Default for Intrtxe {
        #[inline(always)]
        fn default() -> Intrtxe {
            Intrtxe(0)
        }
    }
    impl core::fmt::Debug for Intrtxe {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intrtxe")
                .field("ep_txe[0]", &self.ep_txe(0usize))
                .field("ep_txe[1]", &self.ep_txe(1usize))
                .field("ep_txe[2]", &self.ep_txe(2usize))
                .field("ep_txe[3]", &self.ep_txe(3usize))
                .field("ep_txe[4]", &self.ep_txe(4usize))
                .field("ep_txe[5]", &self.ep_txe(5usize))
                .field("ep_txe[6]", &self.ep_txe(6usize))
                .field("ep_txe[7]", &self.ep_txe(7usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Intrtxe {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Intrtxe {{ ep_txe[0]: {=bool:?}, ep_txe[1]: {=bool:?}, ep_txe[2]: {=bool:?}, ep_txe[3]: {=bool:?}, ep_txe[4]: {=bool:?}, ep_txe[5]: {=bool:?}, ep_txe[6]: {=bool:?}, ep_txe[7]: {=bool:?} }}" , self . ep_txe (0usize) , self . ep_txe (1usize) , self . ep_txe (2usize) , self . ep_txe (3usize) , self . ep_txe (4usize) , self . ep_txe (5usize) , self . ep_txe (6usize) , self . ep_txe (7usize))
        }
    }
    #[doc = "USB Interrupt Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intrusb(pub u8);
    impl Intrusb {
        #[doc = "Suspend signaling detected"]
        #[must_use]
        #[inline(always)]
        pub const fn suspend(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Suspend signaling detected"]
        #[inline(always)]
        pub const fn set_suspend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Resume signaling detected during Suspend"]
        #[must_use]
        #[inline(always)]
        pub const fn resume(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Resume signaling detected during Suspend"]
        #[inline(always)]
        pub const fn set_resume(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "Reset signaling detected (Peripheral mode) or Babble detected (Host mode)"]
        #[must_use]
        #[inline(always)]
        pub const fn reset(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Reset signaling detected (Peripheral mode) or Babble detected (Host mode)"]
        #[inline(always)]
        pub const fn set_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "New frame start"]
        #[must_use]
        #[inline(always)]
        pub const fn sof(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "New frame start"]
        #[inline(always)]
        pub const fn set_sof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Device connection detected"]
        #[must_use]
        #[inline(always)]
        pub const fn conn(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Device connection detected"]
        #[inline(always)]
        pub const fn set_conn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Device disconnection detected"]
        #[must_use]
        #[inline(always)]
        pub const fn discon(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Device disconnection detected"]
        #[inline(always)]
        pub const fn set_discon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Session Request signaling detected"]
        #[must_use]
        #[inline(always)]
        pub const fn sess_req(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Session Request signaling detected"]
        #[inline(always)]
        pub const fn set_sess_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "VBus drops below valid threshold"]
        #[must_use]
        #[inline(always)]
        pub const fn vbus_error(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "VBus drops below valid threshold"]
        #[inline(always)]
        pub const fn set_vbus_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Intrusb {
        #[inline(always)]
        fn default() -> Intrusb {
            Intrusb(0)
        }
    }
    impl core::fmt::Debug for Intrusb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intrusb")
                .field("suspend", &self.suspend())
                .field("resume", &self.resume())
                .field("reset", &self.reset())
                .field("sof", &self.sof())
                .field("conn", &self.conn())
                .field("discon", &self.discon())
                .field("sess_req", &self.sess_req())
                .field("vbus_error", &self.vbus_error())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Intrusb {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Intrusb {{ suspend: {=bool:?}, resume: {=bool:?}, reset: {=bool:?}, sof: {=bool:?}, conn: {=bool:?}, discon: {=bool:?}, sess_req: {=bool:?}, vbus_error: {=bool:?} }}" , self . suspend () , self . resume () , self . reset () , self . sof () , self . conn () , self . discon () , self . sess_req () , self . vbus_error ())
        }
    }
    #[doc = "USB Interrupt Enable Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intrusbe(pub u8);
    impl Intrusbe {
        #[doc = "Enable Suspend interrupt"]
        #[must_use]
        #[inline(always)]
        pub const fn suspend_enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Suspend interrupt"]
        #[inline(always)]
        pub const fn set_suspend_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Enable Resume interrupt"]
        #[must_use]
        #[inline(always)]
        pub const fn resume_enable(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Resume interrupt"]
        #[inline(always)]
        pub const fn set_resume_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "Enable Reset interrupt"]
        #[must_use]
        #[inline(always)]
        pub const fn reset_enable(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Reset interrupt"]
        #[inline(always)]
        pub const fn set_reset_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "Enable Start of Frame interrupt"]
        #[must_use]
        #[inline(always)]
        pub const fn sof_enable(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Start of Frame interrupt"]
        #[inline(always)]
        pub const fn set_sof_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Enable Connection interrupt"]
        #[must_use]
        #[inline(always)]
        pub const fn conn_enable(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Connection interrupt"]
        #[inline(always)]
        pub const fn set_conn_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Enable Disconnection interrupt"]
        #[must_use]
        #[inline(always)]
        pub const fn discon_enable(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Disconnection interrupt"]
        #[inline(always)]
        pub const fn set_discon_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Enable Session Request interrupt"]
        #[must_use]
        #[inline(always)]
        pub const fn sess_req_enable(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Session Request interrupt"]
        #[inline(always)]
        pub const fn set_sess_req_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Enable VBus Error interrupt"]
        #[must_use]
        #[inline(always)]
        pub const fn vbus_error_enable(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Enable VBus Error interrupt"]
        #[inline(always)]
        pub const fn set_vbus_error_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Intrusbe {
        #[inline(always)]
        fn default() -> Intrusbe {
            Intrusbe(0)
        }
    }
    impl core::fmt::Debug for Intrusbe {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intrusbe")
                .field("suspend_enable", &self.suspend_enable())
                .field("resume_enable", &self.resume_enable())
                .field("reset_enable", &self.reset_enable())
                .field("sof_enable", &self.sof_enable())
                .field("conn_enable", &self.conn_enable())
                .field("discon_enable", &self.discon_enable())
                .field("sess_req_enable", &self.sess_req_enable())
                .field("vbus_error_enable", &self.vbus_error_enable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Intrusbe {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Intrusbe {{ suspend_enable: {=bool:?}, resume_enable: {=bool:?}, reset_enable: {=bool:?}, sof_enable: {=bool:?}, conn_enable: {=bool:?}, discon_enable: {=bool:?}, sess_req_enable: {=bool:?}, vbus_error_enable: {=bool:?} }}" , self . suspend_enable () , self . resume_enable () , self . reset_enable () , self . sof_enable () , self . conn_enable () , self . discon_enable () , self . sess_req_enable () , self . vbus_error_enable ())
        }
    }
    #[doc = "Maximum payload size forendpoint"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maxp(pub u16);
    impl Maxp {
        #[doc = "Maximum payload"]
        #[must_use]
        #[inline(always)]
        pub const fn maxp(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Maximum payload"]
        #[inline(always)]
        pub const fn set_maxp(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
        }
    }
    impl Default for Maxp {
        #[inline(always)]
        fn default() -> Maxp {
            Maxp(0)
        }
    }
    impl core::fmt::Debug for Maxp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Maxp").field("maxp", &self.maxp()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Maxp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Maxp {{ maxp: {=u16:?} }}", self.maxp())
        }
    }
    #[doc = "USB Power Control and Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Power(pub u8);
    impl Power {
        #[doc = "Enable SUSPENDM output"]
        #[must_use]
        #[inline(always)]
        pub const fn enable_suspend_m(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable SUSPENDM output"]
        #[inline(always)]
        pub const fn set_enable_suspend_m(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "USB suspend mode control"]
        #[must_use]
        #[inline(always)]
        pub const fn suspend_mode(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "USB suspend mode control"]
        #[inline(always)]
        pub const fn set_suspend_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "Generate resume signaling"]
        #[must_use]
        #[inline(always)]
        pub const fn resume(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Generate resume signaling"]
        #[inline(always)]
        pub const fn set_resume(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "USB reset signaling status"]
        #[must_use]
        #[inline(always)]
        pub const fn reset(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "USB reset signaling status"]
        #[inline(always)]
        pub const fn set_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "High-speed mode negotiation status"]
        #[must_use]
        #[inline(always)]
        pub const fn hs_mode(&self) -> super::vals::HsModeStatus {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::HsModeStatus::from_bits(val as u8)
        }
        #[doc = "High-speed mode negotiation status"]
        #[inline(always)]
        pub const fn set_hs_mode(&mut self, val: super::vals::HsModeStatus) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
        }
        #[doc = "Enable High-speed mode negotiation"]
        #[must_use]
        #[inline(always)]
        pub const fn hs_enab(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Enable High-speed mode negotiation"]
        #[inline(always)]
        pub const fn set_hs_enab(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Enable/disable USB D+/D- lines"]
        #[must_use]
        #[inline(always)]
        pub const fn soft_conn(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Enable/disable USB D+/D- lines"]
        #[inline(always)]
        pub const fn set_soft_conn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Control isochronous packet transmission timing"]
        #[must_use]
        #[inline(always)]
        pub const fn iso_update(&self) -> super::vals::IsoUpdateMode {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::IsoUpdateMode::from_bits(val as u8)
        }
        #[doc = "Control isochronous packet transmission timing"]
        #[inline(always)]
        pub const fn set_iso_update(&mut self, val: super::vals::IsoUpdateMode) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Power {
        #[inline(always)]
        fn default() -> Power {
            Power(0)
        }
    }
    impl core::fmt::Debug for Power {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Power")
                .field("enable_suspend_m", &self.enable_suspend_m())
                .field("suspend_mode", &self.suspend_mode())
                .field("resume", &self.resume())
                .field("reset", &self.reset())
                .field("hs_mode", &self.hs_mode())
                .field("hs_enab", &self.hs_enab())
                .field("soft_conn", &self.soft_conn())
                .field("iso_update", &self.iso_update())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Power {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Power {{ enable_suspend_m: {=bool:?}, suspend_mode: {=bool:?}, resume: {=bool:?}, reset: {=bool:?}, hs_mode: {:?}, hs_enab: {=bool:?}, soft_conn: {=bool:?}, iso_update: {:?} }}" , self . enable_suspend_m () , self . suspend_mode () , self . resume () , self . reset () , self . hs_mode () , self . hs_enab () , self . soft_conn () , self . iso_update ())
        }
    }
    #[doc = "USB Endpoint 0 Received Data Byte Count"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxcount(pub u16);
    impl Rxcount {
        #[doc = "Number of received data bytes in FIFO"]
        #[must_use]
        #[inline(always)]
        pub const fn count(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "Number of received data bytes in FIFO"]
        #[inline(always)]
        pub const fn set_count(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u16) & 0x1fff) << 0usize);
        }
    }
    impl Default for Rxcount {
        #[inline(always)]
        fn default() -> Rxcount {
            Rxcount(0)
        }
    }
    impl core::fmt::Debug for Rxcount {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rxcount")
                .field("count", &self.count())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rxcount {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rxcount {{ count: {=u16:?} }}", self.count())
        }
    }
    #[doc = "RX Control and Status Register High"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxcsrh(pub u8);
    impl Rxcsrh {
        #[doc = "Incomplete packet in high-bandwidth Isochronous/Interrupt transfer"]
        #[must_use]
        #[inline(always)]
        pub const fn incomp_rx(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Incomplete packet in high-bandwidth Isochronous/Interrupt transfer"]
        #[inline(always)]
        pub const fn set_incomp_rx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Select DMA Request Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn dma_req_mode(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Select DMA Request Mode"]
        #[inline(always)]
        pub const fn set_dma_req_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Disable NYET handshakes or indicate PID error"]
        #[must_use]
        #[inline(always)]
        pub const fn dis_nyet_pid_error(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Disable NYET handshakes or indicate PID error"]
        #[inline(always)]
        pub const fn set_dis_nyet_pid_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Enable DMA request for RX endpoint"]
        #[must_use]
        #[inline(always)]
        pub const fn dma_req_enab(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Enable DMA request for RX endpoint"]
        #[inline(always)]
        pub const fn set_dma_req_enab(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "ISO mode enable"]
        #[must_use]
        #[inline(always)]
        pub const fn iso(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "ISO mode enable"]
        #[inline(always)]
        pub const fn set_iso(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Automatically clear RxPktRdy when max packet size is unloaded"]
        #[must_use]
        #[inline(always)]
        pub const fn auto_clear(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Automatically clear RxPktRdy when max packet size is unloaded"]
        #[inline(always)]
        pub const fn set_auto_clear(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Rxcsrh {
        #[inline(always)]
        fn default() -> Rxcsrh {
            Rxcsrh(0)
        }
    }
    impl core::fmt::Debug for Rxcsrh {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rxcsrh")
                .field("incomp_rx", &self.incomp_rx())
                .field("dma_req_mode", &self.dma_req_mode())
                .field("dis_nyet_pid_error", &self.dis_nyet_pid_error())
                .field("dma_req_enab", &self.dma_req_enab())
                .field("iso", &self.iso())
                .field("auto_clear", &self.auto_clear())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rxcsrh {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Rxcsrh {{ incomp_rx: {=bool:?}, dma_req_mode: {=bool:?}, dis_nyet_pid_error: {=bool:?}, dma_req_enab: {=bool:?}, iso: {=bool:?}, auto_clear: {=bool:?} }}" , self . incomp_rx () , self . dma_req_mode () , self . dis_nyet_pid_error () , self . dma_req_enab () , self . iso () , self . auto_clear ())
        }
    }
    #[doc = "RX Control and Status Register Low"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxcsrl(pub u8);
    impl Rxcsrl {
        #[doc = "Data packet received and ready to be unloaded"]
        #[must_use]
        #[inline(always)]
        pub const fn rx_pkt_rdy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Data packet received and ready to be unloaded"]
        #[inline(always)]
        pub const fn set_rx_pkt_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "No more packets can be loaded into Rx FIFO"]
        #[must_use]
        #[inline(always)]
        pub const fn fifo_full(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "No more packets can be loaded into Rx FIFO"]
        #[inline(always)]
        pub const fn set_fifo_full(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "OUT packet could not be loaded into Rx FIFO"]
        #[must_use]
        #[inline(always)]
        pub const fn over_run(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "OUT packet could not be loaded into Rx FIFO"]
        #[inline(always)]
        pub const fn set_over_run(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "CRC or bit-stuff error in data packet"]
        #[must_use]
        #[inline(always)]
        pub const fn data_error(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "CRC or bit-stuff error in data packet"]
        #[inline(always)]
        pub const fn set_data_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Flush next packet from Rx FIFO"]
        #[must_use]
        #[inline(always)]
        pub const fn flush_fifo(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Flush next packet from Rx FIFO"]
        #[inline(always)]
        pub const fn set_flush_fifo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Issue or terminate STALL handshake"]
        #[must_use]
        #[inline(always)]
        pub const fn send_stall(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Issue or terminate STALL handshake"]
        #[inline(always)]
        pub const fn set_send_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "STALL handshake transmission status"]
        #[must_use]
        #[inline(always)]
        pub const fn sent_stall(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "STALL handshake transmission status"]
        #[inline(always)]
        pub const fn set_sent_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Reset endpoint data toggle to 0"]
        #[must_use]
        #[inline(always)]
        pub const fn clr_data_tog(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Reset endpoint data toggle to 0"]
        #[inline(always)]
        pub const fn set_clr_data_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Rxcsrl {
        #[inline(always)]
        fn default() -> Rxcsrl {
            Rxcsrl(0)
        }
    }
    impl core::fmt::Debug for Rxcsrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rxcsrl")
                .field("rx_pkt_rdy", &self.rx_pkt_rdy())
                .field("fifo_full", &self.fifo_full())
                .field("over_run", &self.over_run())
                .field("data_error", &self.data_error())
                .field("flush_fifo", &self.flush_fifo())
                .field("send_stall", &self.send_stall())
                .field("sent_stall", &self.sent_stall())
                .field("clr_data_tog", &self.clr_data_tog())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rxcsrl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Rxcsrl {{ rx_pkt_rdy: {=bool:?}, fifo_full: {=bool:?}, over_run: {=bool:?}, data_error: {=bool:?}, flush_fifo: {=bool:?}, send_stall: {=bool:?}, sent_stall: {=bool:?}, clr_data_tog: {=bool:?} }}" , self . rx_pkt_rdy () , self . fifo_full () , self . over_run () , self . data_error () , self . flush_fifo () , self . send_stall () , self . sent_stall () , self . clr_data_tog ())
        }
    }
    #[doc = "USB test mode configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Testmode(pub u8);
    impl Testmode {
        #[doc = "Enter Test_SE0_NAK high-speed test mode"]
        #[must_use]
        #[inline(always)]
        pub const fn test_se0_nak(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enter Test_SE0_NAK high-speed test mode"]
        #[inline(always)]
        pub const fn set_test_se0_nak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Enter Test_J high-speed test mode"]
        #[must_use]
        #[inline(always)]
        pub const fn test_j(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Enter Test_J high-speed test mode"]
        #[inline(always)]
        pub const fn set_test_j(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "Enter Test_K high-speed test mode"]
        #[must_use]
        #[inline(always)]
        pub const fn test_k(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Enter Test_K high-speed test mode"]
        #[inline(always)]
        pub const fn set_test_k(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "Enter Test_Packet high-speed test mode"]
        #[must_use]
        #[inline(always)]
        pub const fn test_packet(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Enter Test_Packet high-speed test mode"]
        #[inline(always)]
        pub const fn set_test_packet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Force High-speed mode on USB reset"]
        #[must_use]
        #[inline(always)]
        pub const fn force_hs(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Force High-speed mode on USB reset"]
        #[inline(always)]
        pub const fn set_force_hs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Force Full-speed mode on USB reset"]
        #[must_use]
        #[inline(always)]
        pub const fn force_fs(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Force Full-speed mode on USB reset"]
        #[inline(always)]
        pub const fn set_force_fs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Transfer packet from Endpoint 0 TX FIFO to Endpoint 0 Rx FIFO"]
        #[must_use]
        #[inline(always)]
        pub const fn fifo_access(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer packet from Endpoint 0 TX FIFO to Endpoint 0 Rx FIFO"]
        #[inline(always)]
        pub const fn set_fifo_access(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Force core to enter Host mode"]
        #[must_use]
        #[inline(always)]
        pub const fn force_host(&self) -> super::vals::ForceHostMode {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::ForceHostMode::from_bits(val as u8)
        }
        #[doc = "Force core to enter Host mode"]
        #[inline(always)]
        pub const fn set_force_host(&mut self, val: super::vals::ForceHostMode) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Testmode {
        #[inline(always)]
        fn default() -> Testmode {
            Testmode(0)
        }
    }
    impl core::fmt::Debug for Testmode {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Testmode")
                .field("test_se0_nak", &self.test_se0_nak())
                .field("test_j", &self.test_j())
                .field("test_k", &self.test_k())
                .field("test_packet", &self.test_packet())
                .field("force_hs", &self.force_hs())
                .field("force_fs", &self.force_fs())
                .field("fifo_access", &self.fifo_access())
                .field("force_host", &self.force_host())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Testmode {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Testmode {{ test_se0_nak: {=bool:?}, test_j: {=bool:?}, test_k: {=bool:?}, test_packet: {=bool:?}, force_hs: {=bool:?}, force_fs: {=bool:?}, fifo_access: {=bool:?}, force_host: {:?} }}" , self . test_se0_nak () , self . test_j () , self . test_k () , self . test_packet () , self . force_hs () , self . force_fs () , self . fifo_access () , self . force_host ())
        }
    }
    #[doc = "Additional TX endpoint control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txcsrh(pub u8);
    impl Txcsrh {
        #[doc = "Select DMA Request Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn dma_req_mode(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Select DMA Request Mode"]
        #[inline(always)]
        pub const fn set_dma_req_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "Force endpoint data toggle switch"]
        #[must_use]
        #[inline(always)]
        pub const fn frc_data_tog(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Force endpoint data toggle switch"]
        #[inline(always)]
        pub const fn set_frc_data_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Enable DMA request for TX endpoint"]
        #[must_use]
        #[inline(always)]
        pub const fn dmareq_enab(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Enable DMA request for TX endpoint"]
        #[inline(always)]
        pub const fn set_dmareq_enab(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Endpoint direction control"]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> super::vals::EndpointDirection {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::EndpointDirection::from_bits(val as u8)
        }
        #[doc = "Endpoint direction control"]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: super::vals::EndpointDirection) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u8) & 0x01) << 5usize);
        }
        #[doc = "Enable Isochronous transfers"]
        #[must_use]
        #[inline(always)]
        pub const fn iso(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Isochronous transfers"]
        #[inline(always)]
        pub const fn set_iso(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Automatically set TxPktRdy for max packet size"]
        #[must_use]
        #[inline(always)]
        pub const fn auto_set(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Automatically set TxPktRdy for max packet size"]
        #[inline(always)]
        pub const fn set_auto_set(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Txcsrh {
        #[inline(always)]
        fn default() -> Txcsrh {
            Txcsrh(0)
        }
    }
    impl core::fmt::Debug for Txcsrh {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Txcsrh")
                .field("dma_req_mode", &self.dma_req_mode())
                .field("frc_data_tog", &self.frc_data_tog())
                .field("dmareq_enab", &self.dmareq_enab())
                .field("mode", &self.mode())
                .field("iso", &self.iso())
                .field("auto_set", &self.auto_set())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Txcsrh {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Txcsrh {{ dma_req_mode: {=bool:?}, frc_data_tog: {=bool:?}, dmareq_enab: {=bool:?}, mode: {:?}, iso: {=bool:?}, auto_set: {=bool:?} }}" , self . dma_req_mode () , self . frc_data_tog () , self . dmareq_enab () , self . mode () , self . iso () , self . auto_set ())
        }
    }
    #[doc = "TX endpoint control and status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txcsrl(pub u8);
    impl Txcsrl {
        #[doc = "TX packet ready for transmission"]
        #[must_use]
        #[inline(always)]
        pub const fn tx_pkt_rdy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TX packet ready for transmission"]
        #[inline(always)]
        pub const fn set_tx_pkt_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "TX FIFO contains at least one packet"]
        #[must_use]
        #[inline(always)]
        pub const fn fifo_not_empty(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TX FIFO contains at least one packet"]
        #[inline(always)]
        pub const fn set_fifo_not_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "IN token received without TxPktRdy"]
        #[must_use]
        #[inline(always)]
        pub const fn under_run(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "IN token received without TxPktRdy"]
        #[inline(always)]
        pub const fn set_under_run(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "Flush TX FIFO"]
        #[must_use]
        #[inline(always)]
        pub const fn flush_fifo(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Flush TX FIFO"]
        #[inline(always)]
        pub const fn set_flush_fifo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Issue STALL handshake to IN token"]
        #[must_use]
        #[inline(always)]
        pub const fn send_stall(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Issue STALL handshake to IN token"]
        #[inline(always)]
        pub const fn set_send_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "STALL handshake transmission status"]
        #[must_use]
        #[inline(always)]
        pub const fn sent_stall(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "STALL handshake transmission status"]
        #[inline(always)]
        pub const fn set_sent_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Reset endpoint data toggle"]
        #[must_use]
        #[inline(always)]
        pub const fn clr_data_tog(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Reset endpoint data toggle"]
        #[inline(always)]
        pub const fn set_clr_data_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Incomplete high-bandwidth Isochronous transfer"]
        #[must_use]
        #[inline(always)]
        pub const fn incomp_tx(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Incomplete high-bandwidth Isochronous transfer"]
        #[inline(always)]
        pub const fn set_incomp_tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Txcsrl {
        #[inline(always)]
        fn default() -> Txcsrl {
            Txcsrl(0)
        }
    }
    impl core::fmt::Debug for Txcsrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Txcsrl")
                .field("tx_pkt_rdy", &self.tx_pkt_rdy())
                .field("fifo_not_empty", &self.fifo_not_empty())
                .field("under_run", &self.under_run())
                .field("flush_fifo", &self.flush_fifo())
                .field("send_stall", &self.send_stall())
                .field("sent_stall", &self.sent_stall())
                .field("clr_data_tog", &self.clr_data_tog())
                .field("incomp_tx", &self.incomp_tx())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Txcsrl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Txcsrl {{ tx_pkt_rdy: {=bool:?}, fifo_not_empty: {=bool:?}, under_run: {=bool:?}, flush_fifo: {=bool:?}, send_stall: {=bool:?}, sent_stall: {=bool:?}, clr_data_tog: {=bool:?}, incomp_tx: {=bool:?} }}" , self . tx_pkt_rdy () , self . fifo_not_empty () , self . under_run () , self . flush_fifo () , self . send_stall () , self . sent_stall () , self . clr_data_tog () , self . incomp_tx ())
        }
    }
    #[doc = "Vender-specified USB configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Usbcfg(pub u8);
    impl Usbcfg {
        #[must_use]
        #[inline(always)]
        pub const fn avalid_dr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_avalid_dr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn avalid(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_avalid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
    }
    impl Default for Usbcfg {
        #[inline(always)]
        fn default() -> Usbcfg {
            Usbcfg(0)
        }
    }
    impl core::fmt::Debug for Usbcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Usbcfg")
                .field("avalid_dr", &self.avalid_dr())
                .field("avalid", &self.avalid())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Usbcfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Usbcfg {{ avalid_dr: {=bool:?}, avalid: {=bool:?} }}",
                self.avalid_dr(),
                self.avalid()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum DeviceType {
        ADevice = 0x0,
        BDevice = 0x01,
    }
    impl DeviceType {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> DeviceType {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for DeviceType {
        #[inline(always)]
        fn from(val: u8) -> DeviceType {
            DeviceType::from_bits(val)
        }
    }
    impl From<DeviceType> for u8 {
        #[inline(always)]
        fn from(val: DeviceType) -> u8 {
            DeviceType::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum EndpointDirection {
        Rx = 0x0,
        Tx = 0x01,
    }
    impl EndpointDirection {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> EndpointDirection {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for EndpointDirection {
        #[inline(always)]
        fn from(val: u8) -> EndpointDirection {
            EndpointDirection::from_bits(val)
        }
    }
    impl From<EndpointDirection> for u8 {
        #[inline(always)]
        fn from(val: EndpointDirection) -> u8 {
            EndpointDirection::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum ForceHostMode {
        Normal = 0x0,
        Force = 0x01,
    }
    impl ForceHostMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ForceHostMode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ForceHostMode {
        #[inline(always)]
        fn from(val: u8) -> ForceHostMode {
            ForceHostMode::from_bits(val)
        }
    }
    impl From<ForceHostMode> for u8 {
        #[inline(always)]
        fn from(val: ForceHostMode) -> u8 {
            ForceHostMode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum HsModeStatus {
        FullSpeed = 0x0,
        HighSpeed = 0x01,
    }
    impl HsModeStatus {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> HsModeStatus {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for HsModeStatus {
        #[inline(always)]
        fn from(val: u8) -> HsModeStatus {
            HsModeStatus::from_bits(val)
        }
    }
    impl From<HsModeStatus> for u8 {
        #[inline(always)]
        fn from(val: HsModeStatus) -> u8 {
            HsModeStatus::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum IsoUpdateMode {
        Normal = 0x0,
        WaitSof = 0x01,
    }
    impl IsoUpdateMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> IsoUpdateMode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for IsoUpdateMode {
        #[inline(always)]
        fn from(val: u8) -> IsoUpdateMode {
            IsoUpdateMode::from_bits(val)
        }
    }
    impl From<IsoUpdateMode> for u8 {
        #[inline(always)]
        fn from(val: IsoUpdateMode) -> u8 {
            IsoUpdateMode::to_bits(val)
        }
    }
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
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum VbusLevel {
        BelowSessionEnd = 0x0,
        AboveSessionEndBelowAvalid = 0x01,
        AboveAvalidBelowVbusValid = 0x02,
        AboveVbusValid = 0x03,
    }
    impl VbusLevel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> VbusLevel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for VbusLevel {
        #[inline(always)]
        fn from(val: u8) -> VbusLevel {
            VbusLevel::from_bits(val)
        }
    }
    impl From<VbusLevel> for u8 {
        #[inline(always)]
        fn from(val: VbusLevel) -> u8 {
            VbusLevel::to_bits(val)
        }
    }
}
