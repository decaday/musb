use core::future::poll_fn;
use core::marker::PhantomData;
use core::sync::atomic::{AtomicBool, AtomicU16, Ordering};
use core::task::Poll;

use embassy_sync::waitqueue::AtomicWaker;
use embassy_usb_driver as driver;
use embassy_usb_driver::{
    Direction, EndpointAddress, EndpointAllocError, EndpointError, EndpointInfo, EndpointType, Event, Unsupported,
};

use crate::*;

mod endpoint;
pub use endpoint::Endpoint;
use endpoint::{EndpointData, EndPointConfig};

#[path ="driver.rs"]
mod usb_driver;
pub use usb_driver::MusbDriver;

mod bus;
pub use bus::Bus;

mod control_pipe;
pub use control_pipe::ControlPipe;

const NEW_AW: AtomicWaker = AtomicWaker::new();

static BUS_WAKER: AtomicWaker = NEW_AW;

static EP_TX_WAKERS: [AtomicWaker; ENDPOINTS_NUM] = [NEW_AW; ENDPOINTS_NUM];
static EP_RX_WAKERS: [AtomicWaker; ENDPOINTS_NUM] = [NEW_AW; ENDPOINTS_NUM];

static IRQ_RESET: AtomicBool = AtomicBool::new(false);
static IRQ_SUSPEND: AtomicBool = AtomicBool::new(false);
static IRQ_RESUME: AtomicBool = AtomicBool::new(false);
static EP_TX_ENABLED: AtomicU16 = AtomicU16::new(0);
static EP_RX_ENABLED: AtomicU16 = AtomicU16::new(0);

#[inline(always)]
pub unsafe fn on_interrupt<T: MusbInstance>() {
    let intrusb = T::regs().intrusb().read();
    if intrusb.reset() {
        IRQ_RESET.store(true, Ordering::SeqCst);
        BUS_WAKER.wake();
    }
    if intrusb.suspend() {
        IRQ_SUSPEND.store(true, Ordering::SeqCst);
        BUS_WAKER.wake();
    }
    if intrusb.resume() {
        IRQ_RESUME.store(true, Ordering::SeqCst);
        BUS_WAKER.wake();
    }

    let intrtx = T::regs().intrtx().read();
    let intrrx = T::regs().intrrx().read();
    if intrtx.ep_tx(0) {
        EP_TX_WAKERS[0].wake();
        EP_RX_WAKERS[0].wake();
    }

    for index in 1..ENDPOINTS_NUM {
        if intrtx.ep_tx(index) {
            EP_TX_WAKERS[index].wake();
        }
        if intrrx.ep_rx(index) {                
            EP_RX_WAKERS[index].wake();
        }
        if T::regs().txcsrl().read().under_run(){
            T::regs().txcsrl().modify(|w| w.set_under_run(false));
            warn!("Underrun: ep {}", index);
        }
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