
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Power management register."]
    #[inline(always)]
    pub const fn power(self) -> crate::common::Reg<regs::Power, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01usize) as _) }
    }
    #[doc = "Interrupt register for Endpoint 0 plus TX Endpoints 1 to 15."]
    #[inline(always)]
    pub const fn intrtx(self) -> crate::common::Reg<regs::Intrtx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02usize) as _) }
    }
    #[doc = "Interrupt register for Rx Endpoints 1 to 15."]
    #[inline(always)]
    pub const fn intrrx(self) -> crate::common::Reg<regs::Intrrx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Interrupt enable register for INTRTX."]
    #[inline(always)]
    pub const fn intrtxe(self) -> crate::common::Reg<regs::Intrtxe, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06usize) as _) }
    }
    #[doc = "Interrupt enable register for INTRRX."]
    #[inline(always)]
    pub const fn intrrxe(self) -> crate::common::Reg<regs::Intrrxe, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Interrupt register for common USB interrupts."]
    #[inline(always)]
    pub const fn intrusb(self) -> crate::common::Reg<regs::Intrusb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ausize) as _) }
    }
    #[doc = "Interrupt enable register for INTRUSB."]
    #[inline(always)]
    pub const fn intrusbe(self) -> crate::common::Reg<regs::Intrusbe, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0busize) as _) }
    }
    #[doc = "Frame number."]
    #[inline(always)]
    pub const fn frame(self) -> crate::common::Reg<regs::Frame, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Index register for selecting the endpoint status and control registers."]
    #[inline(always)]
    pub const fn index(self) -> crate::common::Reg<regs::Index, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0eusize) as _) }
    }
    #[doc = "Enables the USB 2.0 test modes."]
    #[inline(always)]
    pub const fn testmode(self) -> crate::common::Reg<regs::Testmode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fusize) as _) }
    }
    #[doc = "Maximum packet size for peripheral TX endpoint."]
    #[inline(always)]
    pub const fn txmaxp(self) -> crate::common::Reg<regs::Maxp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Control Status register lower byte for Endpoint 0."]
    #[inline(always)]
    pub const fn csr0l(self) -> crate::common::Reg<regs::Csr0l, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x12usize) as _) }
    }
    #[doc = "Control Status register lower byte for peripheral TX endpoint."]
    #[inline(always)]
    pub const fn txcsrl(self) -> crate::common::Reg<regs::Txcsrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x12usize) as _) }
    }
    #[doc = "Control Status register higher byte for Endpoint 0."]
    #[inline(always)]
    pub const fn csr0h(self) -> crate::common::Reg<regs::Csr0h, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x13usize) as _) }
    }
    #[doc = "Control Status register higher byte for peripheral TX endpoint."]
    #[inline(always)]
    pub const fn txcsrh(self) -> crate::common::Reg<regs::Txcsrh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x13usize) as _) }
    }
    #[doc = "Maximum packet size for peripheral Rx endpoint."]
    #[inline(always)]
    pub const fn rxmaxp(self) -> crate::common::Reg<regs::Maxp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Control Status register lower byte for peripheral Rx endpoint."]
    #[inline(always)]
    pub const fn rxcsrl(self) -> crate::common::Reg<regs::Rxcsrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x16usize) as _) }
    }
    #[doc = "Control Status register higher byte for peripheral Rx endpoint."]
    #[inline(always)]
    pub const fn rxcsrh(self) -> crate::common::Reg<regs::Rxcsrh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x17usize) as _) }
    }
    #[doc = "Number of received bytes in Endpoint 0 FIFO."]
    #[inline(always)]
    pub const fn count0(self) -> crate::common::Reg<regs::Count0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Number of bytes to be read from peripheral Rx endpoint FIFO."]
    #[inline(always)]
    pub const fn rxcount(self) -> crate::common::Reg<regs::Rxcount, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Returns details of core configuration."]
    #[inline(always)]
    pub const fn configdata(self) -> crate::common::Reg<regs::Configdata, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1fusize) as _) }
    }
    #[doc = "FIFO Size Register for TX and RX Endpoints"]
    #[inline(always)]
    pub const fn fifosize(self) -> crate::common::Reg<regs::Fifosize, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1fusize) as _) }
    }
    #[doc = "FIFO for endpoints."]
    #[inline(always)]
    pub const fn fifo(self, n: usize) -> crate::common::Reg<regs::Fifo, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize + n * 4usize) as _) }
    }
    #[doc = "used to select whether the MUSBMHDRC is operating in Peripheral mode or in Host mode, and for controlling and monitoring the USB VBus line."]
    #[inline(always)]
    pub const fn devctl(self) -> crate::common::Reg<regs::Devctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "controls the size of the selected TX endpoint FIFO"]
    #[inline(always)]
    pub const fn tx_fifo_sz(self) -> crate::common::Reg<regs::FifoSz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x62usize) as _) }
    }
    #[doc = "controls the size of the selected Rx endpoint FIFO"]
    #[inline(always)]
    pub const fn rx_fifo_sz(self) -> crate::common::Reg<regs::FifoSz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x63usize) as _) }
    }
    #[doc = "controls the start address of the selected Tx endpoint FIFO"]
    #[inline(always)]
    pub const fn tx_fifo_add(self) -> crate::common::Reg<regs::FifoAdd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "controls the start address of the selected Rx endpoint FIFO"]
    #[inline(always)]
    pub const fn rx_fifo_add(self) -> crate::common::Reg<regs::FifoAdd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x66usize) as _) }
    }
    #[doc = "Double Packet Buffer Disable register."]
    #[inline(always)]
    pub const fn tx_dpktbufdis(self) -> crate::common::Reg<regs::Dpktbufdis, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0342usize) as _) }
    }
    #[doc = "Double Packet Buffer Disable register."]
    #[inline(always)]
    pub const fn rx_dpktbufdis(self) -> crate::common::Reg<regs::Dpktbufdis, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0344usize) as _) }
    }
}
pub mod regs {
    #[doc = "Core configuration information register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Configdata(pub u8);
    impl Configdata {
        #[doc = "UTMI+ data width selection"]
        #[inline(always)]
        pub const fn utmi_data_width(&self) -> super::vals::UtmiWidth {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::UtmiWidth::from_bits(val as u8)
        }
        #[doc = "UTMI+ data width selection"]
        #[inline(always)]
        pub fn set_utmi_data_width(&mut self, val: super::vals::UtmiWidth) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
        }
        #[doc = "Soft Connect/Disconnect feature"]
        #[inline(always)]
        pub const fn soft_con_e(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Soft Connect/Disconnect feature"]
        #[inline(always)]
        pub fn set_soft_con_e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "Dynamic FIFO Sizing option"]
        #[inline(always)]
        pub const fn dyn_fifo_sizing(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Dynamic FIFO Sizing option"]
        #[inline(always)]
        pub fn set_dyn_fifo_sizing(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "High-bandwidth TX ISO Endpoint Support"]
        #[inline(always)]
        pub const fn hbtxe(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "High-bandwidth TX ISO Endpoint Support"]
        #[inline(always)]
        pub fn set_hbtxe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "High-bandwidth Rx ISO Endpoint Support"]
        #[inline(always)]
        pub const fn hbrxe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "High-bandwidth Rx ISO Endpoint Support"]
        #[inline(always)]
        pub fn set_hbrxe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Endian ordering indicator"]
        #[inline(always)]
        pub const fn big_endian(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Endian ordering indicator"]
        #[inline(always)]
        pub fn set_big_endian(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Automatic bulk packet splitting"]
        #[inline(always)]
        pub const fn mptxe(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic bulk packet splitting"]
        #[inline(always)]
        pub fn set_mptxe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Automatic bulk packet amalgamation"]
        #[inline(always)]
        pub const fn mprxe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic bulk packet amalgamation"]
        #[inline(always)]
        pub fn set_mprxe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Configdata {
        #[inline(always)]
        fn default() -> Configdata {
            Configdata(0)
        }
    }
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
    #[doc = "USB Endpoint 0 Control and Status Register High"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr0h(pub u8);
    impl Csr0h {
        #[doc = "Reset FIFO pointer and clear packet ready status"]
        #[inline(always)]
        pub const fn flush_fifo(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Reset FIFO pointer and clear packet ready status"]
        #[inline(always)]
        pub fn set_flush_fifo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
    }
    impl Default for Csr0h {
        #[inline(always)]
        fn default() -> Csr0h {
            Csr0h(0)
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
    #[doc = "Device Control Register for USB mode and VBus monitoring"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Devctl(pub u8);
    impl Devctl {
        #[doc = "Control or monitor USB session state"]
        #[inline(always)]
        pub const fn session(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Control or monitor USB session state"]
        #[inline(always)]
        pub fn set_session(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Initiate Host Negotiation Protocol"]
        #[inline(always)]
        pub const fn host_req(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Initiate Host Negotiation Protocol"]
        #[inline(always)]
        pub fn set_host_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "Indicates USB Host mode operation"]
        #[inline(always)]
        pub const fn host_mode(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates USB Host mode operation"]
        #[inline(always)]
        pub fn set_host_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "VBus voltage level indication"]
        #[inline(always)]
        pub const fn vbus(&self) -> super::vals::VbusLevel {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::VbusLevel::from_bits(val as u8)
        }
        #[doc = "VBus voltage level indication"]
        #[inline(always)]
        pub fn set_vbus(&mut self, val: super::vals::VbusLevel) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u8) & 0x03) << 3usize);
        }
        #[doc = "Low-speed device detection"]
        #[inline(always)]
        pub const fn ls_dev(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Low-speed device detection"]
        #[inline(always)]
        pub fn set_ls_dev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Full-speed or high-speed device detection"]
        #[inline(always)]
        pub const fn fs_dev(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Full-speed or high-speed device detection"]
        #[inline(always)]
        pub fn set_fs_dev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Indicates device type in USB session"]
        #[inline(always)]
        pub const fn b_device(&self) -> super::vals::DeviceType {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::DeviceType::from_bits(val as u8)
        }
        #[doc = "Indicates device type in USB session"]
        #[inline(always)]
        pub fn set_b_device(&mut self, val: super::vals::DeviceType) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Devctl {
        #[inline(always)]
        fn default() -> Devctl {
            Devctl(0)
        }
    }
    #[doc = "Indicates which of the endpoints have disabled the double packet buffer functionality described in section 8.4.2.2 of the MUSBMHDRC Product Specification"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dpktbufdis(pub u16);
    impl Dpktbufdis {
        #[doc = "Double Packet Buffer Disable for Tx/Rx Endpoint x (except EP0)"]
        #[inline(always)]
        pub const fn dis(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Double Packet Buffer Disable for Tx/Rx Endpoint x (except EP0)"]
        #[inline(always)]
        pub fn set_dis(&mut self, n: usize, val: bool) {
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
    #[doc = "controls the start address of the selected endpoint FIFO"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FifoAdd(pub u16);
    impl FifoAdd {
        #[doc = "Start address of the endpoint FIFO in units of 8 bytes"]
        #[inline(always)]
        pub const fn add(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "Start address of the endpoint FIFO in units of 8 bytes"]
        #[inline(always)]
        pub fn set_add(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u16) & 0x1fff) << 0usize);
        }
    }
    impl Default for FifoAdd {
        #[inline(always)]
        fn default() -> FifoAdd {
            FifoAdd(0)
        }
    }
    #[doc = "controls the size of the selected endpoint FIFO"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FifoSz(pub u8);
    impl FifoSz {
        #[doc = "Maximum packet size to be allowed for (before any splitting within the FIFO of Bulk/High Bandwidth packets prior to transmission – see Sections 8.4.1.3, 8.4.1.4 and 8.5.3)"]
        #[inline(always)]
        pub const fn sz(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Maximum packet size to be allowed for (before any splitting within the FIFO of Bulk/High Bandwidth packets prior to transmission – see Sections 8.4.1.3, 8.4.1.4 and 8.5.3)"]
        #[inline(always)]
        pub fn set_sz(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u8) & 0x07) << 0usize);
        }
        #[doc = "Defines whether double-packet buffering supported"]
        #[inline(always)]
        pub const fn dpb(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Defines whether double-packet buffering supported"]
        #[inline(always)]
        pub fn set_dpb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
    }
    impl Default for FifoSz {
        #[inline(always)]
        fn default() -> FifoSz {
            FifoSz(0)
        }
    }
    #[doc = "FIFO Size Register for TX and RX Endpoints"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fifosize(pub u8);
    impl Fifosize {
        #[doc = "Size of the selected Tx endpoint FIFO (2^n bytes, 0 if not configured)"]
        #[inline(always)]
        pub const fn tx_fifo_size(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Size of the selected Tx endpoint FIFO (2^n bytes, 0 if not configured)"]
        #[inline(always)]
        pub fn set_tx_fifo_size(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "Size of the selected Rx endpoint FIFO (2^n bytes, 0 if not configured)"]
        #[inline(always)]
        pub const fn rx_fifo_size(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Size of the selected Rx endpoint FIFO (2^n bytes, 0 if not configured)"]
        #[inline(always)]
        pub fn set_rx_fifo_size(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
        }
    }
    impl Default for Fifosize {
        #[inline(always)]
        fn default() -> Fifosize {
            Fifosize(0)
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
    pub struct Intrrx(pub u16);
    impl Intrrx {
        #[doc = "Receive endpoint interrupt (except EP0)"]
        #[inline(always)]
        pub const fn ep_rx(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Receive endpoint interrupt (except EP0)"]
        #[inline(always)]
        pub fn set_ep_rx(&mut self, n: usize, val: bool) {
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
    #[doc = "Receive Endpoint Interrupt Enable Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intrrxe(pub u16);
    impl Intrrxe {
        #[doc = "Endpoint transmit interrupt enable (except EP0)"]
        #[inline(always)]
        pub const fn ep_rxe(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Endpoint transmit interrupt enable (except EP0)"]
        #[inline(always)]
        pub fn set_ep_rxe(&mut self, n: usize, val: bool) {
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
    #[doc = "Transmit Endpoint Interrupt Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intrtx(pub u16);
    impl Intrtx {
        #[doc = "Endpoint 0 and transmit endpoints interrupt"]
        #[inline(always)]
        pub const fn ep_tx(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Endpoint 0 and transmit endpoints interrupt"]
        #[inline(always)]
        pub fn set_ep_tx(&mut self, n: usize, val: bool) {
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
    #[doc = "Transmit Endpoint Interrupt Enable Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intrtxe(pub u16);
    impl Intrtxe {
        #[doc = "Endpoint transmit interrupt enable (EP0:TXE_RXE)"]
        #[inline(always)]
        pub const fn ep_txe(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Endpoint transmit interrupt enable (EP0:TXE_RXE)"]
        #[inline(always)]
        pub fn set_ep_txe(&mut self, n: usize, val: bool) {
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
        #[doc = "Reset signaling detected (Peripheral mode) or Babble detected (Host mode)"]
        #[inline(always)]
        pub const fn reset(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Reset signaling detected (Peripheral mode) or Babble detected (Host mode)"]
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
        #[doc = "Device connection detected"]
        #[inline(always)]
        pub const fn conn(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Device connection detected"]
        #[inline(always)]
        pub fn set_conn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Device disconnection detected"]
        #[inline(always)]
        pub const fn discon(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Device disconnection detected"]
        #[inline(always)]
        pub fn set_discon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Session Request signaling detected"]
        #[inline(always)]
        pub const fn sess_req(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Session Request signaling detected"]
        #[inline(always)]
        pub fn set_sess_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "VBus drops below valid threshold"]
        #[inline(always)]
        pub const fn vbus_error(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "VBus drops below valid threshold"]
        #[inline(always)]
        pub fn set_vbus_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
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
        #[doc = "Enable Connection interrupt"]
        #[inline(always)]
        pub const fn conn_enable(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Connection interrupt"]
        #[inline(always)]
        pub fn set_conn_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Enable Disconnection interrupt"]
        #[inline(always)]
        pub const fn discon_enable(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Disconnection interrupt"]
        #[inline(always)]
        pub fn set_discon_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Enable Session Request interrupt"]
        #[inline(always)]
        pub const fn sess_req_enable(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Session Request interrupt"]
        #[inline(always)]
        pub fn set_sess_req_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Enable VBus Error interrupt"]
        #[inline(always)]
        pub const fn vbus_error_enable(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Enable VBus Error interrupt"]
        #[inline(always)]
        pub fn set_vbus_error_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Intrusbe {
        #[inline(always)]
        fn default() -> Intrusbe {
            Intrusbe(0)
        }
    }
    #[doc = "Maximum payload size forendpoint"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maxp(pub u16);
    impl Maxp {
        #[doc = "Maximum payload"]
        #[inline(always)]
        pub const fn maxp(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Maximum payload"]
        #[inline(always)]
        pub fn set_maxp(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
        }
    }
    impl Default for Maxp {
        #[inline(always)]
        fn default() -> Maxp {
            Maxp(0)
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
        #[doc = "High-speed mode negotiation status"]
        #[inline(always)]
        pub const fn hs_mode(&self) -> super::vals::HsModeStatus {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::HsModeStatus::from_bits(val as u8)
        }
        #[doc = "High-speed mode negotiation status"]
        #[inline(always)]
        pub fn set_hs_mode(&mut self, val: super::vals::HsModeStatus) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
        }
        #[doc = "Enable High-speed mode negotiation"]
        #[inline(always)]
        pub const fn hs_enab(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Enable High-speed mode negotiation"]
        #[inline(always)]
        pub fn set_hs_enab(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Enable/disable USB D+/D- lines"]
        #[inline(always)]
        pub const fn soft_conn(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Enable/disable USB D+/D- lines"]
        #[inline(always)]
        pub fn set_soft_conn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
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
        #[doc = "Incomplete packet in high-bandwidth Isochronous/Interrupt transfer"]
        #[inline(always)]
        pub const fn incomp_rx(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Incomplete packet in high-bandwidth Isochronous/Interrupt transfer"]
        #[inline(always)]
        pub fn set_incomp_rx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Select DMA Request Mode"]
        #[inline(always)]
        pub const fn dma_req_mode(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Select DMA Request Mode"]
        #[inline(always)]
        pub fn set_dma_req_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Disable NYET handshakes or indicate PID error"]
        #[inline(always)]
        pub const fn dis_nyet_pid_error(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Disable NYET handshakes or indicate PID error"]
        #[inline(always)]
        pub fn set_dis_nyet_pid_error(&mut self, val: bool) {
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
    #[doc = "USB test mode configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Testmode(pub u8);
    impl Testmode {
        #[doc = "Enter Test_SE0_NAK high-speed test mode"]
        #[inline(always)]
        pub const fn test_se0_nak(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enter Test_SE0_NAK high-speed test mode"]
        #[inline(always)]
        pub fn set_test_se0_nak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Enter Test_J high-speed test mode"]
        #[inline(always)]
        pub const fn test_j(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Enter Test_J high-speed test mode"]
        #[inline(always)]
        pub fn set_test_j(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "Enter Test_K high-speed test mode"]
        #[inline(always)]
        pub const fn test_k(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Enter Test_K high-speed test mode"]
        #[inline(always)]
        pub fn set_test_k(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "Enter Test_Packet high-speed test mode"]
        #[inline(always)]
        pub const fn test_packet(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Enter Test_Packet high-speed test mode"]
        #[inline(always)]
        pub fn set_test_packet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Force High-speed mode on USB reset"]
        #[inline(always)]
        pub const fn force_hs(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Force High-speed mode on USB reset"]
        #[inline(always)]
        pub fn set_force_hs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Force Full-speed mode on USB reset"]
        #[inline(always)]
        pub const fn force_fs(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Force Full-speed mode on USB reset"]
        #[inline(always)]
        pub fn set_force_fs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Transfer packet from Endpoint 0 TX FIFO to Endpoint 0 Rx FIFO"]
        #[inline(always)]
        pub const fn fifo_access(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer packet from Endpoint 0 TX FIFO to Endpoint 0 Rx FIFO"]
        #[inline(always)]
        pub fn set_fifo_access(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Force core to enter Host mode"]
        #[inline(always)]
        pub const fn force_host(&self) -> super::vals::ForceHostMode {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::ForceHostMode::from_bits(val as u8)
        }
        #[doc = "Force core to enter Host mode"]
        #[inline(always)]
        pub fn set_force_host(&mut self, val: super::vals::ForceHostMode) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Testmode {
        #[inline(always)]
        fn default() -> Testmode {
            Testmode(0)
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
        #[doc = "Incomplete high-bandwidth Isochronous transfer"]
        #[inline(always)]
        pub const fn incomp_tx(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Incomplete high-bandwidth Isochronous transfer"]
        #[inline(always)]
        pub fn set_incomp_tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Txcsrl {
        #[inline(always)]
        fn default() -> Txcsrl {
            Txcsrl(0)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
