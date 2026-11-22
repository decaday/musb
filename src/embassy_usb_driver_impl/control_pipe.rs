use super::*;

// Whether the host ended the current control transfer: a bus reset since the
// SETUP, or SetupEnd (it went to the status stage early or sent a new SETUP).
fn ep0_aborted<T: MusbInstance>(reset_gen: u8) -> bool {
    if RESET_GEN.load(Ordering::Acquire) != reset_gen {
        return true;
    }
    let regs = T::regs();
    regs.index().write(|w| w.set_index(0));
    regs.csr0l().read().setup_end()
}

/// USB control pipe.
pub struct ControlPipe<'d, T: MusbInstance> {
    pub(super) _phantom: PhantomData<&'d mut T>,
    pub(super) max_packet_size: u16,
    pub(super) reset_gen: u8,
    pub(super) ep_in: Endpoint<'d, T, In>,
    pub(super) ep_out: Endpoint<'d, T, Out>,
}

impl<'d, T: MusbInstance> driver::ControlPipe for ControlPipe<'d, T> {
    fn max_packet_size(&self) -> usize {
        usize::from(self.max_packet_size)
    }

    async fn setup(&mut self) -> [u8; 8] {
        trace!("musb/control_pipe: setup");
        let regs = T::regs();
        loop {
            poll_fn(|cx| {
                EP_RX_WAKERS[0].register(cx.waker());
                regs.index().write(|w| w.set_index(0));
                // Clear what an ended transfer left behind: SentStall is cleared by
                // writing 0, SetupEnd by setting ServicedSetupEnd.
                let csr0 = regs.csr0l().read();
                if csr0.sent_stall() || csr0.setup_end() {
                    regs.csr0l()
                        .write(|w| w.set_serviced_setup_end(csr0.setup_end()));
                }
                if regs.csr0l().read().rx_pkt_rdy() {
                    Poll::Ready(())
                } else {
                    Poll::Pending
                }
            })
            .await;

            regs.index().write(|w| w.set_index(0));
            if regs.count0().read().count() != 8 {
                trace!("musb/setup: read failed, read count: {:?}", regs.count0().read().count());
                // Drop the packet and stall, or RxPktRdy stays set and this loops forever
                regs.csr0l().modify(|w| {
                    w.set_serviced_rx_pkt_rdy(true);
                    w.set_send_stall(true);
                });
                continue;
            }

            self.reset_gen = RESET_GEN.load(Ordering::Acquire);
            let mut buf = [0; 8];
            (&mut buf)
                .into_iter()
                .for_each(|b| *b = regs.fifo(0).read().data());
            regs.csr0l().modify(|w| w.set_serviced_rx_pkt_rdy(true));

            trace!("musb/setup: read OK");
            return buf;
        }
    }

    async fn data_out(
        &mut self,
        buf: &mut [u8],
        first: bool,
        last: bool,
    ) -> Result<usize, EndpointError> {
        trace!(
            "musb/control_pipe: data_out len={} first={} last={}",
            buf.len(),
            first,
            last
        );

        let regs = T::regs();

        poll_fn(|cx| {
            EP_RX_WAKERS[0].register(cx.waker());
            // A new SETUP in the FIFO isn't this transfer's data
            if ep0_aborted::<T>(self.reset_gen) {
                return Poll::Ready(Err(EndpointError::Disabled));
            }
            regs.index().write(|w| w.set_index(0));
            let ready = regs.csr0l().read().rx_pkt_rdy();
            if ready {
                Poll::Ready(Ok(()))
            } else {
                Poll::Pending
            }
        })
        .await?;

        regs.index().write(|w| w.set_index(0));
        let read_count = regs.count0().read().count();
        if read_count as usize > buf.len() {
            return Err(EndpointError::BufferOverflow);
        }

        if read_count as u16 > self.ep_out.info.max_packet_size {
            return Err(EndpointError::BufferOverflow);
        }

        buf.into_iter()
            .take(read_count as _)
            .for_each(|b| *b = regs.fifo(0).read().data());
        regs.csr0l().modify(|w| {
            w.set_serviced_rx_pkt_rdy(true);
            if last {
                w.set_data_end(true);
            }
        });
        trace!("musb/control_pipe: READ OK, rx_len = {}", read_count);

        Ok(read_count as usize)
    }

    async fn data_in(&mut self, data: &[u8], first: bool, last: bool) -> Result<(), EndpointError> {
        trace!(
            "musb/control_pipe: data_in len={} first={} last={}",
            data.len(),
            first,
            last
        );

        if data.len() > self.ep_in.info.max_packet_size as usize {
            return Err(EndpointError::BufferOverflow);
        }

        let regs = T::regs();

        poll_fn(|cx| {
            EP_TX_WAKERS[0].register(cx.waker());
            // Don't load the FIFO once the host has moved on (a new SETUP may be in it)
            if ep0_aborted::<T>(self.reset_gen) {
                return Poll::Ready(Err(EndpointError::Disabled));
            }
            regs.index().write(|w| w.set_index(0));
            let csr0 = regs.csr0l().read();
            if csr0.rx_pkt_rdy() {
                Poll::Ready(Err(EndpointError::Disabled))
            } else if csr0.tx_pkt_rdy() {
                Poll::Pending
            } else {
                Poll::Ready(Ok(()))
            }
        })
        .await?;
        regs.index().write(|w| w.set_index(0));

        data.into_iter()
            .for_each(|b| regs.fifo(0).write(|w| w.set_data(*b)));

        regs.csr0l().modify(|w| {
            w.set_tx_pkt_rdy(true);
            if last {
                w.set_data_end(true);
            }
        });
        Ok(())
    }

    async fn accept(&mut self) {
        trace!("musb/control_pipe: accept");

        // Set `DataEnd` bit to indicate that no more data is expected
        let regs = T::regs();
        regs.index().write(|w| w.set_index(0));
        regs.csr0l().modify(|w| w.set_data_end(true));

        // musb Programming Guide 21.1.2. WRITE REQUESTS
        // After this transfer completes, MUSB will generate an interrupt, but only
        // INTRTX[0] (indicating EP0) is set. To prevent the state machine from getting
        // stuck, we do not await this interrupt for now.
    }

    async fn reject(&mut self) {
        let regs = T::regs();
        trace!("musb/control_pipe: reject");

        regs.index().write(|w| w.set_index(0));
        regs.csr0l().modify(|w| {
            w.set_send_stall(true);
            w.set_serviced_rx_pkt_rdy(true);
        });
    }

    async fn accept_set_address(&mut self, addr: u8) {
        // self.accept().await;
        // trace!("musb/control_pipe: setting addr: {}", addr);
        let regs = T::regs();

        // We intentionally do not set the `DataEnd` so that the `SetupEnd` indicates the interrupt
        // after the Status Stage completes.
        // musb Programming Guide 21.1.1. ZERO DATA REQUESTS
        let acked = poll_fn(|cx| {
            EP_TX_WAKERS[0].register(cx.waker());
            // After a bus reset the host starts over at address 0
            if RESET_GEN.load(Ordering::Acquire) != self.reset_gen {
                return Poll::Ready(false);
            }
            regs.index().write(|w| w.set_index(0));
            let setup_end = regs.csr0l().read().setup_end();
            if setup_end {
                regs.csr0l().modify(|w| w.set_serviced_setup_end(true));
                Poll::Ready(true)
            } else {
                Poll::Pending
            }
        }).await;
        if !acked {
            return;
        }
        trace!("musb/control_pipe: set address acked, setting addr now");
        regs.faddr().write(|w| w.set_func_addr(addr));
    }
}
