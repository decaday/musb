#![no_std]
mod fmt;

pub mod regs;
pub use regs::common;
pub use regs::info::*;

#[cfg(feature = "embassy-usb-driver-impl")]
mod embassy_usb_driver_impl;
#[cfg(feature = "embassy-usb-driver-impl")]
pub use embassy_usb_driver_impl::*;


fn calc_max_fifo_size_dword(len: u16) -> u16 {
    let dwords = ((len + 7) / 8) as u16;
    if dwords > 8 {
        panic!("Invalid length: {}", len);
    }
    dwords
}



pub trait MusbInstance: 'static {
    fn regs() -> regs::Usb;
}