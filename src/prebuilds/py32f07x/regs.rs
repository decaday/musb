
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "USB power management register."]
    #[inline(always)]
    pub const fn power(self) -> crate::common::Reg<regs::Power, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01usize) as _) }
    }
    #[doc = "USB interrupt status register."]
    #[inline(always)]
    pub const fn intrusb(self) -> crate::common::Reg<regs::Intrusb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Interrupt status for OUT endpoint."]
    #[inline(always)]
    pub const fn intrrx(self) -> crate::common::Reg<regs::Intrrx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05usize) as _) }
    }
    #[doc = "Interrupt status for IN endpoint."]
    #[inline(always)]
    pub const fn intrtx(self) -> crate::common::Reg<regs::Intrtx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06usize) as _) }
    }
    #[doc = "USB interrupt enable register."]
    #[inline(always)]
    pub const fn intrusbe(self) -> crate::common::Reg<regs::Intrusbe, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Interrupt enable for OUT endpoint 1."]
    #[inline(always)]
    pub const fn intrrxe(self) -> crate::common::Reg<regs::Intrrxe, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x09usize) as _) }
    }
    #[doc = "Interrupt enable for IN endpoint 1."]
    #[inline(always)]
    pub const fn intrtxe(self) -> crate::common::Reg<regs::Intrtxe, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ausize) as _) }
    }
    #[doc = "USB frame number and endpoint index."]
    #[inline(always)]
    pub const fn frame(self) -> crate::common::Reg<regs::Frame, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Selected endpoint index."]
    #[inline(always)]
    pub const fn index(self) -> crate::common::Reg<regs::Index, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0eusize) as _) }
    }
    #[doc = "Endpoint 0 control and status register."]
    #[inline(always)]
    pub const fn csr0l(self) -> crate::common::Reg<regs::Csr0l, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Data count for endpoint 0."]
    #[inline(always)]
    pub const fn count0(self) -> crate::common::Reg<regs::Count0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x11usize) as _) }
    }
    #[doc = "Control and status register for IN endpoints."]
    #[inline(always)]
    pub const fn txcsrh(self) -> crate::common::Reg<regs::Txcsrh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Control and status register for IN endpoints."]
    #[inline(always)]
    pub const fn txcsrl(self) -> crate::common::Reg<regs::Txcsrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x15usize) as _) }
    }
    #[doc = "Maximum packet size for IN endpoints."]
    #[inline(always)]
    pub const fn txmaxp(self) -> crate::common::Reg<regs::Txmaxp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x16usize) as _) }
    }
    #[doc = "Control and status register for OUT endpoints."]
    #[inline(always)]
    pub const fn rxcsrh(self) -> crate::common::Reg<regs::Rxcsrh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Control and status register for OUT endpoints."]
    #[inline(always)]
    pub const fn rxcsrl(self) -> crate::common::Reg<regs::Rxcsrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x19usize) as _) }
    }
    #[doc = "Maximum packet size for OUT endpoints."]
    #[inline(always)]
    pub const fn rxmaxp(self) -> crate::common::Reg<regs::Rxmaxp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1ausize) as _) }
    }
    #[doc = "Data count for OUT endpoints."]
    #[inline(always)]
    pub const fn rxcount(self) -> crate::common::Reg<regs::Rxcount, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "FIFO for endpoints."]
    #[inline(always)]
    pub const fn fifo(self, n: usize) -> crate::common::Reg<regs::Fifo, crate::common::RW> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "USB Endpoint 0 Received Data Byte Count"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Count0(pub u8);
    impl Count0 {
        #[doc = "Number of received data bytes in FIFO"]
        #[inline(always)]
        pub const fn count(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Number of received data bytes in FIFO"]
        #[inline(always)]
        pub fn set_count(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u8) & 0x7f) << 0usize);
        }
    }
    impl Default for Count0 {
        #[inline(always)]
        fn default() -> Count0 {
            Count0(0)
        }
    }
    #[doc = "USB Endpoint 0 Control and Status Register Low"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr0l(pub u8);
    impl Csr0l {
        #[doc = "Indicates received data packet ready for processing"]
        #[inline(always)]
        pub const fn rx_pkt_rdy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates received data packet ready for processing"]
        #[inline(always)]
        pub fn set_rx_pkt_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Indicates data packet loaded in FIFO ready for transmission"]
        #[inline(always)]
        pub const fn tx_pkt_rdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates data packet loaded in FIFO ready for transmission"]
        #[inline(always)]
        pub fn set_tx_pkt_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "Set when STALL handshake is transmitted"]
        #[inline(always)]
        pub const fn sent_stall(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Set when STALL handshake is transmitted"]
        #[inline(always)]
        pub fn set_sent_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "Marks the end of data transfer"]
        #[inline(always)]
        pub const fn data_end(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Marks the end of data transfer"]
        #[inline(always)]
        pub fn set_data_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Control transaction ended prematurely"]
        #[inline(always)]
        pub const fn setup_end(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Control transaction ended prematurely"]
        #[inline(always)]
        pub fn set_setup_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Terminate current transaction with STALL handshake"]
        #[inline(always)]
        pub const fn send_stall(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Terminate current transaction with STALL handshake"]
        #[inline(always)]
        pub fn set_send_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Clear RxPktRdy bit"]
        #[inline(always)]
        pub const fn serviced_rx_pkt_rdy(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Clear RxPktRdy bit"]
        #[inline(always)]
        pub fn set_serviced_rx_pkt_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Clear SetupEnd bit"]
        #[inline(always)]
        pub const fn serviced_setup_end(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Clear SetupEnd bit"]
        #[inline(always)]
        pub fn set_serviced_setup_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Csr0l {
        #[inline(always)]
        fn default() -> Csr0l {
            Csr0l(0)
        }
    }
    #[doc = "Function Address Register for USB device addressing"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Faddr(pub u8);
    impl Faddr {
        #[doc = "USB device function address"]
        #[inline(always)]
        pub const fn func_addr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "USB device function address"]
        #[inline(always)]
        pub fn set_func_addr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u8) & 0x7f) << 0usize);
        }
    }
    impl Default for Faddr {
        #[inline(always)]
        fn default() -> Faddr {
            Faddr(0)
        }
    }
    #[doc = "FIFO Data Access Register for Endpoints"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fifo(pub u8);
    impl Fifo {
        #[doc = "Data byte for FIFO read/write operation"]
        #[inline(always)]
        pub const fn data(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Data byte for FIFO read/write operation"]
        #[inline(always)]
        pub fn set_data(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Fifo {
        #[inline(always)]
        fn default() -> Fifo {
            Fifo(0)
        }
    }
    #[doc = "Last received USB frame number"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Frame(pub u16);
    impl Frame {
        #[doc = "USB frame number"]
        #[inline(always)]
        pub const fn frame(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "USB frame number"]
        #[inline(always)]
        pub fn set_frame(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
        }
    }
    impl Default for Frame {
        #[inline(always)]
        fn default() -> Frame {
            Frame(0)
        }
    }
    #[doc = "Endpoint index selection register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Index(pub u8);
    impl Index {
        #[doc = "Selects which endpoint control/status registers are accessed"]
        #[inline(always)]
        pub const fn index(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Selects which endpoint control/status registers are accessed"]
        #[inline(always)]
        pub fn set_index(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
    }
    impl Default for Index {
        #[inline(always)]
        fn default() -> Index {
            Index(0)
        }
    }
    #[doc = "Receive Endpoint Interrupt Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intrrx(pub u8);
    impl Intrrx {
        #[doc = "Endpoint transmit interrupt (except EP0)"]
        #[inline(always)]
        pub const fn ep_rx(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Endpoint transmit interrupt (except EP0)"]
        #[inline(always)]
        pub fn set_ep_rx(&mut self, n: usize, val: bool) {
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
    #[doc = "Receive Endpoint Interrupt Enable Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intrrxe(pub u8);
    impl Intrrxe {
        #[doc = "Endpoint transmit interrupt enable (except EP0)"]
        #[inline(always)]
        pub const fn ep_rxe(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Endpoint transmit interrupt enable (except EP0)"]
        #[inline(always)]
        pub fn set_ep_rxe(&mut self, n: usize, val: bool) {
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
    #[doc = "Transmit Endpoint Interrupt Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intrtx(pub u8);
    impl Intrtx {
        #[doc = "Endpoint transmit interrupt (except EP0)"]
        #[inline(always)]
        pub const fn ep_tx(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Endpoint transmit interrupt (except EP0)"]
        #[inline(always)]
        pub fn set_ep_tx(&mut self, n: usize, val: bool) {
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
    #[doc = "Transmit Endpoint Interrupt Enable Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intrtxe(pub u8);
    impl Intrtxe {
        #[doc = "Endpoint transmit interrupt enable (EP0:TXE_RXE)"]
        #[inline(always)]
        pub const fn ep_txe(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Endpoint transmit interrupt enable (EP0:TXE_RXE)"]
        #[inline(always)]
        pub fn set_ep_txe(&mut self, n: usize, val: bool) {
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
    #[doc = "USB Interrupt Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intrusb(pub u8);
    impl Intrusb {
        #[doc = "Suspend signaling detected"]
        #[inline(always)]
        pub const fn suspend(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Suspend signaling detected"]
        #[inline(always)]
        pub fn set_suspend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Resume signaling detected during Suspend"]
        #[inline(always)]
        pub const fn resume(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Resume signaling detected during Suspend"]
        #[inline(always)]
        pub fn set_resume(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "Reset signaling detected"]
        #[inline(always)]
        pub const fn reset(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Reset signaling detected"]
        #[inline(always)]
        pub fn set_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "New frame start"]
        #[inline(always)]
        pub const fn sof(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "New frame start"]
        #[inline(always)]
        pub fn set_sof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
    }
    impl Default for Intrusb {
        #[inline(always)]
        fn default() -> Intrusb {
            Intrusb(0)
        }
    }
    #[doc = "USB Interrupt Enable Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intrusbe(pub u8);
    impl Intrusbe {
        #[doc = "Enable Suspend interrupt"]
        #[inline(always)]
        pub const fn suspend_enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Suspend interrupt"]
        #[inline(always)]
        pub fn set_suspend_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Enable Resume interrupt"]
        #[inline(always)]
        pub const fn resume_enable(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Resume interrupt"]
        #[inline(always)]
        pub fn set_resume_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "Enable Reset interrupt"]
        #[inline(always)]
        pub const fn reset_enable(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Reset interrupt"]
        #[inline(always)]
        pub fn set_reset_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "Enable Start of Frame interrupt"]
        #[inline(always)]
        pub const fn sof_enable(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Start of Frame interrupt"]
        #[inline(always)]
        pub fn set_sof_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
    }
    impl Default for Intrusbe {
        #[inline(always)]
        fn default() -> Intrusbe {
            Intrusbe(0)
        }
    }
    #[doc = "USB Power Control and Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Power(pub u8);
    impl Power {
        #[doc = "Enable SUSPENDM output"]
        #[inline(always)]
        pub const fn enable_suspend_m(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable SUSPENDM output"]
        #[inline(always)]
        pub fn set_enable_suspend_m(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "USB suspend mode control"]
        #[inline(always)]
        pub const fn suspend_mode(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "USB suspend mode control"]
        #[inline(always)]
        pub fn set_suspend_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "Generate resume signaling"]
        #[inline(always)]
        pub const fn resume(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Generate resume signaling"]
        #[inline(always)]
        pub fn set_resume(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "USB reset signaling status"]
        #[inline(always)]
        pub const fn reset(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "USB reset signaling status"]
        #[inline(always)]
        pub fn set_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Control isochronous packet transmission timing"]
        #[inline(always)]
        pub const fn iso_update(&self) -> super::vals::IsoUpdateMode {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::IsoUpdateMode::from_bits(val as u8)
        }
        #[doc = "Control isochronous packet transmission timing"]
        #[inline(always)]
        pub fn set_iso_update(&mut self, val: super::vals::IsoUpdateMode) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Power {
        #[inline(always)]
        fn default() -> Power {
            Power(0)
        }
    }
    #[doc = "USB Endpoint 0 Received Data Byte Count"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxcount(pub u16);
    impl Rxcount {
        #[doc = "Number of received data bytes in FIFO"]
        #[inline(always)]
        pub const fn count(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "Number of received data bytes in FIFO"]
        #[inline(always)]
        pub fn set_count(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u16) & 0x1fff) << 0usize);
        }
    }
    impl Default for Rxcount {
        #[inline(always)]
        fn default() -> Rxcount {
            Rxcount(0)
        }
    }
    #[doc = "RX Control and Status Register High"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxcsrh(pub u8);
    impl Rxcsrh {
        #[doc = "Select DMA Request Mode"]
        #[inline(always)]
        pub const fn dma_req_mode(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Select DMA Request Mode"]
        #[inline(always)]
        pub fn set_dma_req_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Enable DMA request for RX endpoint"]
        #[inline(always)]
        pub const fn dma_req_enab(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Enable DMA request for RX endpoint"]
        #[inline(always)]
        pub fn set_dma_req_enab(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "ISO mode enable"]
        #[inline(always)]
        pub const fn iso(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "ISO mode enable"]
        #[inline(always)]
        pub fn set_iso(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Automatically clear RxPktRdy when max packet size is unloaded"]
        #[inline(always)]
        pub const fn auto_clear(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Automatically clear RxPktRdy when max packet size is unloaded"]
        #[inline(always)]
        pub fn set_auto_clear(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Rxcsrh {
        #[inline(always)]
        fn default() -> Rxcsrh {
            Rxcsrh(0)
        }
    }
    #[doc = "RX Control and Status Register Low"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxcsrl(pub u8);
    impl Rxcsrl {
        #[doc = "Data packet received and ready to be unloaded"]
        #[inline(always)]
        pub const fn rx_pkt_rdy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Data packet received and ready to be unloaded"]
        #[inline(always)]
        pub fn set_rx_pkt_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "No more packets can be loaded into Rx FIFO"]
        #[inline(always)]
        pub const fn fifo_full(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "No more packets can be loaded into Rx FIFO"]
        #[inline(always)]
        pub fn set_fifo_full(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "OUT packet could not be loaded into Rx FIFO"]
        #[inline(always)]
        pub const fn over_run(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "OUT packet could not be loaded into Rx FIFO"]
        #[inline(always)]
        pub fn set_over_run(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "CRC or bit-stuff error in data packet"]
        #[inline(always)]
        pub const fn data_error(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "CRC or bit-stuff error in data packet"]
        #[inline(always)]
        pub fn set_data_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Flush next packet from Rx FIFO"]
        #[inline(always)]
        pub const fn flush_fifo(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Flush next packet from Rx FIFO"]
        #[inline(always)]
        pub fn set_flush_fifo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Issue or terminate STALL handshake"]
        #[inline(always)]
        pub const fn send_stall(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Issue or terminate STALL handshake"]
        #[inline(always)]
        pub fn set_send_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "STALL handshake transmission status"]
        #[inline(always)]
        pub const fn sent_stall(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "STALL handshake transmission status"]
        #[inline(always)]
        pub fn set_sent_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Reset endpoint data toggle to 0"]
        #[inline(always)]
        pub const fn clr_data_tog(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Reset endpoint data toggle to 0"]
        #[inline(always)]
        pub fn set_clr_data_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Rxcsrl {
        #[inline(always)]
        fn default() -> Rxcsrl {
            Rxcsrl(0)
        }
    }
    #[doc = "Maximum payload size for RX endpoint"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxmaxp(pub u16);
    impl Rxmaxp {
        #[doc = "Maximum payload in 8bytes"]
        #[inline(always)]
        pub const fn maxp(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Maximum payload in 8bytes"]
        #[inline(always)]
        pub fn set_maxp(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
        }
    }
    impl Default for Rxmaxp {
        #[inline(always)]
        fn default() -> Rxmaxp {
            Rxmaxp(0)
        }
    }
    #[doc = "Additional TX endpoint control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txcsrh(pub u8);
    impl Txcsrh {
        #[doc = "Select DMA Request Mode"]
        #[inline(always)]
        pub const fn dma_req_mode(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Select DMA Request Mode"]
        #[inline(always)]
        pub fn set_dma_req_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "Force endpoint data toggle switch"]
        #[inline(always)]
        pub const fn frc_data_tog(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Force endpoint data toggle switch"]
        #[inline(always)]
        pub fn set_frc_data_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Enable DMA request for TX endpoint"]
        #[inline(always)]
        pub const fn dmareq_enab(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Enable DMA request for TX endpoint"]
        #[inline(always)]
        pub fn set_dmareq_enab(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Endpoint direction control"]
        #[inline(always)]
        pub const fn mode(&self) -> super::vals::EndpointDirection {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::EndpointDirection::from_bits(val as u8)
        }
        #[doc = "Endpoint direction control"]
        #[inline(always)]
        pub fn set_mode(&mut self, val: super::vals::EndpointDirection) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u8) & 0x01) << 5usize);
        }
        #[doc = "Enable Isochronous transfers"]
        #[inline(always)]
        pub const fn iso(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Isochronous transfers"]
        #[inline(always)]
        pub fn set_iso(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Automatically set TxPktRdy for max packet size"]
        #[inline(always)]
        pub const fn auto_set(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Automatically set TxPktRdy for max packet size"]
        #[inline(always)]
        pub fn set_auto_set(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Txcsrh {
        #[inline(always)]
        fn default() -> Txcsrh {
            Txcsrh(0)
        }
    }
    #[doc = "TX endpoint control and status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txcsrl(pub u8);
    impl Txcsrl {
        #[doc = "TX packet ready for transmission"]
        #[inline(always)]
        pub const fn tx_pkt_rdy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TX packet ready for transmission"]
        #[inline(always)]
        pub fn set_tx_pkt_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "TX FIFO contains at least one packet"]
        #[inline(always)]
        pub const fn fifo_not_empty(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TX FIFO contains at least one packet"]
        #[inline(always)]
        pub fn set_fifo_not_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "IN token received without TxPktRdy"]
        #[inline(always)]
        pub const fn under_run(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "IN token received without TxPktRdy"]
        #[inline(always)]
        pub fn set_under_run(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "Flush TX FIFO"]
        #[inline(always)]
        pub const fn flush_fifo(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Flush TX FIFO"]
        #[inline(always)]
        pub fn set_flush_fifo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Issue STALL handshake to IN token"]
        #[inline(always)]
        pub const fn send_stall(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Issue STALL handshake to IN token"]
        #[inline(always)]
        pub fn set_send_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "STALL handshake transmission status"]
        #[inline(always)]
        pub const fn sent_stall(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "STALL handshake transmission status"]
        #[inline(always)]
        pub fn set_sent_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Reset endpoint data toggle"]
        #[inline(always)]
        pub const fn clr_data_tog(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Reset endpoint data toggle"]
        #[inline(always)]
        pub fn set_clr_data_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
    }
    impl Default for Txcsrl {
        #[inline(always)]
        fn default() -> Txcsrl {
            Txcsrl(0)
        }
    }
    #[doc = "Maximum payload size for TX endpoint"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txmaxp(pub u16);
    impl Txmaxp {
        #[doc = "Maximum payload in 8bytes"]
        #[inline(always)]
        pub const fn maxp(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Maximum payload in 8bytes"]
        #[inline(always)]
        pub fn set_maxp(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
        }
    }
    impl Default for Txmaxp {
        #[inline(always)]
        fn default() -> Txmaxp {
            Txmaxp(0)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum EndpointDirection {
        RX = 0x0,
        TX = 0x01,
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum HsModeStatus {
        FULL_SPEED = 0x0,
        HIGH_SPEED = 0x01,
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum IsoUpdateMode {
        NORMAL = 0x0,
        WAIT_SOF = 0x01,
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
