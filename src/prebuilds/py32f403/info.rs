pub struct UsbInstance;
impl crate::MusbInstance for UsbInstance {
    fn regs() -> crate::regs::Usb {
        unsafe { crate::regs::Usb::from_ptr((0x40005c00) as _ ) }
    }
}
pub const ENDPOINTS_NUM: usize = 8;
pub const MAX_FIFO_SIZE_BYTE: u8 = 8;
