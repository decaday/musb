/// `usb-device` implementation.
// Although MUSB's control transfer has a state machine, there are no
// corresponding registers. This makes it impossible to determine if
// a packet is a Setup packet without knowing the state.
//
// The state of the `usb-device`'s control transfer state machine
// cannot be accessed at the porting layer, so we need to manually
// determine whether to set the data_end bit.
//
// `usb-device` sends a zero-length packet after receiving data in
// control transfer, while MUSB doesn't need this behavior, we still
// need to tell `usb-device` that this packet was sent successfully.
// (see ControlStateEnum::Accepted)

// Therefore, this porting layer maintains its own state machine:
// `UsbdBus.control_state`.
use core::marker::PhantomData;
use portable_atomic::{AtomicBool, AtomicU16, Ordering};

use crate::types::{Direction, EndpointType};
use usb_device::bus::PollResult;
use usb_device::{UsbDirection, UsbError};

use crate::alloc_endpoint::{self, EndpointAllocError, EndpointConfig, EndpointData};
use crate::{bus, endpoint, endpoint0};
use crate::{trace, warn};
use crate::MusbInstance;
use crate::info::ENDPOINTS;

mod control_state;
use control_state::{ControlState, ControlStateEnum};



pub struct UsbdBus<T: MusbInstance> {
    phantom: PhantomData<T>,
    endpoints: [EndpointData; ENDPOINTS.len()],
    #[cfg(not(feature = "_fixed-fifo-size"))]
    next_fifo_addr_8bytes: u16,
    control_state: ControlState,
}

