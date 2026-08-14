use crate::alloc_endpoint::EndpointConfig;
use crate::types::EndpointType;
#[cfg(feature = "_ep-shared-fifo")]
use crate::regs::vals::EndpointDirection;
use crate::{trace, warn, MusbInstance};
use crate::info::ENDPOINTS;

/// Read and clear TX interrupts.
/// bit 0 indicates an interrupt for EP0 in either direction.
#[inline]
pub fn read_and_clear_tx_interrupts<T: MusbInstance>() -> u16 {
    T::regs().intrtx().read().0.into()
}

#[inline]
pub fn read_and_clear_rx_interrupts<T: MusbInstance>() -> u16 {
    T::regs().intrrx().read().0.into()
}

/// Stall TX endpoint
/// `index` must not be 0.
#[inline]
pub fn ep_tx_stall<T: MusbInstance>(index: u8, stalled: bool) {
    let regs = T::regs();
    regs.index().write(|w| w.set_index(index as _));
    
    regs.txcsrl().write(|w| {
        w.set_send_stall(stalled);
        if !stalled {
            w.set_sent_stall(false);
            w.set_clr_data_tog(true);
        }
    });
}

/// Stall RX endpoint
/// `index` must not be 0.
#[inline]
pub fn ep_rx_stall<T: MusbInstance>(index: u8, stalled: bool) {
    let regs = T::regs();
    regs.index().write(|w| w.set_index(index as _));

    regs.rxcsrl().write(|w| {
        w.set_send_stall(stalled);
        if !stalled {
            w.set_sent_stall(false);
            w.set_clr_data_tog(true);
        }
    });
}

/// Check if RX endpoint is stalled
/// `index` must not be 0.
#[inline]
pub fn ep_rx_is_stalled<T: MusbInstance>(index: u8) -> bool {
    let regs = T::regs();
    regs.index().write(|w| w.set_index(index as _));
    regs.rxcsrl().read().send_stall()
}

/// Check if TX endpoint is stalled
/// `index` must not be 0.
#[inline]
pub fn ep_tx_is_stalled<T: MusbInstance>(index: u8) -> bool {
    let regs = T::regs();
    regs.index().write(|w| w.set_index(index as _));
    regs.txcsrl().read().send_stall()
}

/// Enable and configure TX endpoint
/// `index` must not be 0.
#[inline]
pub fn ep_tx_enable<T: MusbInstance>(index: u8, config: &EndpointConfig) {
    #[cfg(not(feature="_fixed-fifo-size"))]
    trace!(
        "musb/ep_enable: Enabling TX endpoint {}: max_packet_size={}, fifo_size_bits={}, fifo_addr_8bytes={}, ep_type={:?}",
        index, config.tx_max_packet_size, config.tx_fifo_size_bits, config.tx_fifo_addr_8bytes, config.ep_type
    );
    #[cfg(feature = "_fixed-fifo-size")]
    trace!(
        "musb/ep_enable: Enabling TX endpoint {}: max_packet_size={}, ep_type={:?}",
        index, config.tx_max_packet_size, config.ep_type
    );

    let regs = T::regs();
    regs.index().write(|w| w.set_index(index));

    regs.intrtxe().modify(|w| w.set_ep_txe(index as _, true));

    #[cfg(not(feature = "_fixed-fifo-size"))]
    {
        regs.tx_fifo_sz().write(|w| {
            let size_code = (config.tx_fifo_size_bits - 3) as u8;
            w.set_sz(size_code);
        });
        regs.tx_fifo_add().write(|w| w.set_add(config.tx_fifo_addr_8bytes));
    }

    cfg_if::cfg_if! {
        if #[cfg(feature = "_lite")] {
            if config.tx_max_packet_size % 8 != 0 {
                warn!("TX max packet size must be a multiple of 8 for lite musb IP, using {} instead",
                    ((config.tx_max_packet_size + 7) / 8) * 8
                );
            }
            regs.txmaxp().write(|w| w.set_maxp((config.tx_max_packet_size + 7) / 8));
        } else {
            regs.txmaxp().write(|w| w.set_maxp(config.tx_max_packet_size));
        }
    }

    regs.txcsrl().modify(|w| {
        w.set_clr_data_tog(true);
    });

    if config.ep_type == EndpointType::Isochronous {
        regs.txcsrh().write(|w| {
            w.set_iso(true);
        });
    }

    #[cfg(feature = "_ep-shared-fifo")]
    regs.txcsrh().write(|w| w.set_mode(EndpointDirection::Tx));

    if regs.txcsrl().read().fifo_not_empty() {
        regs.txcsrl().modify(|w| w.set_flush_fifo(true));
        regs.txcsrl().modify(|w| w.set_flush_fifo(true));
    }
}

