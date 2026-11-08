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
                true,
            )?;
            Some((index as usize, &mut alloc[index as usize]))
        }
    } else {
        // Pair different types on one endpoint number only when nothing else is
        // free, so endpoints that fit before keep their numbers.
        let fits = |alloc: &[EndpointData], mixed| {
            alloc.iter().enumerate().position(|(i, ep)| {
                // EP0 is reserved for control pipe
                i != 0
                    && check_endpoint(ep, ep_type, i as _, direction, max_packet_size, mixed)
                        .is_ok()
            })
        };
        fits(alloc, false)
            .or_else(|| fits(alloc, true))
            .map(|i| (i, &mut alloc[i]))
    };

    let (index, ep) = match res {
        Some(x) => x,
        None => return Err(EndpointAllocError::EndpointOverflow),
    };

    ep.ep_conf.ep_type = ep_type;
    
    // --- Dynamic FIFO Allocation Logic ---
    #[cfg(not(feature = "_fixed-fifo-size"))]
    if index == 0 {
        assert!(max_packet_size <= 64, "endpoint 0 max packet size must be <= 64");
        match direction {
            // EP0 has fixed FIFO size(64k) and address.
            Direction::Out => {
                ep.ep_conf.rx_max_packet_size = max_packet_size;
                ep.ep_conf.rx_fifo_size_bits = 0;
                ep.ep_conf.rx_fifo_addr_8bytes = 0;
            }
            Direction::In => {
                ep.ep_conf.tx_max_packet_size = max_packet_size;
                ep.ep_conf.tx_fifo_size_bits =0;
                ep.ep_conf.tx_fifo_addr_8bytes = 0;
            }
        }
    }
    else {
        let fifo_size_bytes = max_packet_size.next_power_of_two().max(8) as u16;
        let fifo_size_8bytes = fifo_size_bytes / 8;
        
        let assigned_addr_8bytes = *next_fifo_addr_8bytes;
        
        // Advance the FIFO address for this new allocation
        *next_fifo_addr_8bytes += fifo_size_8bytes;

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
    mixed: bool,
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

    let used_dir = match direction {
        Direction::Out => ep.used_rx,
        Direction::In => ep.used_tx,
    };

    // Without a shared FIFO, IN and OUT of one endpoint number have their own
    // FIFO and CSRs, so they can be of different types. Only the ISO bit comes
    // from the single `ep_type` kept per number.
    #[cfg(not(feature = "_ep-shared-fifo"))]
    if mixed && used && !used_dir {
        let iso = EndpointType::Isochronous;
        if ep.ep_conf.ep_type != iso && alloc_ep_type != iso {
            return Ok(());
        }
    }
    let _ = mixed;

    if alloc_ep_type == EndpointType::Bulk && used {
        return Err(EndpointAllocError::EpUsed);
    }
    
    if !used || (ep.ep_conf.ep_type == alloc_ep_type && !used_dir) {
        Ok(())
    } else {
        Err(EndpointAllocError::EpUsed)
    }
}

