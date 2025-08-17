use embassy_usb_driver::EndpointType;

use crate::alloc_endpoint::EndpointConfig;
use crate::regs::vals::EndpointDirection;
use crate::{warn, MusbInstance};
use crate::info::ENDPOINTS;

pub(crate) fn bus_enable<T: MusbInstance>() {
    T::regs().intrusbe().write(|w| {
        w.set_reset_enable(true);
        w.set_suspend_enable(true);
        w.set_resume_enable(true);
    });
}

pub(crate) fn ep_tx_stall<T: MusbInstance>(index: u8, stalled: bool) {
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
pub(crate) fn ep_rx_stall<T: MusbInstance>(index: u8, stalled: bool) {
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
pub(crate) fn ep_rx_is_stalled<T: MusbInstance>(index: u8) -> bool {
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
pub(crate) fn ep_tx_is_stalled<T: MusbInstance>(index: u8) -> bool {
    let regs = T::regs();
    regs.index().write(|w| w.set_index(index as _));

    if index == 0 {
        // TODO: py32 offiial CherryUsb port returns false directly for EP0
        regs.csr0l().read().send_stall()
    } else {
        regs.txcsrl().read().send_stall()
    }
}

pub(crate) fn ep_tx_enable<T: MusbInstance>(index: u8, config: &EndpointConfig) {
    T::regs().index().write(|w| w.set_index(index));
    if index == 0 {
        T::regs().intrtxe().modify(|w| w.set_ep_txe(0, true))
    } else {
        T::regs()
            .intrtxe()
            .modify(|w| w.set_ep_txe(index as _, true));
    }

    // T::regs().txcsrh().write(|w| {
    //     w.set_auto_set(true);
    // });

    // TODO: DMA

    T::regs()
        .txmaxp()
        .write(|w| w.set_maxp(config.tx_max_packet_size));

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
        .write(|w| w.set_mode(EndpointDirection::TX));

    if T::regs().txcsrl().read().fifo_not_empty() {
        T::regs().txcsrl().modify(|w| w.set_flush_fifo(true));
        T::regs().txcsrl().modify(|w| w.set_flush_fifo(true));
    }
}

pub(crate) fn ep_rx_enable<T: MusbInstance>(index: u8, config: &EndpointConfig) {
    T::regs().index().write(|w| w.set_index(index));

    if index == 0 {
        T::regs().intrtxe().modify(|w|
            // EP0 has only one interrupt enable register
            w.set_ep_txe(0, true))
    } else {
        T::regs()
            .intrrxe()
            .modify(|w| w.set_ep_rxe(index as _, true));
    }

    // T::regs().rxcsrh().write(|w| {
    //     w.set_auto_clear(true);
    // });

    T::regs()
        .rxmaxp()
        .write(|w| w.set_maxp(config.rx_max_packet_size));

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

#[allow(unused)]
pub(crate) fn check_overrun<T: MusbInstance>() {
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
