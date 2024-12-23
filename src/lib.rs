#![no_std]
mod fmt;

pub mod regs;
pub use regs::common;
pub use regs::info::*;

#[cfg(feature = "embassy-usb-driver-impl")]
mod embassy_usb_driver_impl;
#[cfg(feature = "embassy-usb-driver-impl")]
pub use embassy_usb_driver_impl::*;

#[cfg(feature = "usb-device-impl")]
mod usb_device_impl;
#[cfg(feature = "usb-device-impl")]
pub use usb_device_impl::*;

mod alloc_endpoint;



pub trait MusbInstance: 'static + Send + Sync {
    fn regs() -> regs::Usb;
}