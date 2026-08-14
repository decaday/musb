use crate::MusbInstance;

/// Stall EP0
#[inline]
pub fn ep0_stall<T: MusbInstance>() {
    let regs = T::regs();
    regs.index().write(|w| w.set_index(0));
    regs.csr0l().write(|w| {
        w.set_send_stall(true);
        w.set_serviced_rx_pkt_rdy(true);
    });
}

/// Check if EP0 RX FIFO has data ready to be read.
#[inline]
pub fn ep0_is_rx_ready<T: MusbInstance>() -> bool {
    let regs = T::regs();
    regs.index().write(|w| w.set_index(0));
    regs.csr0l().read().rx_pkt_rdy()
}

/// Check if EP0 TX FIFO is busy (not writable).
#[inline]
pub fn ep0_is_tx_busy<T: MusbInstance>() -> bool {
    let regs = T::regs();
    regs.index().write(|w| w.set_index(0));
    regs.csr0l().read().tx_pkt_rdy()
}

/// Read data from EP0 RX FIFO unconditionally.
/// **WARNING: Must call `ep0_is_rx_ready` and ensure it returns true before calling this.**
/// **WARNING**: This function DOES NOT clear the `RxPktRdy` flag or set `serviced_rx_pkt_rdy`.
/// You must explicitly call `ep0_clear_rx_ready` to advance the hardware state machine.
#[inline]
pub fn ep0_read_packet_unchecked<T: MusbInstance>(buf: &mut [u8]) -> usize {
    let regs = T::regs();
    regs.index().write(|w| w.set_index(0));
    
    let count = regs.count0().read().count() as usize;
    let limit = core::cmp::min(count, buf.len());
    
    for b in buf.iter_mut().take(limit) {
        *b = regs.fifo(0).read().data();
    }
    limit
}

/// Acknowledge the received packet by setting `serviced_rx_pkt_rdy` and optionally `data_end`
/// in a single register write to satisfy the MUSB simultaneous write requirement for Zero Data requests.
#[inline]
pub fn ep0_clear_rx_ready<T: MusbInstance>(data_end: bool) {
    let regs = T::regs();
    regs.index().write(|w| w.set_index(0));
    regs.csr0l().modify(|w| {
        w.set_serviced_rx_pkt_rdy(true);
        if data_end {
            w.set_data_end(true);
        }
    });
}

/// Write data to EP0 TX FIFO and trigger transmission unconditionally.
/// **WARNING: Must call `ep0_is_tx_busy` and ensure it returns false before calling this.**
/// If `is_last` is true, it also sets the `data_end` bit.
#[inline]
pub fn ep0_write_packet_unchecked<T: MusbInstance>(buf: &[u8], is_last: bool) {
    let regs = T::regs();
    regs.index().write(|w| w.set_index(0));
    
    for b in buf {
        regs.fifo(0).write(|w| w.set_data(*b));
    }
    
    regs.csr0l().modify(|w| {
        w.set_tx_pkt_rdy(true);
        if is_last {
            w.set_data_end(true);
        }
    });
}

/// Set DataEnd bit on EP0 to indicate the end of data phase (or zero data request).
#[inline]
pub fn ep0_set_data_end<T: MusbInstance>() {
    let regs = T::regs();
    regs.index().write(|w| w.set_index(0));
    regs.csr0l().modify(|w| w.set_data_end(true));
}

/// Check if SetupEnd bit is set on EP0.
#[inline]
pub fn ep0_is_setup_end<T: MusbInstance>() -> bool {
    let regs = T::regs();
    regs.index().write(|w| w.set_index(0));
    regs.csr0l().read().setup_end()
}

/// Clear the SetupEnd bit on EP0 by setting `serviced_setup_end`.
#[inline]
pub fn ep0_clear_setup_end<T: MusbInstance>() {
    let regs = T::regs();
    regs.index().write(|w| w.set_index(0));
    regs.csr0l().modify(|w| w.set_serviced_setup_end(true));
}
