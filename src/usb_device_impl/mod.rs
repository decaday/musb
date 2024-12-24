
use core::marker::PhantomData;
use core::sync::atomic::{AtomicBool, AtomicU16, Ordering};

use embassy_usb_driver::{Direction, EndpointType};
use usb_device::bus::PollResult;
use usb_device::{UsbDirection, UsbError};

use crate::common_impl;
use crate::{trace, warn};
use crate::{alloc_endpoint, MusbInstance, ENDPOINTS_NUM};
use crate::alloc_endpoint::{EndpointAllocError, EndpointConfig, EndpointData};


pub struct UsbdBus<T: MusbInstance> {
    phantom: PhantomData<T>,
    alloc: [EndpointData; ENDPOINTS_NUM],
}

impl<T: MusbInstance> UsbdBus<T> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
            alloc: [EndpointData {
                ep_conf: EndpointConfig {
                    ep_type: EndpointType::Bulk,
                    tx_max_fifo_size_dword: 1,
                    rx_max_fifo_size_dword: 1,
                },
                used_tx: false,
                used_rx: false,
            }; ENDPOINTS_NUM],
        }
    }
}

impl<T: MusbInstance> usb_device::bus::UsbBus for UsbdBus<T> {
    fn alloc_ep(
        &mut self,
        ep_dir: usb_device::UsbDirection,
        ep_addr: Option<usb_device::endpoint::EndpointAddress>,
        ep_type: usb_device::endpoint::EndpointType,
        max_packet_size: u16,
        _interval: u8,
    ) -> usb_device::Result<usb_device::endpoint::EndpointAddress> {
        let index= ep_addr.map(|addr| addr.index() as u8);
        let ep_type = match ep_type {
            usb_device::endpoint::EndpointType::Bulk => EndpointType::Bulk,
            usb_device::endpoint::EndpointType::Interrupt => EndpointType::Interrupt,
            usb_device::endpoint::EndpointType::Isochronous{..} => EndpointType::Isochronous,
            usb_device::endpoint::EndpointType::Control => EndpointType::Control,
        };
        let dir = match ep_dir {
            usb_device::UsbDirection::In => Direction::In,
            usb_device::UsbDirection::Out => Direction::Out,
        };

        alloc_endpoint::alloc_endpoint(&mut self.alloc, ep_type, index, dir, max_packet_size)
            .map_err(|e| match e {
                EndpointAllocError::EndpointOverflow => UsbError::EndpointOverflow,
                EndpointAllocError::InvalidEndpoint => UsbError::InvalidEndpoint,
                #[cfg(not(feature = "_fixed-fifo-size"))]
                EndpointAllocError::BufferOverflow => UsbError::EndpointOverflow,
            })
            .map(|index| usb_device::endpoint::EndpointAddress::from_parts(index as usize, ep_dir))
    }

    fn enable(&mut self) {
        trace!("call enable");
        common_impl::bus_enable::<T>();
    }

    fn reset(&self) {
        trace!("call reset");
        T::regs().power().write(|w| w.set_suspend_mode(true));

        self.alloc.iter().enumerate().for_each(|(index, ep)| {
            if ep.used_tx { 
                trace!("call ep_tx_enable, index = {}", index);
                common_impl::ep_tx_enable::<T>(index as _, &ep.ep_conf);
            }
            if ep.used_rx {
                trace!("call ep_rx_enable, index = {}", index);
                common_impl::ep_rx_enable::<T>(index as _, &ep.ep_conf);
            }
        });
    }

    fn set_device_address(&self, addr: u8) {
        trace!("call set_device_address: {}", addr);
        T::regs().faddr().write(|w| w.set_func_addr(addr));
    }

    fn write(&self, ep_addr: usb_device::endpoint::EndpointAddress, buf: &[u8]) -> usb_device::Result<usize> {
        let index = ep_addr.index();
        trace!("WRITE len = {}, index = {} ", buf.len(), index);
        let regs = T::regs();
        regs.index().write(|w| w.set_index(index as _));

        // if buf.len() > self.alloc[index].ep_conf.tx_max_fifo_size_dword as usize * 8 {
        //     return Err(UsbError::BufferOverflow);
        // }
        let unready = if index == 0 {
            regs.csr0l().read().tx_pkt_rdy()
        } else {
            regs.txcsrl().read().tx_pkt_rdy()
        };
        if unready {
            return Err(UsbError::WouldBlock);
        }

        buf.into_iter().for_each(|b|
            regs.fifo(index).write(|w| w.set_data(*b))
        );
        
        if index == 0 {
            regs.csr0l().modify(|w| w.set_tx_pkt_rdy(true));
            // necessary
            if buf.len() < self.alloc[0].ep_conf.tx_max_fifo_size_dword as usize * 8 {
                // Last Package. include ZLP
                regs.csr0l().modify(|w| w.set_data_end(true));
                CONTROL_TRANSACTION.store(false, Ordering::Release);
            }
        } else {
            regs.txcsrl().modify(|w| w.set_tx_pkt_rdy(true));
        }
        trace!("WRITE OK");
        Ok(buf.len())
    }

