#![no_std]
use core::future::poll_fn;
use core::marker::PhantomData;
use core::sync::atomic::{AtomicBool, AtomicU16, Ordering};
use core::task::Poll;

use embassy_sync::waitqueue::AtomicWaker;
use embassy_usb_driver as driver;
use embassy_usb_driver::{
    Direction, EndpointAddress, EndpointAllocError, EndpointError, EndpointInfo, EndpointType, Event, Unsupported,
};

mod fmt;

pub mod regs;
pub use regs::common;
pub use regs::UsbInstance;

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

// TODO
const EP_COUNT: usize = 6;


const MAX_FIFO_SIZE_BTYES: [u8; EP_COUNT] = [8, 8, 16, 16, 16, 64];

// TODO
// const MAX_FIFO_SIZE_BTYES: u8 = 8;

const NEW_AW: AtomicWaker = AtomicWaker::new();

static BUS_WAKER: AtomicWaker = NEW_AW;

static EP_TX_WAKERS: [AtomicWaker; EP_COUNT] = [NEW_AW; EP_COUNT];
static EP_RX_WAKERS: [AtomicWaker; EP_COUNT] = [NEW_AW; EP_COUNT];

static IRQ_RESET: AtomicBool = AtomicBool::new(false);
static IRQ_SUSPEND: AtomicBool = AtomicBool::new(false);
static IRQ_RESUME: AtomicBool = AtomicBool::new(false);
static EP_TX_ENABLED: AtomicU16 = AtomicU16::new(0);
static EP_RX_ENABLED: AtomicU16 = AtomicU16::new(0);

fn calc_max_fifo_size_btyes(len: u16) -> u16 {
    let btyes = ((len + 7) / 8) as u16;
    if btyes > 8 {
        panic!("Invalid length: {}", len);
    }
    btyes
}

/// Interrupt handler.
pub struct InterruptHandler<T: MusbInstance> {
    _phantom: PhantomData<T>,
}

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

    for index in 1..EP_COUNT {
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

pub trait MusbInstance: 'static {
    fn regs() -> regs::Usb;
}

// typedef volatile struct
// {
//     /* common registers */
//     REG8    faddr;                          //0x0000
//     REG8    power;                          //0x0001
//     REG16   intrtx;                         //0x0002
//     REG16   intrrx;                         //0x0004
//     REG16   intrtxe;                        //0x0006
//     REG16   intrrxe;                        //0x0008
//     REG8    intrusb;                        //0x000a
//     REG8    intrusbe;                       //0x000b
//     REG16   frame;                          //0x000c
//     REG8    index;                          //0x000e
//     REG8    testmode;                       //0x000f
//     /* indexed registers */
//     REG16   txmaxp;                         //0x0010
//     REG16   csr0_txcsr;                     //0x0012
//     REG16   rxmaxp;                         //0x0014
//     REG16   rxcsr;                          //0x0016
//     REG16   rxcount;                        //0x0018
//     REG8    txtype;                         //0x001a
//     REG8    txinterval;                     //0x001b
//     REG8    rxtype;                         //0x001c
//     REG8    rxinterval;                     //0x001d
//     REG8    reserved0;                      //0x001e
//     REG8    cfdt_fifosz;                    //0x001f
//     /* fifo */
//     REG32    fifox[0x10];                   //0x0020
//     /* OTG, dynamic FIFO, version & vendor registers */
//     REG8    devctl;                         //0x0060
//     REG8    reserved1;                      //0x0061
//     REG8    txfifosz;                       //0x0062
//     REG8    rxfifosz;                       //0x0063
//     REG16    txfifoadd;                     //0x0064
//     REG16    rxfifoadd;                     //0x0066
//     REG32    vcontrol;                      //0x0068
//     REG16    hwvers;                        //0x006c
//     REG16    reserved2a[1];                 //0x006e
//     REG8    ulpi_busctl;                    //0x0070
//     REG8    reserved2b[1];                  //0x0071
//     REG16    reserved2[3];                  //0x0072
//     REG8    epinfo;                         //0x0078
//     REG8    raminfo;                        //0x0079
//     REG8    linkinfo;                       //0x007a
//     REG8    vplen;                          //0x007b
//     REG8    hseof1;                         //0x007c
//     REG8    fseof1;                         //0x007d
//     REG8    lseof1;                         //0x007e
//     REG8    soft_rst;                       //0x007f
//     /* target address registers */          //0x0080
//     struct musb_tar_regs
//     {
//         REG8    txfuncaddr;
//         REG8    reserved0;
//         REG8    txhubaddr;
//         REG8    txhubport;
//         REG8    rxfuncaddr;
//         REG8    reserved1;
//         REG8    rxhubaddr;
//         REG8    rxhubport;
//     } tar[0x10];
//     /*
//      * endpoint registers
//      * ep0 elements are valid when array index is 0
//      * otherwise epN is valid
//      */
//     union musb_ep_regs                      //0x0100
//     {
//         struct musb_ep0_regs ep0;
//         struct musb_epN_regs epN;
//     } ep[0x10];

//     //REG32 reserved1fe;                    //0x0200
//     REG32 dmaintr;

//     struct musb_dma_regs
//     {
//         REG32    cntl;                      //0x0204
//         REG32    addr;                      //0x0208
//         REG32    count;                     //0x020c
//         REG32    rsvd;                      //0x0210
//     } dma[0x10];


//     struct musb_reserved_1
//     {
//         REG32  reserved_1;                  //0x0304
//     } rsvd_1[0xf];

//     REG8 dpbrxdisl;                         //0x0340
//     REG8 dpbrxdish;                         //0x0341
//     REG8 dpbtxdisl;                         //0x0342
//     REG8 dpbtxdish;                         //0x0343

//     struct musb_reserved_2
//     {
//         REG32 reserved_2;                      //0x0344
//     } rsvd_2[0x9];

//     REG8    dbgl;                           //0x0368
//     REG8    dbgh;                           //0x0369
//     REG16   reserved_0;                     //0x036a
//     REG32   reserved_1;                     //0x036c
//     REG8    usbcfg;                         //0x0370

//     struct musb_reserved_3
//     {
//         REG8 reserved_3;                    //0x0371~0x3bb
//     } rsvd_3[0x4b];

//     REG8     dbgcntl0;                     //0x3bc
//     REG8     dbgcntl1;                     //0x3bd
// } __attribute__((packed, aligned(32))) USBC_X_Typedef;