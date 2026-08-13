#![no_std]
pub mod types;
mod fmt;

#[cfg(feature = "embassy-usb-driver-impl")]
mod embassy_usb_driver_impl;
#[cfg(feature = "embassy-usb-driver-impl")]
pub use embassy_usb_driver_impl::*;

#[cfg(feature = "usb-device-impl")]
mod usb_device_impl;
#[cfg(feature = "usb-device-impl")]
pub use usb_device_impl::*;

#[cfg(not(feature = "builtin-readconf"))]
pub mod alloc_endpoint;
pub mod bus;
pub mod endpoint;
pub mod endpoint0;

pub mod generated;
pub use generated::common;
pub use generated::regs;

pub mod info {
    pub use crate::generated::ENDPOINTS;
    #[cfg(not(feature = "_fixed-fifo-size"))]
    pub use crate::generated::TOTAL_FIFO_SIZE;

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


// In fact, only the function name `on_interrupt` conflicts.
// However, it is unlikely that anyone would use both at the same time.
#[cfg(all(feature = "embassy-usb-driver-impl", feature = "usb-device-impl"))]
compile_error!(
    "The `embassy-usb-driver-impl` feature is incompatible with the `usb-device-impl` feature. "
);