    fn read(&self, ep_addr: usb_device::endpoint::EndpointAddress, buf: &mut [u8]) -> usb_device::Result<usize> {
        let index = ep_addr.index();
        trace!("READ, buf.len() = {}, index = {}", buf.len(), index);

        let regs = T::regs();
        regs.index().write(|w| w.set_index(index as _));

        let unready = if index == 0 {
            !regs.csr0l().read().rx_pkt_rdy()
        } else {
            !regs.rxcsrl().read().rx_pkt_rdy()
        };
        if unready {
            trace!("unready");
            return Err(UsbError::WouldBlock);
        }

        let read_count = if index == 0 {
            regs.count0().read().count() as u16
        } 
        else { 
            regs.rxcount().read().count()
        };
        trace!("read_count = {}", read_count);
        // if read_count as usize > buf.len() {
        //     panic!("read_count > buf.len()");
        //     return Err(UsbError::BufferOverflow);
        // }

        buf.into_iter().for_each(|b|
            *b = regs.fifo(index).read().data()
        );

        if index == 0 {
            regs.csr0l().modify(|w| w.set_serviced_rx_pkt_rdy(true));
            if SETUP.load(Ordering::Acquire) {
                SETUP.store(false, Ordering::Release);
            }

            if buf.len() < self.alloc[0].ep_conf.rx_max_fifo_size_dword as usize * 8 {
                // Last Package. include ZLP
                regs.csr0l().modify(|w| w.set_data_end(true));
                CONTROL_TRANSACTION.store(false, Ordering::Release);
            }
        } else {
            regs.rxcsrl().modify(|w| w.set_rx_pkt_rdy(false));
        }
        
        trace!("READ OK, rx_len = {}", read_count);

        Ok(read_count as usize)
    }

    fn set_stalled(&self, ep_addr: usb_device::endpoint::EndpointAddress, stalled: bool) {
        let index = ep_addr.index();
        match ep_addr.direction() {
            UsbDirection::In => common_impl::ep_tx_stall::<T>(index as _, stalled),
            UsbDirection::Out => common_impl::ep_rx_stall::<T>(index as _, stalled),
        }
    }

    fn is_stalled(&self, ep_addr: usb_device::endpoint::EndpointAddress) -> bool {
        match ep_addr.direction() {
            UsbDirection::In => common_impl::ep_tx_is_stalled::<T>(ep_addr.index() as _),
            UsbDirection::Out => common_impl::ep_rx_is_stalled::<T>(ep_addr.index() as _),
        }
    }

    fn suspend(&self) {
    }

    fn resume(&self) {
    }

