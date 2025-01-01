#![no_std]
mod fmt;

pub mod regs;
pub use regs::common;
pub use regs::info::*;

#[cfg(all(feature = "embassy-usb-driver-impl", feature = "usb-device-impl"))]
compile_error!(
    "The `embassy-usb-driver-impl` feature is incompatible with the `usb-device-impl` feature. "
);

#[cfg(feature = "embassy-usb-driver-impl")]
mod embassy_usb_driver_impl;
#[cfg(feature = "embassy-usb-driver-impl")]
pub use embassy_usb_driver_impl::*;

#[cfg(feature = "usb-device-impl")]
mod usb_device_impl;
#[cfg(feature = "usb-device-impl")]
pub use usb_device_impl::*;

mod alloc_endpoint;
mod common_impl;

pub trait MusbInstance: 'static + Send + Sync {
    fn regs() -> regs::Usb;
}
