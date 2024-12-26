/// `usb-device` implementation.

// In EP0 control transfers, the DataEnd bit needs to be set when 
// reading or writing the last data packet. However, usb-device 
// doesn't provide us with this state information.
// Additionally, musb doesn't have dedicated status registers for 
// Setup packets - it only has RxPktRdy to indicate packet reception.
// Therefore, in this implementation, I used a state machine to handle
// control transfers. However, there are still some issues, and the operation 
// is not stable at present.

use core::marker::PhantomData;
use core::sync::atomic::{AtomicBool, AtomicU16, AtomicU32, AtomicU8, Ordering};

use embassy_usb_driver::{Direction, EndpointType};
use usb_device::bus::PollResult;
use usb_device::{UsbDirection, UsbError};

use crate::common_impl;
use crate::{trace, warn};
use crate::{alloc_endpoint, MusbInstance, ENDPOINTS_NUM};
use crate::alloc_endpoint::{EndpointAllocError, EndpointConfig, EndpointData};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, PartialEq)]
enum ControlStateEnum {
    Idle,
    Setup,
    DataIn,
    DataOut,
    NodataPhase,
    // Error,
}

impl From<u8> for ControlStateEnum {
    fn from(value: u8) -> Self {
        match value {
            0 => ControlStateEnum::Idle,
            1 => ControlStateEnum::Setup,
            2 => ControlStateEnum::DataIn,
            3 => ControlStateEnum::DataOut,
            4 => ControlStateEnum::NodataPhase,
            _ => unreachable!(),
        }
    }
}

struct ControlState {
    state: AtomicU8,
    remain: AtomicU32,
}

impl ControlState {
    const fn new() -> Self {
        Self {
            state: AtomicU8::new(ControlStateEnum::Idle as u8),
            remain: AtomicU32::new(0),
        }
    }

    fn set_state(&self, state: ControlStateEnum) {
        self.state.store(state as u8, Ordering::SeqCst);
    }

    fn get_state(&self) -> ControlStateEnum {
        ControlStateEnum::from(self.state.load(Ordering::SeqCst))
    }

    fn set_remain(&self, remain: u32) {
        self.remain.store(remain, Ordering::SeqCst);
    }

    fn get_remain(&self) -> u32 {
        self.remain.load(Ordering::SeqCst)
    }
}

pub struct UsbdBus<T: MusbInstance> {
    phantom: PhantomData<T>,
    endpoints: [EndpointData; ENDPOINTS_NUM],
    control_state:ControlState,
}

impl<T: MusbInstance> UsbdBus<T> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
            endpoints: [EndpointData {
                ep_conf: EndpointConfig {
                    ep_type: EndpointType::Bulk,
                    tx_max_fifo_size_dword: 1,
                    rx_max_fifo_size_dword: 1,
                },
                used_tx: false,
                used_rx: false,
            }; ENDPOINTS_NUM],
            control_state: ControlState::new(),
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

        alloc_endpoint::alloc_endpoint(&mut self.endpoints, ep_type, index, dir, max_packet_size)
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

        self.endpoints.iter().enumerate().for_each(|(index, ep)| {
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

        // if buf.len() > self.endpoints[index].ep_conf.tx_max_fifo_size_dword as usize * 8 {
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

        if buf.len() != 0 {
            buf.into_iter().for_each(|b|
                regs.fifo(index).write(|w| w.set_data(*b))
            );
        }
        
        if index == 0 {
            match self.control_state.get_state() {
                ControlStateEnum::NodataPhase => {
                    if buf.len() != 0 {
                        panic!("NodataPhase, write buf.len() != 0");
                    }
                    trace!("NodataPhase, buf.len() = 0");
                    self.control_state.set_state(ControlStateEnum::Idle);
                    let flags = IRQ_EP_TX.load(Ordering::Acquire) | 1 as u16;
                    IRQ_EP_TX.store(flags, Ordering::Release);
                },
                ControlStateEnum::DataIn => {
                    regs.csr0l().modify(|w| w.set_tx_pkt_rdy(true));
                    if buf.len() < self.endpoints[0].ep_conf.tx_max_fifo_size_dword as usize * 8 {
                        // Last Package. include ZLP
                        regs.csr0l().modify(|w| w.set_data_end(true));
                        self.control_state.set_state(ControlStateEnum::Idle);
                        trace!("WRITE END, buf.len() = {}", buf.len());
                    }
                },
                ControlStateEnum::DataOut => {
                    if buf.len() != 0 {
                        panic!("DataOut, but write buf.len() != 0");
                    }
                }
                _ => {
                    panic!("Writing, Invalid state: {:?}", self.control_state.get_state());
                }
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
        buf.into_iter().take(read_count as _).for_each(|b| 
            *b = regs.fifo(index).read().data()
        );
        if index == 0 {
            regs.csr0l().modify(|w| w.set_serviced_rx_pkt_rdy(true));
            match self.control_state.get_state() {
                ControlStateEnum::Setup => {
                    assert!(read_count == 8);
                    let direction = buf[0] & 0x80;
                    let w_length = buf[6];
                    if direction == 0 { // OUT
                        if w_length == 0 {
                            regs.csr0l().modify(|w| w.set_data_end(true));
                            self.control_state.set_state(ControlStateEnum::NodataPhase);
                        } else {
                            self.control_state.set_state(ControlStateEnum::DataOut);
                            // self.control_state.set_remain(read_count as _);
                        }
                    } else { // IN
                        if w_length == 0 {
                            regs.csr0l().modify(|w| w.set_data_end(true));
                            self.control_state.set_state(ControlStateEnum::NodataPhase);
                        } else {
                            self.control_state.set_state(ControlStateEnum::DataIn);
                        }
                    }
                },
                ControlStateEnum::DataOut => {
                    if buf.len() < self.endpoints[0].ep_conf.rx_max_fifo_size_dword as usize * 8 {
                        // Last Package. include ZLP
                        regs.csr0l().modify(|w| w.set_data_end(true));
                        self.control_state.set_state(ControlStateEnum::Idle);
                    }
                },
                _ => {
                    panic!("Unknown control state when reading: {:?}", self.control_state.get_state());
                }
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
        if index == 0 {
            if stalled {
                self.control_state.set_state(ControlStateEnum::Idle);
            }
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

        common_impl::check_underrun::<T>();

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
                    let count = regs.count0().read().count();

                    match self.control_state.get_state() {
                        ControlStateEnum::Idle => {
                            match count {
                                8 => {
                                    self.control_state.set_state(ControlStateEnum::Setup);
                                    setup = true;
                                }
                                _ => {
                                    warn!("setup packet not 8 bytes long, count = {}", count);
                                }
                            }
                        },
                        ControlStateEnum::DataOut => {
                            let flags = IRQ_EP_RX.load(Ordering::Acquire) | 1u16;
                            IRQ_EP_RX.store(flags, Ordering::Release);
                        }
                        _ => {
                            warn!("Unknown control state when reading: {:?}", self.control_state.get_state());
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
    trace!("intrtx: {:b}, intrrx: {:b}, intrtxe: {:b}", intrtx.0, intrrx.0, intrtxe.0);

    for index in 1..ENDPOINTS_NUM {
        if intrtx.ep_tx(index) {
            let flags = IRQ_EP_TX.load(Ordering::Acquire) | ( 1 << index ) as u16;
            IRQ_EP_TX.store(flags, Ordering::Release);
        }
        if intrrx.ep_rx(index) {             
            let flags = IRQ_EP_RX.load(Ordering::Acquire) | ( 1 << index ) as u16;
            IRQ_EP_RX.store(flags, Ordering::Release);
        }
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