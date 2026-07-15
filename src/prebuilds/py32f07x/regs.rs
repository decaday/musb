
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
    #[doc = "Function address of the USB device."]
    #[inline(always)]
    pub const fn faddr(self) -> crate::common::Reg<regs::Faddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "USB power management register."]
    #[inline(always)]
    pub const fn power(self) -> crate::common::Reg<regs::Power, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01usize) as _) }
    }
    #[doc = "USB interrupt status register."]
    #[inline(always)]
    pub const fn intrusb(self) -> crate::common::Reg<regs::Intrusb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Interrupt status for OUT endpoint."]
    #[inline(always)]
    pub const fn intrrx(self) -> crate::common::Reg<regs::Intrrx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05usize) as _) }
    }
    #[doc = "Interrupt status for IN endpoint."]
    #[inline(always)]
    pub const fn intrtx(self) -> crate::common::Reg<regs::Intrtx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "USB interrupt enable register."]
    #[inline(always)]
    pub const fn intrusbe(self) -> crate::common::Reg<regs::Intrusbe, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Interrupt enable for OUT endpoint 1."]
    #[inline(always)]
    pub const fn intrrxe(self) -> crate::common::Reg<regs::Intrrxe, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09usize) as _) }
    }
    #[doc = "Interrupt enable for IN endpoint 1."]
    #[inline(always)]
    pub const fn intrtxe(self) -> crate::common::Reg<regs::Intrtxe, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "USB frame number and endpoint index."]
    #[inline(always)]
    pub const fn frame(self) -> crate::common::Reg<regs::Frame, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Selected endpoint index."]
    #[inline(always)]
    pub const fn index(self) -> crate::common::Reg<regs::Index, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0eusize) as _) }
    }
    #[doc = "Endpoint 0 control and status register."]
    #[inline(always)]
    pub const fn csr0l(self) -> crate::common::Reg<regs::Csr0l, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Data count for endpoint 0."]
    #[inline(always)]
    pub const fn count0(self) -> crate::common::Reg<regs::Count0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11usize) as _) }
    }
    #[doc = "Control and status register for IN endpoints."]
    #[inline(always)]
    pub const fn txcsrh(self) -> crate::common::Reg<regs::Txcsrh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Control and status register for IN endpoints."]
    #[inline(always)]
    pub const fn txcsrl(self) -> crate::common::Reg<regs::Txcsrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x15usize) as _) }
    }
    #[doc = "Maximum packet size for IN endpoints."]
    #[inline(always)]
    pub const fn txmaxp(self) -> crate::common::Reg<regs::Maxp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x16usize) as _) }
    }
    #[doc = "Control and status register for OUT endpoints."]
    #[inline(always)]
    pub const fn rxcsrh(self) -> crate::common::Reg<regs::Rxcsrh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Control and status register for OUT endpoints."]
    #[inline(always)]
    pub const fn rxcsrl(self) -> crate::common::Reg<regs::Rxcsrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x19usize) as _) }
    }
    #[doc = "Maximum packet size for OUT endpoints."]
    #[inline(always)]
    pub const fn rxmaxp(self) -> crate::common::Reg<regs::Maxp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1ausize) as _) }
    }
    #[doc = "Data count for OUT endpoints."]
    #[inline(always)]
    pub const fn rxcount(self) -> crate::common::Reg<regs::Rxcount, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "FIFO for endpoints."]
    #[inline(always)]
    pub const fn fifo(self, n: usize) -> crate::common::Reg<regs::Fifo, crate::common::RW> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize + n * 4usize) as _) }
    }
}
pub mod regs {
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
    pub struct Intrrx(pub u8);
    impl Intrrx {
        #[doc = "Receive endpoint interrupt (except EP0)"]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rx(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Receive endpoint interrupt (except EP0)"]
        #[inline(always)]
        pub const fn set_ep_rx(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u8) & 0x01) << offs);
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
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Intrrx {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Intrrx {{ ep_rx[0]: {=bool:?}, ep_rx[1]: {=bool:?}, ep_rx[2]: {=bool:?}, ep_rx[3]: {=bool:?}, ep_rx[4]: {=bool:?}, ep_rx[5]: {=bool:?} }}" , self . ep_rx (0usize) , self . ep_rx (1usize) , self . ep_rx (2usize) , self . ep_rx (3usize) , self . ep_rx (4usize) , self . ep_rx (5usize))
        }
    }
    #[doc = "Receive Endpoint Interrupt Enable Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intrrxe(pub u8);
    impl Intrrxe {
        #[doc = "Endpoint transmit interrupt enable (except EP0)"]
        #[must_use]
        #[inline(always)]
        pub const fn ep_rxe(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Endpoint transmit interrupt enable (except EP0)"]
        #[inline(always)]
        pub const fn set_ep_rxe(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u8) & 0x01) << offs);
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
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Intrrxe {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Intrrxe {{ ep_rxe[0]: {=bool:?}, ep_rxe[1]: {=bool:?}, ep_rxe[2]: {=bool:?}, ep_rxe[3]: {=bool:?}, ep_rxe[4]: {=bool:?}, ep_rxe[5]: {=bool:?} }}" , self . ep_rxe (0usize) , self . ep_rxe (1usize) , self . ep_rxe (2usize) , self . ep_rxe (3usize) , self . ep_rxe (4usize) , self . ep_rxe (5usize))
        }
    }
    #[doc = "Transmit Endpoint Interrupt Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intrtx(pub u8);
    impl Intrtx {
        #[doc = "Endpoint 0 and transmit endpoints interrupt"]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tx(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Endpoint 0 and transmit endpoints interrupt"]
        #[inline(always)]
        pub const fn set_ep_tx(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u8) & 0x01) << offs);
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
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Intrtx {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Intrtx {{ ep_tx[0]: {=bool:?}, ep_tx[1]: {=bool:?}, ep_tx[2]: {=bool:?}, ep_tx[3]: {=bool:?}, ep_tx[4]: {=bool:?}, ep_tx[5]: {=bool:?} }}" , self . ep_tx (0usize) , self . ep_tx (1usize) , self . ep_tx (2usize) , self . ep_tx (3usize) , self . ep_tx (4usize) , self . ep_tx (5usize))
        }
    }
    #[doc = "Transmit Endpoint Interrupt Enable Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intrtxe(pub u8);
    impl Intrtxe {
        #[doc = "Endpoint transmit interrupt enable (EP0:TXE_RXE)"]
        #[must_use]
        #[inline(always)]
        pub const fn ep_txe(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Endpoint transmit interrupt enable (EP0:TXE_RXE)"]
        #[inline(always)]
        pub const fn set_ep_txe(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u8) & 0x01) << offs);
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
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Intrtxe {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Intrtxe {{ ep_txe[0]: {=bool:?}, ep_txe[1]: {=bool:?}, ep_txe[2]: {=bool:?}, ep_txe[3]: {=bool:?}, ep_txe[4]: {=bool:?}, ep_txe[5]: {=bool:?} }}" , self . ep_txe (0usize) , self . ep_txe (1usize) , self . ep_txe (2usize) , self . ep_txe (3usize) , self . ep_txe (4usize) , self . ep_txe (5usize))
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
        #[doc = "Reset signaling detected"]
        #[must_use]
        #[inline(always)]
        pub const fn reset(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Reset signaling detected"]
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
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Intrusb {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Intrusb {{ suspend: {=bool:?}, resume: {=bool:?}, reset: {=bool:?}, sof: {=bool:?} }}" , self . suspend () , self . resume () , self . reset () , self . sof ())
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
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Intrusbe {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Intrusbe {{ suspend_enable: {=bool:?}, resume_enable: {=bool:?}, reset_enable: {=bool:?}, sof_enable: {=bool:?} }}" , self . suspend_enable () , self . resume_enable () , self . reset_enable () , self . sof_enable ())
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
                .field("iso_update", &self.iso_update())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Power {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Power {{ enable_suspend_m: {=bool:?}, suspend_mode: {=bool:?}, resume: {=bool:?}, reset: {=bool:?}, iso_update: {:?} }}" , self . enable_suspend_m () , self . suspend_mode () , self . resume () , self . reset () , self . iso_update ())
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
        #[doc = "Select DMA Request Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn dma_req_mode(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Select DMA Request Mode"]
        #[inline(always)]
        pub const fn set_dma_req_mode(&mut self, val: bool) {
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
                .field("dma_req_mode", &self.dma_req_mode())
                .field("dma_req_enab", &self.dma_req_enab())
                .field("iso", &self.iso())
                .field("auto_clear", &self.auto_clear())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rxcsrh {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Rxcsrh {{ dma_req_mode: {=bool:?}, dma_req_enab: {=bool:?}, iso: {=bool:?}, auto_clear: {=bool:?} }}" , self . dma_req_mode () , self . dma_req_enab () , self . iso () , self . auto_clear ())
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
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Txcsrl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Txcsrl {{ tx_pkt_rdy: {=bool:?}, fifo_not_empty: {=bool:?}, under_run: {=bool:?}, flush_fifo: {=bool:?}, send_stall: {=bool:?}, sent_stall: {=bool:?}, clr_data_tog: {=bool:?} }}" , self . tx_pkt_rdy () , self . fifo_not_empty () , self . under_run () , self . flush_fifo () , self . send_stall () , self . sent_stall () , self . clr_data_tog ())
        }
    }
}
pub mod vals {
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
}