impl<T: MusbInstance> UsbdBus<T> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
            endpoints: [EndpointData {
                ep_conf: EndpointConfig {
                    ep_type: EndpointType::Bulk,
                    tx_max_packet_size: 0,
                    rx_max_packet_size: 0,
                    #[cfg(not(feature = "_fixed-fifo-size"))]
                    tx_fifo_size_bits: 0,
                    #[cfg(not(feature = "_fixed-fifo-size"))]
                    rx_fifo_size_bits: 0,
                    #[cfg(not(feature = "_fixed-fifo-size"))]
                    tx_fifo_addr_8bytes: 0,
                    #[cfg(not(feature = "_fixed-fifo-size"))]
                    rx_fifo_addr_8bytes: 0,
                },
                used_tx: false,
                used_rx: false,
            }; ENDPOINTS.len()],
            #[cfg(not(feature = "_fixed-fifo-size"))]
            next_fifo_addr_8bytes: 8, // 64 bytes for EP0
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
        let index = ep_addr.map(|addr| addr.index() as u8);
        let ep_type = match ep_type {
            usb_device::endpoint::EndpointType::Bulk => EndpointType::Bulk,
            usb_device::endpoint::EndpointType::Interrupt => EndpointType::Interrupt,
            usb_device::endpoint::EndpointType::Isochronous { .. } => EndpointType::Isochronous,
            usb_device::endpoint::EndpointType::Control => EndpointType::Control,
        };
        let dir = match ep_dir {
            usb_device::UsbDirection::In => Direction::In,
            usb_device::UsbDirection::Out => Direction::Out,
        };

        alloc_endpoint::alloc_endpoint(
            &mut self.endpoints,
            #[cfg(not(feature = "_fixed-fifo-size"))]
            &mut self.next_fifo_addr_8bytes,
            ep_type,
            index,
            dir,
            max_packet_size,
        )
            .map_err(|e| match e {
                EndpointAllocError::EndpointOverflow => UsbError::EndpointOverflow,
                EndpointAllocError::InvalidEndpoint => UsbError::InvalidEndpoint,
                #[cfg(not(feature = "_fixed-fifo-size"))]
                EndpointAllocError::BufferOverflow => UsbError::EndpointMemoryOverflow,
                EndpointAllocError::EpDirNotSupported => UsbError::InvalidEndpoint,
                EndpointAllocError::EpUsed => UsbError::InvalidEndpoint,
                EndpointAllocError::MaxPacketSizeBiggerThanEpFifoSize => UsbError::EndpointMemoryOverflow,
            })
            .map(|index| usb_device::endpoint::EndpointAddress::from_parts(index as usize, ep_dir))
    }

    fn enable(&mut self) {
        trace!("call enable");
        bus::bus_set_device_address::<T>(0);
    }

    fn reset(&self) {
        trace!("call reset");
        T::regs().power().write(|w| w.set_suspend_mode(true));

        self.endpoints.iter().enumerate().for_each(|(index, ep)| {
            if ep.used_tx {
                trace!("call ep_tx_enable, index = {}", index);
                endpoint::ep_tx_enable::<T>(index as _, &ep.ep_conf);
            }
            if ep.used_rx {
                trace!("call ep_rx_enable, index = {}", index);
                endpoint::ep_rx_enable::<T>(index as _, &ep.ep_conf);
            }
        });

        self.control_state.set_state(ControlStateEnum::Idle);
        self.control_state.reset_tx_len();
    }

    fn set_device_address(&self, addr: u8) {
        trace!("call set_device_address: {}", addr);
        bus::bus_set_device_address::<T>(addr);
    }

    fn write(
        &self,
        ep_addr: usb_device::endpoint::EndpointAddress,
        buf: &[u8],
    ) -> usb_device::Result<usize> {
        let index = ep_addr.index();
        trace!(
            "WRITE len = {}, index = {} ,control state = {:?}",
            buf.len(),
            index,
            self.control_state.get_state()
        );

        // if buf.len() > self.endpoints[index].ep_conf.tx_max_packet_size as usize {
        //     return Err(UsbError::BufferOverflow);
        // }
        let unready = if index == 0 {
            endpoint0::ep0_is_tx_busy::<T>()
        } else {
            endpoint::ep_is_tx_busy::<T>(index as _)
        };
        if unready {
            return Err(UsbError::WouldBlock);
        }

        if index == 0 {
            match self.control_state.get_state() {
                ControlStateEnum::NodataPhase => {
                    if buf.len() != 0 {
                        panic!("NodataPhase, write buf.len() != 0");
                    }
                    trace!("NodataPhase, buf.len() = 0");
                    self.control_state.set_state(ControlStateEnum::Idle);
                    IRQ_EP_TX.fetch_or(1, Ordering::Release);
                }
                ControlStateEnum::DataIn => {
                    self.control_state.decrease_tx_len(buf.len() as u32);
                    let mut is_last = false;
                    if self.control_state.get_tx_len() == 0 {
                        is_last = true;
                        self.control_state.set_state(ControlStateEnum::Idle);
                    } else if buf.len()
                        < self.endpoints[0].ep_conf.tx_max_packet_size as usize
                    {
                        // Last Package. include ZLP
                        is_last = true;
                        self.control_state.set_state(ControlStateEnum::Idle);
                        self.control_state.reset_tx_len();
                    }
                    endpoint0::ep0_write_packet_unchecked::<T>(buf, is_last);
                }
                ControlStateEnum::Idle => {
                    if buf.len() != 0 {
                        panic!("Idle, but write buf.len() != 0");
                    }
                    // In complete
                    self.control_state.set_state(ControlStateEnum::Accepted);
                }
                _ => {
                    panic!(
                        "Writing, Invalid state: {:?}",
                        self.control_state.get_state()
                    );
                }
            }
        } else {
            if buf.len() != 0 {
                endpoint::ep_write_packet_unchecked::<T>(index as _, buf);
            } else {
                T::regs().index().write(|w| w.set_index(index as _));
                T::regs().txcsrl().modify(|w| w.set_tx_pkt_rdy(true));
            }
        }
        trace!("WRITE OK");
        Ok(buf.len())
    }

    fn read(
        &self,
        ep_addr: usb_device::endpoint::EndpointAddress,
        buf: &mut [u8],
    ) -> usb_device::Result<usize> {
        let index = ep_addr.index();
        trace!("READ, buf.len() = {}, index = {}", buf.len(), index);

        let unready = if index == 0 {
            !endpoint0::ep0_is_rx_ready::<T>()
        } else {
            !endpoint::ep_is_rx_ready::<T>(index as _)
        };
        if unready {
            // trace!("unready");
            return Err(UsbError::WouldBlock);
        }

        let read_count = if index == 0 {
            endpoint0::ep0_read_packet_unchecked::<T>(buf)
        } else {
            endpoint::ep_read_packet_unchecked::<T>(index as _, buf)
        };
        
        if index == 0 {
            match self.control_state.get_state() {
                ControlStateEnum::Setup => {
                    assert!(read_count == 8);
                    let direction = buf[0] & 0x80;
                    let w_length = buf[6] as u16 | (buf[7] as u16) << 8;
                    if direction == 0 {
                        // OUT
                        if w_length == 0 {
                            endpoint0::ep0_set_data_end::<T>();
                            self.control_state.set_state(ControlStateEnum::NodataPhase);
                        } else {
                            self.control_state.set_state(ControlStateEnum::DataOut);
                            // self.control_state.set_rx_len(w_length as _);
                        }
                    } else {
                        // IN
                        if w_length == 0 {
                            endpoint0::ep0_set_data_end::<T>();
                            self.control_state.set_state(ControlStateEnum::NodataPhase);
                        } else {
                            self.control_state.set_state(ControlStateEnum::DataIn);
                            self.control_state.set_tx_len(w_length as _);
                        }
                    }
                }
                ControlStateEnum::DataOut => {
                    if (read_count as u32)
                        < self.endpoints[0].ep_conf.rx_max_packet_size as u32
                    {
                        // Last Package. include ZLP
                        endpoint0::ep0_set_data_end::<T>();
                        self.control_state.set_state(ControlStateEnum::Idle);
                        trace!("READ END, buf.len() = {}", buf.len());
                    }
                }
                _ => {
                    panic!(
                        "Unknown control state when reading: {:?}",
                        self.control_state.get_state()
                    );
                }
            }
        }

        trace!("READ OK, rx_len = {}", read_count);

        Ok(read_count as usize)
    }

    fn set_stalled(&self, ep_addr: usb_device::endpoint::EndpointAddress, stalled: bool) {
        let index = ep_addr.index();
        match ep_addr.direction() {
            UsbDirection::In => endpoint::ep_tx_stall::<T>(index as _, stalled),
            UsbDirection::Out => endpoint::ep_rx_stall::<T>(index as _, stalled),
        }
        if index == 0 {
            if stalled {
                self.control_state.set_state(ControlStateEnum::Idle);
                self.control_state.reset_tx_len();
            }
        }
    }

    fn is_stalled(&self, ep_addr: usb_device::endpoint::EndpointAddress) -> bool {
        match ep_addr.direction() {
            UsbDirection::In => endpoint::ep_tx_is_stalled::<T>(ep_addr.index() as _),
            UsbDirection::Out => endpoint::ep_rx_is_stalled::<T>(ep_addr.index() as _),
        }
    }

    fn suspend(&self) {}

    fn resume(&self) {}

    fn poll(&self) -> PollResult {
        let regs = T::regs();
        let mut setup = false;

        endpoint::check_overrun::<T>();

        if IRQ_RESET.load(Ordering::Acquire) {
            IRQ_RESET.store(false, Ordering::Relaxed);
            return PollResult::Reset;
        }
        if IRQ_RESUME.load(Ordering::Acquire) {
            IRQ_RESUME.store(false, Ordering::Relaxed);
            return PollResult::Resume;
        }
        if IRQ_SUSPEND.load(Ordering::Acquire) {
            IRQ_SUSPEND.store(false, Ordering::Relaxed);
            return PollResult::Suspend;
        }

        if IRQ_EP0.load(Ordering::Acquire) {
            let rx_pkt_rdy = endpoint0::ep0_is_rx_ready::<T>();
            let tx_pkt_rdy = endpoint0::ep0_is_tx_busy::<T>();

            match (rx_pkt_rdy, tx_pkt_rdy) {
                (false, false) => {
                    IRQ_EP0.store(false, Ordering::Release);
                    match self.control_state.get_state() {
                        ControlStateEnum::DataIn => {
                            // interrupt generated due to a packet has been transmitted
                            IRQ_EP_TX.fetch_or(1u16, Ordering::Release);
                        }
                        _ => {}
                    }
                }
                (true, _) => {
                    IRQ_EP0.store(false, Ordering::Release);
                    let count = regs.count0().read().count();

                    match self.control_state.get_state() {
                        ControlStateEnum::Idle => match count {
                            8 => {
                                self.control_state.set_state(ControlStateEnum::Setup);
                                endpoint0::ep0_clear_setup_end::<T>();
                                setup = true;
                            }
                            _ => {
                                warn!("setup packet not 8 bytes long, count = {}", count);
                            }
                        },
                        ControlStateEnum::DataOut => {
                            IRQ_EP_RX.fetch_or(1u16, Ordering::Release);
                        }
                        ControlStateEnum::Accepted => {}
                        ControlStateEnum::DataIn => {
                            if endpoint0::ep0_is_setup_end::<T>() {
                                warn!("setup end, count = {}", count);
                                endpoint0::ep0_clear_setup_end::<T>();
                                self.control_state.set_state(ControlStateEnum::Idle);
                                self.control_state.reset_tx_len();

                                match count {
                                    8 => {
                                        self.control_state.set_state(ControlStateEnum::Setup);
                                        setup = true;
                                    }
                                    _ => {
                                        warn!("setup packet not 8 bytes long, count = {}", count);
                                    }
                                }
                            } else {
                                warn!(
                                    "Unknown control state when reading: {:?}",
                                    self.control_state.get_state()
                                );
                            }
                        }
                        _ => {
                            warn!(
                                "Unknown control state when reading: {:?}",
                                self.control_state.get_state()
                            );
                        }
                    }
                }
                (false, true) => {
                    IRQ_EP0.store(false, Ordering::Release);

                    // let flags = IRQ_EP_TX.load(Ordering::Acquire) | 1u16;
                    // IRQ_EP_TX.store(flags, Ordering::Release);
                }
            }
        }

        if self.control_state.get_state() == ControlStateEnum::Accepted {
            // // Ignore RX. This will be addressed in the next poll.
            let flags = IRQ_EP_RX.load(Ordering::Acquire);
            if flags & 1u16 != 0 {
                trace!("Accepted with IRQ_EP_RX != 0");
                IRQ_EP0.store(true, Ordering::Release);
                IRQ_EP_RX.fetch_and(!1u16, Ordering::Release);
            }

            IRQ_EP_TX.fetch_or(1u16, Ordering::Release);

            self.control_state.set_state(ControlStateEnum::Idle);
        }

        let rx_flags = IRQ_EP_RX.load(Ordering::Acquire);
        for index in BitIter(rx_flags) {
            let rdy = if index == 0 {
                endpoint0::ep0_is_rx_ready::<T>()
            } else {
                endpoint::ep_is_rx_ready::<T>(index as _)
            };
            // clean flags after packet was read, rx_pkt_rdy == false
            if !rdy {
                IRQ_EP_RX.fetch_and(!((1 << index) as u16), Ordering::Release);
            }
        }
        let in_complete = IRQ_EP_TX.swap(0, Ordering::Acquire);
        let out = IRQ_EP_RX.load(Ordering::Acquire);

        if in_complete != 0 || out != 0 || setup {
            PollResult::Data {
                ep_out: out,
                ep_in_complete: in_complete,
                ep_setup: if setup { 1 } else { 0 },
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
    let usb_status = crate::bus::read_and_clear_usb_interrupts::<T>();
    if usb_status.reset {
        IRQ_RESET.store(true, Ordering::Release);
    }
    if usb_status.suspend {
        IRQ_SUSPEND.store(true, Ordering::Release);
    }
    if usb_status.resume {
        IRQ_RESUME.store(true, Ordering::Release);
    }

    let tx_status = crate::endpoint::read_and_clear_tx_interrupts::<T>();
    let rx_status = crate::endpoint::read_and_clear_rx_interrupts::<T>();

    if (tx_status & 1) != 0 {
        IRQ_EP0.store(true, Ordering::Release);
    }

    for index in 1..ENDPOINTS.len() {
        if (tx_status & (1 << index)) != 0 {
            IRQ_EP_TX.fetch_or((1 << index) as u16, Ordering::Release);
        }
        if (rx_status & (1 << index)) != 0 {
            IRQ_EP_RX.fetch_or((1 << index) as u16, Ordering::Release);
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
