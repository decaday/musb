use embassy_usb_driver::{Direction, EndpointType};

use crate::info::ENDPOINTS;

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub(crate) struct EndpointData {
    pub(crate) ep_conf: EndpointConfig, // only valid if used_tx || used_rx
    pub(crate) used_tx: bool,
    pub(crate) used_rx: bool,
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub(crate) struct EndpointConfig {
    pub(crate) ep_type: EndpointType,
    pub(crate) tx_max_fifo_size_dword: u16,
    pub(crate) rx_max_fifo_size_dword: u16,
}

pub(crate) enum EndpointAllocError {
    EndpointOverflow,
    InvalidEndpoint,
    #[cfg(not(feature = "_fixed-fifo-size"))]
    BufferOverflow,
}

pub(crate) fn alloc_endpoint(
    alloc: &mut [EndpointData; ENDPOINTS.len()],
    ep_type: EndpointType,
    ep_index: Option<u8>,
    direction: Direction,
    max_packet_size: u16,
) -> Result<u8, EndpointAllocError> {
    let res = if let Some(index) = ep_index {
        if index >= ENDPOINTS.len() as u8 {
            return Err(EndpointAllocError::EndpointOverflow);
        }
        if index == 0 {
            Some((0, &mut alloc[0]))
        } else {
            if check_endpoint(
                &alloc[index as usize],
                ep_type,
                index,
                direction,
                max_packet_size,
            ) {
                Some((index as usize, &mut alloc[index as usize]))
            } else {
                return Err(EndpointAllocError::InvalidEndpoint);
            }
        }
    } else {
        alloc.iter_mut().enumerate().find(|(i, ep)| {
            if *i == 0 {
                return false; // reserved for control pipe
            }
            check_endpoint(ep, ep_type, *i as _, direction, max_packet_size)
        })
    };

    let (index, ep) = match res {
        Some(x) => x,
        None => return Err(EndpointAllocError::EndpointOverflow),
    };

    ep.ep_conf.ep_type = ep_type;

    match direction {
        Direction::Out => {
            assert!(!ep.used_rx);
            ep.used_rx = true;
            ep.ep_conf.rx_max_fifo_size_dword = calc_max_fifo_size_dword(max_packet_size);
        }
        Direction::In => {
            assert!(!ep.used_tx);
            ep.used_tx = true;

            ep.ep_conf.tx_max_fifo_size_dword = calc_max_fifo_size_dword(max_packet_size);
        }
    };

    Ok(index as u8)
}

fn check_endpoint(
    ep: &EndpointData,
    alloc_ep_type: EndpointType,
    index: u8,
    direction: Direction,
    max_packet_size: u16,
) -> bool {
    let used = ep.used_rx || ep.used_tx;
    let _ = index;

    // #[cfg(all(not(feature = "allow-ep-shared-fifo"), feature = "_ep-shared-fifo"))]
    // if used && index != 0 { return false }

    if ((max_packet_size + 7) / 8) as u8 > ENDPOINTS[index as usize].max_packet_size_dword {
        return false;
    }

    if ENDPOINTS[index as usize].ep_direction != crate::info::EpDirection::RXTX {
        match direction {
            Direction::Out => {
                if ENDPOINTS[index as usize].ep_direction != crate::info::EpDirection::RX {
                    return false;
                }
            }
            Direction::In => {
                if ENDPOINTS[index as usize].ep_direction != crate::info::EpDirection::TX {
                    return false;
                }
            }
        }
    }

    if alloc_ep_type == EndpointType::Bulk && used {
        return false;
    }

    let used_dir = match direction {
        Direction::Out => ep.used_rx,
        Direction::In => ep.used_tx,
    };
    !used || (ep.ep_conf.ep_type == alloc_ep_type && !used_dir)
}

fn calc_max_fifo_size_dword(len: u16) -> u16 {
    let dwords = ((len + 7) / 8) as u16;
    if dwords > 8 {
        panic!("Invalid length: {}", len);
    }
    dwords
}
