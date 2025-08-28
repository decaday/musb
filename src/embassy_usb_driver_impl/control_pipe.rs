use super::*;

/// USB control pipe.
pub struct ControlPipe<'d, T: MusbInstance> {
    pub(super) _phantom: PhantomData<&'d mut T>,
    pub(super) max_packet_size: u16,
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
                continue;
            }

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

        let _ = poll_fn(|cx| {
            EP_RX_WAKERS[0].register(cx.waker());

            regs.index().write(|w| w.set_index(0));
            let ready = regs.csr0l().read().rx_pkt_rdy();
            if ready {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
        .await;

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

        let _ = poll_fn(|cx| {
            EP_TX_WAKERS[0].register(cx.waker());
            regs.index().write(|w| w.set_index(0));
            let unready = regs.csr0l().read().tx_pkt_rdy();
            if unready {
                Poll::Pending
            } else {
                Poll::Ready(())
            }
        })
        .await;
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
        // If SendStall is not set, Musb will automatically send ACK,
        // Programming Guide: No further action is required from the software
        // Should we await for SetupEnd?
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
        trace!("musb/control_pipe: setting addr: {}", addr);
        let regs = T::regs();

        // Wait for SetupEnd
        let _ = poll_fn(|cx| {
            EP_TX_WAKERS[0].register(cx.waker());
            regs.index().write(|w| w.set_index(0));
            let setup_end = regs.csr0l().read().setup_end();
            if setup_end {
                regs.csr0l().modify(|w| w.set_serviced_setup_end(true));
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
        .await;

        trace!("musb/control_pipe: set address acked, setting addr now");
        regs.faddr().write(|w| w.set_func_addr(addr));
    }
}
