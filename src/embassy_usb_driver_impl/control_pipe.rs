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
        let regs = T::regs();
        loop {
            trace!("SETUP read waiting");
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
                trace!("SETUP read failed: {:?}", regs.count0().read().count());
                continue;
            }

            let mut buf = [0; 8];
            (&mut buf)
                .into_iter()
                .for_each(|b| *b = regs.fifo(0).read().data());
            regs.csr0l().modify(|w| w.set_serviced_rx_pkt_rdy(true));

            trace!("SETUP read ok");
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
            "control: data_out len={} first={} last={}",
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
        trace!("READ OK, rx_len = {}", read_count);

        Ok(read_count as usize)
    }

    async fn data_in(&mut self, data: &[u8], first: bool, last: bool) -> Result<(), EndpointError> {
        trace!(
            "control: data_in len={} first={} last={}",
            data.len(),
            first,
            last
        );

        if data.len() > self.ep_in.info.max_packet_size as usize {
            return Err(EndpointError::BufferOverflow);
        }

        let regs = T::regs();

        trace!("WRITE WAITING");

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
        trace!("control: accept");
        // If SendStall is not set, Musb will automatically send ACK
    }

    async fn reject(&mut self) {
        let regs = T::regs();
        trace!("control: reject");

        regs.index().write(|w| w.set_index(0));
        regs.csr0l().modify(|w| {
            w.set_send_stall(true);
            w.set_serviced_rx_pkt_rdy(true);
        });
    }

    async fn accept_set_address(&mut self, addr: u8) {
        // self.accept().await;

        let regs = T::regs();
        trace!("setting addr: {}", addr);
        regs.faddr().write(|w| w.set_func_addr(addr));
    }
}
