use super::*;

/// USB endpoint.
pub struct Endpoint<'d, T: MusbInstance, D> {
    pub(super) _phantom: PhantomData<(&'d mut T, D)>,
    pub(super) info: EndpointInfo,
}

// impl<'d, T: MusbInstance, > driver::Endpoint for Endpoint<'d, T, In> {
impl<'d, T: MusbInstance, D: Dir> driver::Endpoint for Endpoint<'d, T, D> {
    fn info(&self) -> &EndpointInfo {
        &self.info
    }

    async fn wait_enabled(&mut self) {
        let _ = poll_fn(|cx| {
            let index = self.info.addr.index();

            let enabled = match self.info.addr.direction() {
                Direction::Out => {
                    EP_RX_WAKERS[index].register(cx.waker());
                    EP_RX_ENABLED.load(Ordering::Acquire) & ((1 << index) as u16) != 0
                }
                Direction::In => {
                    EP_TX_WAKERS[index].register(cx.waker());
                    EP_TX_ENABLED.load(Ordering::Acquire) & ((1 << index) as u16) != 0
                }
            };
            if enabled {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
        .await;
        trace!("Endpoint {:#X} wait enabled OK", self.info.addr);
    }
}

impl<'d, T: MusbInstance> driver::EndpointOut for Endpoint<'d, T, Out> {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, EndpointError> {
        trace!("READ WAITING, buf.len() = {}", buf.len());
        let index = self.info.addr.index();
        let regs = T::regs();

        let _ = poll_fn(|cx| {
            EP_RX_WAKERS[index].register(cx.waker());
            regs.index().write(|w| w.set_index(index as _));
            let ready = regs.rxcsrl().read().rx_pkt_rdy();
            if ready {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
        .await;

        regs.index().write(|w| w.set_index(index as _));
        let read_count = regs.rxcount().read().count();
        if read_count as usize > buf.len() {
            return Err(EndpointError::BufferOverflow);
        }

        buf.into_iter()
            .take(read_count as _)
            .for_each(|b| *b = regs.fifo(index).read().data());
        regs.rxcsrl().modify(|w| w.set_rx_pkt_rdy(false));
        trace!("READ OK, rx_len = {}", read_count);

        Ok(read_count as usize)
    }
}

impl<'d, T: MusbInstance> driver::EndpointIn for Endpoint<'d, T, In> {
    async fn write(&mut self, buf: &[u8]) -> Result<(), EndpointError> {
        if buf.len() > self.info.max_packet_size as usize {
            return Err(EndpointError::BufferOverflow);
        }

        let index = self.info.addr.index();
        let regs = T::regs();

        trace!("WRITE WAITING len = {}", buf.len());

        let _ = poll_fn(|cx| {
            EP_TX_WAKERS[index].register(cx.waker());
            regs.index().write(|w| w.set_index(index as _));

            let unready = regs.txcsrl().read().tx_pkt_rdy();

            if unready {
                Poll::Pending
            } else {
                Poll::Ready(())
            }
        })
        .await;

        regs.index().write(|w| w.set_index(index as _));
        buf.into_iter()
            .for_each(|b| regs.fifo(index).write(|w| w.set_data(*b)));

        regs.txcsrl().modify(|w| w.set_tx_pkt_rdy(true));
        trace!("WRITE OK");
        Ok(())
    }
}
