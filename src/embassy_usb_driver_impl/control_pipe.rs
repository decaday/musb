use super::*;
use crate::{bus, endpoint0};

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
        loop {
            poll_fn(|cx| {
                EP_RX_WAKERS[0].register(cx.waker());
                let ready = endpoint0::ep0_is_rx_ready::<T>();
                if ready {
                    Poll::Ready(())
                } else {
                    Poll::Pending
                }
            })
            .await;

            let mut buf = [0; 8];
            let read_count = endpoint0::ep0_read_packet_unchecked::<T>(&mut buf);
            if read_count != 8 {
                trace!("musb/setup: read failed, read count: {:?}", read_count);
                continue;
            }

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

        let _ = poll_fn(|cx| {
            EP_RX_WAKERS[0].register(cx.waker());

            let ready = endpoint0::ep0_is_rx_ready::<T>();
            if ready {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
        .await;

        let read_count = endpoint0::ep0_read_packet_unchecked::<T>(buf);

        if last {
            endpoint0::ep0_set_data_end::<T>();
        }
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

        let _ = poll_fn(|cx| {
            EP_TX_WAKERS[0].register(cx.waker());
            let busy = endpoint0::ep0_is_tx_busy::<T>();
            if busy {
                Poll::Pending
            } else {
                Poll::Ready(())
            }
        })
        .await;

        endpoint0::ep0_write_packet_unchecked::<T>(data, last);
        Ok(())
    }

    async fn accept(&mut self) {
        trace!("musb/control_pipe: accept");

        // Set `DataEnd` bit to indicate that no more data is expected
        endpoint0::ep0_set_data_end::<T>();

        // musb Programming Guide 21.1.2. WRITE REQUESTS
        // After this transfer completes, MUSB will generate an interrupt, but only
        // INTRTX[0] (indicating EP0) is set. To prevent the state machine from getting
        // stuck, we do not await this interrupt for now.
    }

    async fn reject(&mut self) {
        trace!("musb/control_pipe: reject");
        endpoint0::ep0_stall::<T>();
    }

    async fn accept_set_address(&mut self, addr: u8) {
        // self.accept().await;
        // trace!("musb/control_pipe: setting addr: {}", addr);

        // We intentionally do not set the `DataEnd` so that the `SetupEnd` indicates the interrupt
        // after the Status Stage completes.
        // musb Programming Guide 21.1.1. ZERO DATA REQUESTS
        let _ = poll_fn(|cx| {
            EP_TX_WAKERS[0].register(cx.waker());
            let setup_end = endpoint0::ep0_is_setup_end::<T>();
            if setup_end {
                endpoint0::ep0_clear_setup_end::<T>();
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        }).await;
        trace!("musb/control_pipe: set address acked, setting addr now");
        bus::bus_set_device_address::<T>(addr);
    }
}
