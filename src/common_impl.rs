use embassy_usb_driver::EndpointType;

use crate::alloc_endpoint::EndpointConfig;
use crate::regs::vals::EndpointDirection;
use crate::{trace, warn, MusbInstance};
use crate::info::ENDPOINTS;

pub fn bus_init<T: MusbInstance>() {
    T::regs().intrusbe().write(|w| {
        w.set_reset_enable(true);
        w.set_suspend_enable(true);
        w.set_resume_enable(true);
    });

    T::regs().intrrxe().write_value(Intrrxe(0xFE));
    T::regs().intrtxe().write_value(Intrtxe(0xFF));
}

pub fn ep_tx_stall<T: MusbInstance>(index: u8, stalled: bool) {
    let regs = T::regs();
    regs.index().write(|w| w.set_index(index as _));

    if index == 0 {
        regs.csr0l().write(|w| {
            w.set_send_stall(stalled);
            // TODO
            // if stalled { w.set_serviced_tx_pkt_rdy(true); }
        });
    } else {
        regs.txcsrl().write(|w| {
            w.set_send_stall(stalled);
            if !stalled {
                w.set_sent_stall(false);
                w.set_clr_data_tog(true);
            }
        });
    }
}

#[inline]
pub fn ep_rx_stall<T: MusbInstance>(index: u8, stalled: bool) {
    let regs = T::regs();
    regs.index().write(|w| w.set_index(index as _));
    if index == 0 {
        regs.csr0l().write(|w| {
            w.set_send_stall(stalled);
            if stalled {
                w.set_serviced_rx_pkt_rdy(true);
            }
        });
    } else {
        regs.rxcsrl().write(|w| {
            w.set_send_stall(stalled);
            if !stalled {
                w.set_sent_stall(false);
                w.set_clr_data_tog(true);
            }
        });
    }
}

#[inline]
pub fn ep_rx_is_stalled<T: MusbInstance>(index: u8) -> bool {
    let regs = T::regs();
    regs.index().write(|w| w.set_index(index as _));

    if index == 0 {
        // TODO: py32 offiial CherryUsb port returns false directly for EP0
        regs.csr0l().read().send_stall()
    } else {
        regs.rxcsrl().read().send_stall()
    }
}

#[inline]
pub fn ep_tx_is_stalled<T: MusbInstance>(index: u8) -> bool {
    let regs = T::regs();
    regs.index().write(|w| w.set_index(index as _));

    if index == 0 {
        // TODO: py32 offiial CherryUsb port returns false directly for EP0
        regs.csr0l().read().send_stall()
    } else {
        regs.txcsrl().read().send_stall()
    }
}

pub fn ep_tx_enable<T: MusbInstance>(index: u8, config: &EndpointConfig) {
    #[cfg(not(feature="_fixed-fifo-size"))]
    trace!(
        "Enabling TX endpoint {}: max_packet_size={}, fifo_size_bits={}, fifo_addr_8bytes={}, ep_type={:?}",
        index,
        config.tx_max_packet_size,
        config.tx_fifo_size_bits,
        config.tx_fifo_addr_8bytes,
        config.ep_type
    );
    #[cfg(feature = "_fixed-fifo-size")]
    trace!(
        "Enabling TX endpoint {}: max_packet_size={}, ep_type={:?}",
        index,
        config.tx_max_packet_size,
        config.ep_type
    );

    T::regs().index().write(|w| w.set_index(index));
    if index == 0 {
        T::regs().intrtxe().modify(|w| w.set_ep_txe(0, true));
        T::regs().csr0l().modify(|w| {
             w.set_serviced_rx_pkt_rdy(true);
             w.set_serviced_setup_end(true);
        });
        #[cfg(not(feature = "_mini"))]
        T::regs().csr0h().modify(|w| {
            w.set_flush_fifo(true);
        });
    } else {
        T::regs()
            .intrtxe()
            .modify(|w| w.set_ep_txe(index as _, true));
    }

    // T::regs().txcsrh().write(|w| {
    //     w.set_auto_set(true);
    // });

    // TODO: DMA

    if index != 0 {
        // This logic is only compiled when we are NOT using fixed FIFOs.
        #[cfg(not(feature = "_fixed-fifo-size"))]
        {
        T::regs().tx_fifo_sz().write(|w| {
            let size_code = (config.tx_fifo_size_bits - 3) as u8;
            w.set_sz(size_code);
            w.set_dpb(true);
        });
        T::regs().tx_fifo_add().write(|w| w.set_add(config.tx_fifo_addr_8bytes));
    }

    cfg_if::cfg_if! {
        if #[cfg(feature = "_mini")] {
            if config.tx_max_packet_size % 8 != 0 {
                warn!("TX max packet size must be a multiple of 8 for mini musb IP, using {} instead",
                    ((config.tx_max_packet_size + 7) / 8) * 8
                );
            }

            // Mini version uses 8-byte unit
            T::regs()
                .txmaxp()
                .write(|w| w.set_maxp((config.tx_max_packet_size + 7) / 8));
        } else {
            // Full version uses full packet size
            T::regs()
                .txmaxp()
                .write(|w| w.set_maxp(config.tx_max_packet_size));
        }
    }

    T::regs().txcsrl().write(|w| {
        w.set_clr_data_tog(true);
    });

    if config.ep_type == EndpointType::Isochronous {
        T::regs().txcsrh().write(|w| {
            w.set_iso(true);
        });
    }
    T::regs()
        .txcsrh()
        .write(|w| w.set_mode(EndpointDirection::Tx));

    if T::regs().txcsrl().read().fifo_not_empty() {
        T::regs().txcsrl().modify(|w| w.set_flush_fifo(true));
        T::regs().txcsrl().modify(|w| w.set_flush_fifo(true));
        }
    }
}

