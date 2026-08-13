use crate::{trace, MusbInstance};

pub struct UsbInterruptStatus {
    pub reset: bool,
    pub suspend: bool,
    pub resume: bool,
}

#[inline]
pub fn read_and_clear_usb_interrupts<T: MusbInstance>() -> UsbInterruptStatus {
    let intrusb = T::regs().intrusb().read();
    UsbInterruptStatus {
        reset: intrusb.reset(),
        suspend: intrusb.suspend(),
        resume: intrusb.resume(),
    }
}

#[inline]
pub fn init_usb_interrupts<T: MusbInstance>() {
    T::regs().intrusbe().write(|w| {
        w.set_reset_enable(true);
        w.set_suspend_enable(true);
        w.set_resume_enable(true);
    });
}

#[inline]
pub fn bus_reset<T: MusbInstance>() {
    let regs = T::regs();
    regs.index().write(|w| w.set_index(0));
    #[cfg(not(feature = "_lite"))]
    regs.csr0h().modify(|w| w.set_flush_fifo(true));
    regs.csr0l().modify(|w| w.set_serviced_rx_pkt_rdy(true));
    for index in 1..crate::info::ENDPOINTS.len() {
        regs.index().write(|w| w.set_index(index as _));
        regs.txcsrl().modify(|w| w.set_flush_fifo(true));
        regs.rxcsrl().modify(|w| w.set_flush_fifo(true));
    }
}

/// Set device address.
/// Must be called after the Status Stage of the `SET_ADDRESS` transfer has completely finished.
#[inline]
pub fn bus_set_device_address<T: MusbInstance>(addr: u8) {
    trace!("musb/bus: set_device_address: {}", addr);
    T::regs().faddr().write(|w| w.set_func_addr(addr));
}

#[inline]
#[allow(unused_variables)]
pub fn bus_connect<T: MusbInstance>(connected: bool) {
    #[cfg(not(feature = "_lite"))]
    T::regs().power().modify(|w| {
        w.set_soft_conn(connected);
    });

    // T::regs().devctl().write(|w| {
    //     w.set_session(connected);
    // });
}
