use core::future::poll_fn;
use core::marker::PhantomData;
use core::sync::atomic::{AtomicBool, AtomicU16, Ordering};
use core::task::Poll;

use embassy_sync::waitqueue::AtomicWaker;
use embassy_usb_driver as driver;
use embassy_usb_driver::{
    Direction, EndpointAddress, EndpointError, EndpointInfo, EndpointType, Event, Unsupported,
};

use crate::*;
use crate::info::ENDPOINTS;

mod endpoint;
pub use endpoint::Endpoint;

#[path = "driver.rs"]
mod usb_driver;
pub use usb_driver::MusbDriver;

mod bus;
pub use bus::Bus;

mod control_pipe;
pub use control_pipe::ControlPipe;

const NEW_AW: AtomicWaker = AtomicWaker::new();

static BUS_WAKER: AtomicWaker = NEW_AW;

static EP_TX_WAKERS: [AtomicWaker; ENDPOINTS.len()] = [NEW_AW; ENDPOINTS.len()];
static EP_RX_WAKERS: [AtomicWaker; ENDPOINTS.len()] = [NEW_AW; ENDPOINTS.len()];

static IRQ_RESET: AtomicBool = AtomicBool::new(false);
static IRQ_SUSPEND: AtomicBool = AtomicBool::new(false);
static IRQ_RESUME: AtomicBool = AtomicBool::new(false);
static EP_TX_ENABLED: AtomicU16 = AtomicU16::new(0);
static EP_RX_ENABLED: AtomicU16 = AtomicU16::new(0);

#[inline(always)]
pub unsafe fn on_interrupt<T: MusbInstance>() {
    let usb_status = crate::bus::read_and_clear_usb_interrupts::<T>();
    let tx_status = crate::endpoint::read_and_clear_tx_interrupts::<T>();
    let rx_status = crate::endpoint::read_and_clear_rx_interrupts::<T>();
    
    trace!("musb/on_interrupt: reset: {}, suspend: {}, resume: {}, tx: {:b}, rx: {:b}", 
        usb_status.reset, usb_status.suspend, usb_status.resume, tx_status, rx_status);

    if usb_status.reset {
        IRQ_RESET.store(true, Ordering::SeqCst);
        BUS_WAKER.wake();
    }
    if usb_status.suspend {
        IRQ_SUSPEND.store(true, Ordering::SeqCst);
        BUS_WAKER.wake();
    }
    if usb_status.resume {
        IRQ_RESUME.store(true, Ordering::SeqCst);
        BUS_WAKER.wake();
    }
    
    if (tx_status & 1) != 0 {
        EP_TX_WAKERS[0].wake();
        EP_RX_WAKERS[0].wake();
    }

    for index in 1..ENDPOINTS.len() {
        if (tx_status & (1 << index)) != 0 {
            EP_TX_WAKERS[index].wake();
        }
        if (rx_status & (1 << index)) != 0 {
            EP_RX_WAKERS[index].wake();
        }

        // TODO: move to another location
        // T::regs().index().write(|w| w.set_index(index as _));
        // if T::regs().txcsrl().read().under_run(){
        //     T::regs().txcsrl().modify(|w| w.set_under_run(false));
        //     warn!("Underrun: ep {}", index);
        // }
    }
}

pub trait Dir {
    fn dir() -> Direction;
}

/// Marker type for the "IN" direction.
pub enum In {}
impl Dir for In {
    fn dir() -> Direction {
        Direction::In
    }
}

/// Marker type for the "OUT" direction.
pub enum Out {}
impl Dir for Out {
    fn dir() -> Direction {
        Direction::Out
    }
}