    fn poll(&self) -> PollResult {
        let regs = T::regs();
        let mut setup = false;

        if IRQ_RESET.load(Ordering::Acquire) {
            IRQ_RESET.store(false, Ordering::Release);
            return PollResult::Reset;
        }
        if IRQ_RESUME.load(Ordering::Acquire) {
            IRQ_RESUME.store(false, Ordering::Release);
            return PollResult::Resume;
        }
        if IRQ_SUSPEND.load(Ordering::Acquire) {
            IRQ_RESET.store(false, Ordering::Release);
            return PollResult::Suspend;
        }

        if IRQ_EP0.load(Ordering::Acquire) {
            regs.index().write(|w| w.set_index(0));
            let rx_pkt_rdy = regs.csr0l().read().rx_pkt_rdy();
            let tx_pkt_rdy = regs.csr0l().read().tx_pkt_rdy();

            match (rx_pkt_rdy, tx_pkt_rdy) {
                (false, false) => {
                    // interrupt generated due to a packet has been transmitted
                    let flags = IRQ_EP_TX.load(Ordering::Acquire) | 1u16;
                    IRQ_EP_TX.store(flags, Ordering::Release);
                    IRQ_EP0.store(false, Ordering::Release);

                    let flags = IRQ_EP_RX.load(Ordering::Acquire) & !1u16;
                    IRQ_EP_RX.store(flags, Ordering::SeqCst);
                },
                (true, _) => {
                    if CONTROL_TRANSACTION.load(Ordering::Acquire) {
                        let flags = IRQ_EP_RX.load(Ordering::Acquire) | 1u16;
                        IRQ_EP_RX.store(flags, Ordering::Release);
                    } else {
                        let count = regs.count0().read().count();
                        match count {
                            0 => {
                                // ZLP?
                                // Last Package. include ZLP
                                regs.csr0l().modify(|w| w.set_data_end(true));
                                CONTROL_TRANSACTION.store(false, Ordering::Release);
                            },
                            8 => {
                                setup = true;
                                SETUP.store(true, Ordering::Release);
                                CONTROL_TRANSACTION.store(true, Ordering::Release);
                            }
                            _ => {
                                warn!("setup packet not 8 bytes long, count = {}", count);
                                CONTROL_TRANSACTION.store(true, Ordering::Release);
                            }
                        }
                    }
                },
                (false, true) => {
                    IRQ_EP0.store(false, Ordering::Release);
                    let flags = IRQ_EP_RX.load(Ordering::Acquire) & !1u16;
                    IRQ_EP_RX.store(flags, Ordering::SeqCst);
                }
            }
        }
        
        let rx_flags = IRQ_EP_RX.load(Ordering::Acquire);
        for index in BitIter(rx_flags) {
            if index == 0 {
                continue;
            }
            regs.index().write(|w| w.set_index(index as _));
            
            // clean flags after packet was read, rx_pkt_rdy == false
            if !regs.rxcsrl().read().rx_pkt_rdy() {
                IRQ_EP_RX.store(rx_flags & !((1 << index) as u16), Ordering::SeqCst);
            }
        }
        let in_complete = IRQ_EP_TX.load(Ordering::Acquire);
        IRQ_EP_TX.store(0, Ordering::SeqCst);
        let out = IRQ_EP_RX.load(Ordering::Acquire);

        if in_complete != 0 || out != 0 || setup {
            PollResult::Data { 
                ep_out: out, 
                ep_in_complete: in_complete, 
                ep_setup: if setup {1} else {0}, 
            }
        } else {
            PollResult::None
        }
    }
    
    fn force_reset(&self) -> usb_device::Result<()> {
        Err(UsbError::Unsupported)
    }
    
    const QUIRK_SET_ADDRESS_BEFORE_STATUS: bool = true;
}

static IRQ_RESET: AtomicBool = AtomicBool::new(false);
static IRQ_SUSPEND: AtomicBool = AtomicBool::new(false);
static IRQ_RESUME: AtomicBool = AtomicBool::new(false);

static IRQ_EP_TX: AtomicU16 = AtomicU16::new(0);
static IRQ_EP_RX: AtomicU16 = AtomicU16::new(0);
static IRQ_EP0: AtomicBool = AtomicBool::new(false);
static SETUP: AtomicBool = AtomicBool::new(false);
static CONTROL_TRANSACTION: AtomicBool = AtomicBool::new(false);

#[inline(always)]
pub unsafe fn on_interrupt<T: MusbInstance>() {
    let intrusb = T::regs().intrusb().read();
    if intrusb.reset() {
        IRQ_RESET.store(true, Ordering::SeqCst);
    }
    if intrusb.suspend() {
        IRQ_SUSPEND.store(true, Ordering::SeqCst);
    }
    if intrusb.resume() {
        IRQ_RESUME.store(true, Ordering::SeqCst);
    }

    let intrtx = T::regs().intrtx().read();
    let intrrx = T::regs().intrrx().read();
    if intrtx.ep_tx(0) {
        IRQ_EP0.store(true, Ordering::SeqCst);
    }
    
    let intrtxe = T::regs().intrtxe().read();
    crate::info!("intrtx: {:b}, intrrx: {:b}, intrtxe: {:b}", intrtx.0, intrrx.0, intrtxe.0);

    for index in 1..ENDPOINTS_NUM {
        if intrtx.ep_tx(index) {
            let flags = IRQ_EP_TX.load(Ordering::Acquire) | ( 1 << index ) as u16;
            IRQ_EP_TX.store(flags, Ordering::Release);
        }
        if intrrx.ep_rx(index) {             
            let flags = IRQ_EP_RX.load(Ordering::Acquire) | ( 1 << index ) as u16;
            IRQ_EP_RX.store(flags, Ordering::Release);
        }

        // TODO: move to another location
        // T::regs().index().write(|w| w.set_index(index as _));
        // if T::regs().txcsrl().read().under_run(){
        //     T::regs().txcsrl().modify(|w| w.set_under_run(false));
        //     warn!("Underrun: ep {}", index);
        // }
    }
}

struct BitIter(u16);

impl Iterator for BitIter {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.trailing_zeros() as u16 {
            16 => None,
            b => {
                self.0 &= !(1 << b);
                Some(b)
            }
        }
    }
}