use super::*;
use crate::alloc_endpoint::{self, EndpointConfig, EndpointData};
use crate::info::ENDPOINTS;

/// MUSB driver.
pub struct MusbDriver<'d, T: MusbInstance> {
    phantom: PhantomData<&'d mut T>,
    alloc: [EndpointData; ENDPOINTS.len()],
    #[cfg(not(feature = "_fixed-fifo-size"))]
    next_fifo_addr_8bytes: u16,
}

impl<'d, T: MusbInstance> MusbDriver<'d, T> {
    /// Create a new USB driver.
    pub fn new() -> Self {
        #[cfg(not(feature = "_fixed-fifo-size"))]
        let next_fifo_addr_8bytes = 8; // Start after EP0's 64 bytes
        
        let regs = T::regs();
        regs.index().write(|w| w.set_index(0));

        // Initialize the bus so that it signals that power is available
        BUS_WAKER.wake();

        Self {
            phantom: PhantomData,
            alloc: [EndpointData {
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
            next_fifo_addr_8bytes,
        }
    }

    pub fn alloc_endpoint<D: Dir>(
        &mut self,
        ep_type: EndpointType,
        max_packet_size: u16,
        interval_ms: u8,
        ep_index: Option<u8>,
    ) -> Result<Endpoint<'d, T, D>, driver::EndpointAllocError> {
        trace!(
            "allocating type={:?} mps={:?} interval_ms={}, dir={:?}",
            ep_type,
            max_packet_size,
            interval_ms,
            D::dir()
        );

        let index = alloc_endpoint::alloc_endpoint(
            &mut self.alloc,
            #[cfg(not(feature = "_fixed-fifo-size"))]
            &mut self.next_fifo_addr_8bytes,
            ep_type,
            ep_index,
            D::dir(),
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
            .alloc_endpoint(EndpointType::Control, control_max_packet_size, 0, Some(0))
            .unwrap();
        let ep_in = self
            .alloc_endpoint(EndpointType::Control, control_max_packet_size, 0, Some(0))
            .unwrap();

        trace!("enabled");

        let mut ep_confs = [EndpointConfig {
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
        }; ENDPOINTS.len()];

        for i in 0..ENDPOINTS.len() {
            ep_confs[i] = self.alloc[i].ep_conf;
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
                ep_out,
                ep_in,
            },
        )
    }
}

// impl<'d, T: MusbInstance> driver::Driver<'d> for Driver<'d, T> {
//     type EndpointOut = Endpoint<'d, T, Out>;
//     type EndpointIn = Endpoint<'d, T, In>;
//     type ControlPipe = ControlPipe<'d, T>;
//     type Bus = Bus<'d, T>;

//     fn alloc_endpoint_in(
//         &mut self,
//         ep_type: EndpointType,
//         max_packet_size: u16,
//         interval_ms: u8,
//     ) -> Result<Self::EndpointIn, driver::EndpointAllocError> {
//         self.alloc_endpoint(ep_type, max_packet_size, interval_ms, false)
//     }

//     fn alloc_endpoint_out(
//         &mut self,
//         ep_type: EndpointType,
//         max_packet_size: u16,
//         interval_ms: u8,
//     ) -> Result<Self::EndpointOut, driver::EndpointAllocError> {
//         self.alloc_endpoint(ep_type, max_packet_size, interval_ms, false)
//     }
// }
