#![no_std]
mod fmt;

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

pub mod generated;
pub use generated::common;
pub use generated::regs;

mod info {
    pub use crate::generated::ENDPOINTS;

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum EpDirection {
        TX,
        RX,
        RXTX,
    }

    pub struct EpInfo {
        pub ep_direction: EpDirection,
        pub max_packet_size: u16,
    }
}
#[cfg(feature = "_gen-usb-instance")]
pub use generated::UsbInstance;

pub trait MusbInstance: 'static + Send + Sync {
    fn regs() -> regs::Usb;
}
