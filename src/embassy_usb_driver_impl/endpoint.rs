use super::*;
use crate::endpoint;

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
        trace!("musb/ep: endpoint {:#X} wait enabled OK", self.info.addr);
    }
}

impl<'d, T: MusbInstance> driver::EndpointOut for Endpoint<'d, T, Out> {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, EndpointError> {
        trace!("musb/ep: read waiting, buf.len() = {}", buf.len());
        let index = self.info.addr.index();
        let _ = poll_fn(|cx| {
            EP_RX_WAKERS[index].register(cx.waker());
            let ready = endpoint::ep_is_rx_ready::<T>(index as _);
            if ready {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
        .await;

        let read_count = endpoint::ep_read_packet_unchecked::<T>(index as _, buf);
        trace!("musb/ep: read ok, rx_len = {}", read_count);

        Ok(read_count as usize)
    }
}

impl<'d, T: MusbInstance> driver::EndpointIn for Endpoint<'d, T, In> {
    async fn write(&mut self, buf: &[u8]) -> Result<(), EndpointError> {
        if buf.len() > self.info.max_packet_size as usize {
            return Err(EndpointError::BufferOverflow);
        }

        let index = self.info.addr.index();
        trace!("musb/ep: write waiting len = {}", buf.len());

        let _ = poll_fn(|cx| {
            EP_TX_WAKERS[index].register(cx.waker());

            let busy = endpoint::ep_is_tx_busy::<T>(index as _);

            if busy {
                Poll::Pending
            } else {
                Poll::Ready(())
            }
        })
        .await;

        endpoint::ep_write_packet_unchecked::<T>(index as _, buf);
        trace!("musb/ep: write ok");
        Ok(())
    }
}