pub fn ep_rx_enable<T: MusbInstance>(index: u8, config: &EndpointConfig) {
    #[cfg(not(feature="_fixed-fifo-size"))]
    trace!(
        "Enabling RX endpoint {}: max_packet_size={}, fifo_size_bits={}, fifo_addr_8bytes={}, ep_type={:?}",
        index,
        config.rx_max_packet_size,
        config.rx_fifo_size_bits,
        config.rx_fifo_addr_8bytes,
        config.ep_type
    );
    #[cfg(feature = "_fixed-fifo-size")]
    trace!(
        "Enabling RX endpoint {}: max_packet_size={}, ep_type={:?}",
        index,
        config.rx_max_packet_size,
        config.ep_type
    );

    T::regs().index().write(|w| w.set_index(index));

    if index == 0 {
        T::regs().intrtxe().modify(|w|
            // EP0 has only one interrupt enable register
            w.set_ep_txe(0, true));
        T::regs().csr0l().modify(|w| {
             w.set_serviced_rx_pkt_rdy(true);
             w.set_serviced_setup_end(true);
        });
        #[cfg(not(feature = "_mini"))]
        T::regs().csr0h().modify(|w| {
            w.set_flush_fifo(true);
        });
    } else {
        T::regs()
            .intrrxe()
            .modify(|w| w.set_ep_rxe(index as _, true));

    // T::regs().rxcsrh().write(|w| {
    //     w.set_auto_clear(true);
    // });

    #[cfg(not(feature = "_fixed-fifo-size"))]
        {
        T::regs().rx_fifo_sz().write(|w| {
            let size_code = (config.rx_fifo_size_bits - 3) as u8;
            w.set_sz(size_code);
            w.set_dpb(true);
        });
        T::regs().rx_fifo_add().write(|w| w.set_add(config.rx_fifo_addr_8bytes));
    }

    cfg_if::cfg_if! {
        if #[cfg(feature = "_mini")] {
            if config.rx_max_packet_size % 8 != 0 {
                warn!("RX max packet size must be a multiple of 8 for mini musb IP, using {} instead",
                    ((config.rx_max_packet_size + 7) / 8) * 8
                );
            }
            // Mini version uses 8-byte unit
            T::regs()
                .rxmaxp()
                .write(|w| w.set_maxp((config.rx_max_packet_size + 7) / 8));
        } else {
            T::regs()
                .rxmaxp()
                .write(|w| w.set_maxp(config.rx_max_packet_size));
        }
    }

    T::regs().rxcsrl().write(|w| {
        w.set_clr_data_tog(true);
    });

    //TODO: DMA

    if config.ep_type == EndpointType::Isochronous {
        T::regs().rxcsrh().write(|w| {
            w.set_iso(true);
        });
    }

    if T::regs().rxcsrl().read().rx_pkt_rdy() {
        T::regs().rxcsrl().modify(|w| w.set_flush_fifo(true));
        T::regs().rxcsrl().modify(|w| w.set_flush_fifo(true));
        }
    }
}

#[allow(unused)]
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

#[cfg(not(feature = "_mini"))]
pub fn endpoint_set_rx_dualpacket_enabled<T: MusbInstance>(index: u8, enabled: bool) {
    let regs = T::regs();
    if index == 0 {
        // should panic?
        warn!("EP0 does not support dual packet mode");
    } else {
        regs.rx_dpktbufdis()
            .modify(|w| w.set_dis(index as _,!enabled));
    }
}

#[cfg(not(feature = "_mini"))]
pub fn endpoint_set_tx_dualpacket_enabled<T: MusbInstance>(index: u8, enabled: bool) {
    let regs = T::regs();
    if index == 0 {
        // should panic?
        warn!("EP0 does not support dual packet mode");
    } else {
        regs.tx_dpktbufdis()
            .modify(|w| w.set_dis(index as _, !enabled));
    }
}

#[cfg(not(feature = "_mini"))]
pub fn endpoints_set_rx_dualpacket_enabled<T: MusbInstance>(index_bits: u16) {
    use crate::regs::regs::Dpktbufdis;
    let regs = T::regs();
    let bits = (!index_bits) & 0xFFE; // clear EP0 bit
    regs.rx_dpktbufdis().write_value(Dpktbufdis(bits));
}

#[cfg(not(feature = "_mini"))]
pub fn endpoints_set_tx_dualpacket_enabled<T: MusbInstance>(index_bits: u16) {
    use crate::regs::regs::Dpktbufdis;
    let regs = T::regs();
    let bits = (!index_bits) & 0xFFE; // clear EP0 bit
    regs.tx_dpktbufdis().write_value(Dpktbufdis(bits));
}
