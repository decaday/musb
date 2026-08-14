use super::*;
use crate::alloc_endpoint::{EndpointAllocator, EndpointConfig, EndpointData};
use crate::info::ENDPOINTS;
use crate::assert_eq;

/// MUSB driver.
pub struct MusbDriver<'d, T: MusbInstance> {
    phantom: PhantomData<&'d mut T>,
    allocator: EndpointAllocator,
}

impl<'d, T: MusbInstance> MusbDriver<'d, T> {
    pub fn new() -> Self {
        // Initialize the bus so that it signals that power is available
        BUS_WAKER.wake();

        Self {
            phantom: PhantomData,
            allocator: EndpointAllocator::new(),
        }
    }

    pub fn alloc_endpoint<D: Dir>(
        &mut self,
        ep_type: EndpointType,
        ep_addr: Option<EndpointAddress>,
        max_packet_size: u16,
        interval_ms: u8,
    ) -> Result<Endpoint<'d, T, D>, driver::EndpointAllocError> {
        trace!(
            "musb/alloc_ep: allocating type={:?} mps={:?} interval_ms={}, dir={:?}",
            ep_type,
            max_packet_size,
            interval_ms,
            D::dir()
        );

        if let Some(addr) = ep_addr {
            assert_eq!(addr.direction(), D::dir(), "Wrong addr.direction");
        }

        let internal_ep_type = match ep_type {
            EndpointType::Bulk => crate::types::EndpointType::Bulk,
            EndpointType::Interrupt => crate::types::EndpointType::Interrupt,
            EndpointType::Isochronous => crate::types::EndpointType::Isochronous,
            EndpointType::Control => crate::types::EndpointType::Control,
        };
        let internal_dir = match D::dir() {
            Direction::In => crate::types::Direction::In,
            Direction::Out => crate::types::Direction::Out,
        };

        let index = self.allocator.alloc_endpoint(
            internal_ep_type,
            ep_addr.map(|addr| addr.index() as u8),
            internal_dir,
            max_packet_size,
        )
        .map_err(|_| driver::EndpointAllocError)?;

        Ok(Endpoint {
            _phantom: PhantomData,
            info: EndpointInfo {
                addr: EndpointAddress::from_parts(index as usize, D::dir()),
                ep_type,
                max_packet_size,
                interval_ms,
            },
        })
    }

    pub fn start(
        mut self,
        control_max_packet_size: u16,
    ) -> (crate::Bus<'d, T>, crate::ControlPipe<'d, T>) {
        let ep_out = self
            .alloc_endpoint(EndpointType::Control, Some(0x00.into()), control_max_packet_size, 0)
            .unwrap();
        let ep_in = self
            .alloc_endpoint(EndpointType::Control, Some(0x80.into()), control_max_packet_size, 0)
            .unwrap();

        trace!("musb driver: start");

        let mut ep_confs = [EndpointConfig::default(); ENDPOINTS.len()];

        for i in 0..ENDPOINTS.len() {
            ep_confs[i] = self.allocator.endpoints()[i].ep_conf;
        }

        (
            Bus {
                phantom: PhantomData,
                ep_confs,
                inited: false,
            },
            ControlPipe {
                _phantom: PhantomData,
                max_packet_size: control_max_packet_size,
                _ep_out: ep_out,
                ep_in,
            },
        )
    }
}

impl<'d, T: MusbInstance> driver::Driver<'d> for MusbDriver<'d, T> {
    type EndpointOut = Endpoint<'d, T, Out>;
    type EndpointIn = Endpoint<'d, T, In>;
    type ControlPipe = ControlPipe<'d, T>;
    type Bus = Bus<'d, T>;

    fn alloc_endpoint_in(
        &mut self,
        ep_type: EndpointType,
        ep_addr: Option<driver::EndpointAddress>,
        max_packet_size: u16,
        interval_ms: u8,
    ) -> Result<Self::EndpointIn, driver::EndpointAllocError> {
        self.alloc_endpoint(ep_type, ep_addr, max_packet_size, interval_ms)
    }

    fn alloc_endpoint_out(
        &mut self,
        ep_type: EndpointType,
        ep_addr: Option<driver::EndpointAddress>,
        max_packet_size: u16,
        interval_ms: u8,
    ) -> Result<Self::EndpointOut, driver::EndpointAllocError> {
        self.alloc_endpoint(ep_type, ep_addr, max_packet_size, interval_ms)
    }

    fn start(self, control_max_packet_size: u16) -> (Self::Bus, Self::ControlPipe) {
        self.start(control_max_packet_size)
    }
}