/// Enable and configure RX endpoint
/// `index` must not be 0.
#[inline]
pub fn ep_rx_enable<T: MusbInstance>(index: u8, config: &EndpointConfig) {
    #[cfg(not(feature="_fixed-fifo-size"))]
    trace!(
        "musb/ep_enable: Enabling RX endpoint {}: max_packet_size={}, fifo_size_bits={}, fifo_addr_8bytes={}, ep_type={:?}",
        index, config.rx_max_packet_size, config.rx_fifo_size_bits, config.rx_fifo_addr_8bytes, config.ep_type
    );
    #[cfg(feature = "_fixed-fifo-size")]
    trace!(
        "musb/ep_enable: Enabling RX endpoint {}: max_packet_size={}, ep_type={:?}",
        index, config.rx_max_packet_size, config.ep_type
    );

    let regs = T::regs();
    regs.index().write(|w| w.set_index(index));

    regs.intrrxe().modify(|w| w.set_ep_rxe(index as _, true));

    #[cfg(not(feature = "_fixed-fifo-size"))]
    {
        regs.rx_fifo_sz().write(|w| {
            let size_code = (config.rx_fifo_size_bits - 3) as u8;
            w.set_sz(size_code);
        });
        regs.rx_fifo_add().write(|w| w.set_add(config.rx_fifo_addr_8bytes));
    }

    cfg_if::cfg_if! {
        if #[cfg(feature = "_lite")] {
            if config.rx_max_packet_size % 8 != 0 {
                warn!("RX max packet size must be a multiple of 8 for lite musb IP, using {} instead",
                    ((config.rx_max_packet_size + 7) / 8) * 8
                );
            }
            regs.rxmaxp().write(|w| w.set_maxp((config.rx_max_packet_size + 7) / 8));
        } else {
            regs.rxmaxp().write(|w| w.set_maxp(config.rx_max_packet_size));
        }
    }

    regs.rxcsrl().write(|w| {
        w.set_clr_data_tog(true);
    });

    #[cfg(feature = "_ep-shared-fifo")]
    regs.txcsrh().write(|w| w.set_mode(EndpointDirection::Rx));

    if config.ep_type == EndpointType::Isochronous {
        regs.rxcsrh().write(|w| {
            w.set_iso(true);
        });
    }

    if regs.rxcsrl().read().rx_pkt_rdy() {
        regs.rxcsrl().modify(|w| w.set_flush_fifo(true));
        regs.rxcsrl().modify(|w| w.set_flush_fifo(true));
    }
}

pub fn ep_tx_disable<T: MusbInstance>(index: u8) {
    trace!("musb/ep_disable: Disabling TX endpoint {}", index);
    let regs = T::regs();
    regs.index().write(|w| w.set_index(index));

    regs.txcsrl().modify(|w| w.set_flush_fifo(true));
    regs.txcsrl().modify(|w| w.set_flush_fifo(true));

    regs.txcsrl().write(|w| w.set_clr_data_tog(true));

    regs.txmaxp().write(|w| w.set_maxp(0));
    regs.intrtxe().modify(|w| w.set_ep_txe(index as _, false));
}

pub fn ep_rx_disable<T: MusbInstance>(index: u8) {
    trace!("musb/ep_disable: Disabling RX endpoint {}", index);
    let regs = T::regs();
    regs.index().write(|w| w.set_index(index));

    regs.rxcsrl().modify(|w| w.set_flush_fifo(true));
    regs.rxcsrl().modify(|w| w.set_flush_fifo(true));

    regs.rxcsrl().write(|w| w.set_clr_data_tog(true));

    regs.rxmaxp().write(|w| w.set_maxp(0));
    regs.intrrxe().modify(|w| w.set_ep_rxe(index as _, false));
}

