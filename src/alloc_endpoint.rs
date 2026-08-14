use crate::types::{Direction, EndpointType};

use crate::info::ENDPOINTS;
#[cfg(not(feature = "_fixed-fifo-size"))]
use crate::info::TOTAL_FIFO_SIZE;

#[derive(Debug, Clone, Copy)]
pub struct EndpointConfig {
    pub ep_type: EndpointType,
    pub tx_max_packet_size: u16,
    pub rx_max_packet_size: u16,
    
    #[cfg(not(feature = "_fixed-fifo-size"))]
    pub tx_fifo_size_bits: u8,
    #[cfg(not(feature = "_fixed-fifo-size"))]
    pub rx_fifo_size_bits: u8,

    #[cfg(not(feature = "_fixed-fifo-size"))]
    pub tx_fifo_addr_8bytes: u16,
    #[cfg(not(feature = "_fixed-fifo-size"))]
    pub rx_fifo_addr_8bytes: u16,
}

impl Default for EndpointConfig {
    fn default() -> Self {
        Self {
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
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct EndpointData {
    pub ep_conf: EndpointConfig,
    pub used_tx: bool,
    pub used_rx: bool,
}

pub struct EndpointAllocator {
    alloc: [EndpointData; ENDPOINTS.len()],
    #[cfg(not(feature = "_fixed-fifo-size"))]
    next_fifo_addr_8bytes: u16,
}

impl EndpointAllocator {
    pub fn new() -> Self {
        Self {
            alloc: [EndpointData::default(); ENDPOINTS.len()],
            #[cfg(not(feature = "_fixed-fifo-size"))]
            next_fifo_addr_8bytes: 8, // Start after EP0's 64 bytes
        }
    }

    pub fn endpoints(&self) -> &[EndpointData] {
        &self.alloc
    }

    pub fn endpoints_mut(&mut self) -> &mut [EndpointData] {
        &mut self.alloc
    }

    pub fn alloc_endpoint(
        &mut self,
        ep_type: EndpointType,
        ep_index: Option<u8>,
        direction: Direction,
        max_packet_size: u16,
    ) -> Result<u8, EndpointAllocError> {
        let res = if let Some(index) = ep_index {
            if index >= ENDPOINTS.len() as u8 {
                return Err(EndpointAllocError::InvalidEndpoint);
            }
            if index == 0 {
                Some((0, &mut self.alloc[0]))
            } else {
                check_endpoint(
                    &self.alloc[index as usize],
                    ep_type,
                    index,
                    direction,
                    max_packet_size,
                )?;
                Some((index as usize, &mut self.alloc[index as usize]))
            }
        } else {
            self.alloc.iter_mut().enumerate().find(|(i, ep)| {
                if *i == 0 {
                    return false; // reserved for control pipe
                }
                check_endpoint(ep, ep_type, *i as _, direction, max_packet_size).is_ok()
            })
        };

        let (index, ep) = match res {
            Some(x) => x,
            None => return Err(EndpointAllocError::EndpointOverflow),
        };

        // --- Dynamic FIFO Allocation Logic ---
        #[cfg(not(feature = "_fixed-fifo-size"))]
        if index == 0 {
            assert!(max_packet_size <= 64, "endpoint 0 max packet size must be <= 64");
            ep.ep_conf.ep_type = ep_type;
            match direction {
                // EP0 has fixed FIFO size(64k) and address.
                Direction::Out => {
                    ep.ep_conf.rx_max_packet_size = max_packet_size;
                    ep.ep_conf.rx_fifo_size_bits = 0;
                    ep.ep_conf.rx_fifo_addr_8bytes = 0;
                }
                Direction::In => {
                    ep.ep_conf.tx_max_packet_size = max_packet_size;
                    ep.ep_conf.tx_fifo_size_bits = 0;
                    ep.ep_conf.tx_fifo_addr_8bytes = 0;
                }
            }
        } else {
            let fifo_size_bytes = max_packet_size.next_power_of_two().max(8) as u16;
            let fifo_size_8bytes = fifo_size_bytes / 8;
            
            let next_addr = self.next_fifo_addr_8bytes + fifo_size_8bytes;
            if next_addr * 8 > TOTAL_FIFO_SIZE {
                return Err(EndpointAllocError::BufferOverflow);
            }
            let assigned_addr_8bytes = self.next_fifo_addr_8bytes;
            self.next_fifo_addr_8bytes = next_addr;

            ep.ep_conf.ep_type = ep_type;
            match direction {
                Direction::Out => {
                    ep.ep_conf.rx_max_packet_size = max_packet_size;
                    ep.ep_conf.rx_fifo_size_bits = fifo_size_bytes.trailing_zeros() as u8;
                    ep.ep_conf.rx_fifo_addr_8bytes = assigned_addr_8bytes;
                }
                Direction::In => {
                    ep.ep_conf.tx_max_packet_size = max_packet_size;
                    ep.ep_conf.tx_fifo_size_bits = fifo_size_bytes.trailing_zeros() as u8;
                    ep.ep_conf.tx_fifo_addr_8bytes = assigned_addr_8bytes;
                }
            }
        }
        
        #[cfg(feature = "_fixed-fifo-size")]
        {
            ep.ep_conf.ep_type = ep_type;
            match direction {
                Direction::Out => ep.ep_conf.rx_max_packet_size = max_packet_size,
                Direction::In => ep.ep_conf.tx_max_packet_size = max_packet_size,
            }
        }

        match direction {
            Direction::Out => ep.used_rx = true,
            Direction::In => ep.used_tx = true,
        };

        Ok(index as u8)
    }
}

pub enum EndpointAllocError {
    EndpointOverflow,
    InvalidEndpoint,
    EpDirNotSupported,
    EpUsed,
    MaxPacketSizeBiggerThanEpFifoSize,
    #[cfg(not(feature = "_fixed-fifo-size"))]
    BufferOverflow,
}

// alloc_endpoint function is now moved inside EndpointAllocator

fn check_endpoint(
    ep: &EndpointData,
    alloc_ep_type: EndpointType,
    index: u8,
    direction: Direction,
    max_packet_size: u16,
) -> Result<(), EndpointAllocError> {
    let used = ep.used_rx || ep.used_tx;
    let _ = index;

    #[cfg(feature = "_ep-shared-fifo")]
    if used && index != 0 { return Err(EndpointAllocError::EpUsed) }

    if max_packet_size > ENDPOINTS[index as usize].max_packet_size {
        return Err(EndpointAllocError::MaxPacketSizeBiggerThanEpFifoSize);
    }

    if ENDPOINTS[index as usize].ep_direction != crate::info::EpDirection::RXTX {
        match direction {
            Direction::Out => {
                if ENDPOINTS[index as usize].ep_direction != crate::info::EpDirection::RX {
                    return Err(EndpointAllocError::EpDirNotSupported);
                }
            }
            Direction::In => {
                if ENDPOINTS[index as usize].ep_direction != crate::info::EpDirection::TX {
                    return Err(EndpointAllocError::EpDirNotSupported);
                }
            }
        }
    }

    if alloc_ep_type == EndpointType::Bulk && used {
        return Err(EndpointAllocError::EpUsed);
    }

    let used_dir = match direction {
        Direction::Out => ep.used_rx,
        Direction::In => ep.used_tx,
    };
    
    if !used || (ep.ep_conf.ep_type == alloc_ep_type && !used_dir) {
        Ok(())
    } else {
        Err(EndpointAllocError::EpUsed)
    }
}

