use embassy_usb_driver::{Direction, EndpointType};

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

#[derive(Debug, Clone, Copy)]
pub struct EndpointData {
    pub ep_conf: EndpointConfig,
    pub used_tx: bool,
    pub used_rx: bool,
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

pub fn alloc_endpoint(
    alloc: &mut [EndpointData; ENDPOINTS.len()],
    #[cfg(not(feature = "_fixed-fifo-size"))] next_fifo_addr_8bytes: &mut u16,
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
            Some((0, &mut alloc[0]))
        } else {
            check_endpoint(
                &alloc[index as usize],
                ep_type,
                index,
                direction,
                max_packet_size,
            )?;
            Some((index as usize, &mut alloc[index as usize]))
        }
    } else {
        alloc.iter_mut().enumerate().find(|(i, ep)| {
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

    ep.ep_conf.ep_type = ep_type;
    
    // --- Dynamic FIFO Allocation Logic ---
    #[cfg(not(feature = "_fixed-fifo-size"))]
    {
        let fifo_size_bytes = max_packet_size.next_power_of_two().max(8) as u16;
        let fifo_size_8bytes = fifo_size_bytes / 8;
        
        let assigned_addr_8bytes = *next_fifo_addr_8bytes;
        
        if ep.ep_conf.ep_type == EndpointType::Control {
            *next_fifo_addr_8bytes += fifo_size_8bytes;
        }

        if *next_fifo_addr_8bytes * 8 > TOTAL_FIFO_SIZE {
            return Err(EndpointAllocError::BufferOverflow);
        }

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
        // For fixed FIFO, we don't calculate or assign, just record the packet size.
        // The sizes and addresses will be retrieved from `crate::generated`.
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

fn check_endpoint(
    ep: &EndpointData,
    alloc_ep_type: EndpointType,
    index: u8,
    direction: Direction,
    max_packet_size: u16,
) -> Result<(), EndpointAllocError> {
    let used = ep.used_rx || ep.used_tx;
    let _ = index;

    // #[cfg(all(not(feature = "allow-ep-shared-fifo"), feature = "_ep-shared-fifo"))]
    // if used && index != 0 { return false }

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