/// Check if TX FIFO is busy (not writable).
/// Returns true if the hardware is still transmitting the previous packet.
/// `index` must not be 0.
#[inline]
pub fn ep_is_tx_busy<T: MusbInstance>(index: u8) -> bool {
    let regs = T::regs();
    regs.index().write(|w| w.set_index(index as _));
    regs.txcsrl().read().tx_pkt_rdy()
}

/// Write data to TX FIFO and trigger transmission unconditionally.
/// **WARNING: Must call `ep_is_tx_busy` and ensure it returns false before calling this function.**
/// `index` must not be 0.
#[inline]
pub fn ep_write_packet_unchecked<T: MusbInstance>(index: u8, buf: &[u8]) {
    let regs = T::regs();
    regs.index().write(|w| w.set_index(index as _));
    
    for b in buf {
        regs.fifo(index as usize).write(|w| w.set_data(*b));
    }
    
    regs.txcsrl().modify(|w| w.set_tx_pkt_rdy(true));
}

/// Check if RX FIFO has data ready to be read.
/// Returns true if a packet has been successfully received.
/// `index` must not be 0.
#[inline]
pub fn ep_is_rx_ready<T: MusbInstance>(index: u8) -> bool {
    let regs = T::regs();
    regs.index().write(|w| w.set_index(index as _));
    regs.rxcsrl().read().rx_pkt_rdy()
}

/// Read data from RX FIFO and clear the ready flag unconditionally.
/// **WARNING: Must call `ep_is_rx_ready` and ensure it returns true before calling this function.**
/// Caller must ensure `buf` is large enough.
/// `index` must not be 0.
#[inline]
pub fn ep_read_packet_unchecked<T: MusbInstance>(index: u8, buf: &mut [u8]) -> usize {
    let regs = T::regs();
    regs.index().write(|w| w.set_index(index as _));
    
    let count = regs.rxcount().read().count() as usize;
    let limit = core::cmp::min(count, buf.len());
    
    for b in buf.iter_mut().take(limit) {
        *b = regs.fifo(index as usize).read().data();
    }
    
    regs.rxcsrl().modify(|w| w.set_rx_pkt_rdy(false));
    limit
}

#[allow(unused)]
#[inline]
pub fn check_overrun<T: MusbInstance>() {
    let regs = T::regs();

    for index in 1..ENDPOINTS.len() {
        regs.index().write(|w| w.set_index(index as _));
        if regs.txcsrl().read().under_run() {
            regs.txcsrl().modify(|w| w.set_under_run(false));
            warn!("Underrun: ep {}", index);
        }
        if regs.rxcsrl().read().over_run() {
            regs.rxcsrl().modify(|w| w.set_over_run(false));
            warn!("Overrun: ep {}", index);
        }
    }
}

#[cfg(not(feature = "_lite"))]
/// `index` must not be 0.
pub fn endpoint_set_rx_dualpacket_enabled<T: MusbInstance>(index: u8, enabled: bool) {
    let regs = T::regs();
    regs.rx_dpktbufdis().modify(|w| w.set_dis(index as _, !enabled));
}

#[cfg(not(feature = "_lite"))]
/// `index` must not be 0.
pub fn endpoint_set_tx_dualpacket_enabled<T: MusbInstance>(index: u8, enabled: bool) {
    let regs = T::regs();
    regs.tx_dpktbufdis().modify(|w| w.set_dis(index as _, !enabled));
}

#[cfg(not(feature = "_lite"))]
pub fn endpoints_set_rx_dualpacket_enabled<T: MusbInstance>(index_bits: u16) {
    use crate::regs::regs::Dpktbufdis;
    let regs = T::regs();
    let bits = (!index_bits) & 0xFFE; // clear EP0 bit
    regs.rx_dpktbufdis().write_value(Dpktbufdis(bits));
}

#[cfg(not(feature = "_lite"))]
pub fn endpoints_set_tx_dualpacket_enabled<T: MusbInstance>(index_bits: u16) {
    use crate::regs::regs::Dpktbufdis;
    let regs = T::regs();
    let bits = (!index_bits) & 0xFFE; // clear EP0 bit
    regs.tx_dpktbufdis().write_value(Dpktbufdis(bits));
}
