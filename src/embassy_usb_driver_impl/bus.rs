use super::*;

use crate::alloc_endpoint::EndpointConfig;
use crate::{bus, endpoint};
use crate::info::ENDPOINTS;

/// USB bus.
pub struct Bus<'d, T: MusbInstance> {
    pub(super) phantom: PhantomData<&'d mut T>,
    pub(super) ep_confs: [EndpointConfig; ENDPOINTS.len()],
    pub(super) inited: bool,
}

impl<'d, T: MusbInstance> driver::Bus for Bus<'d, T> {
    async fn poll(&mut self) -> Event {
        poll_fn(move |cx| {
            BUS_WAKER.register(cx.waker());

            // TODO: implement VBUS detection.
            if !self.inited {
                self.inited = true;
                return Poll::Ready(Event::PowerDetected);
            }

            if IRQ_RESUME.load(Ordering::Acquire) {
                IRQ_RESUME.store(false, Ordering::Relaxed);
                return Poll::Ready(Event::Resume);
            }

            if IRQ_RESET.load(Ordering::Acquire) {
                IRQ_RESET.store(false, Ordering::Relaxed);

                bus::bus_reset::<T>();

                trace!("musb/poll: reset");

                for w in &EP_TX_WAKERS {
                    w.wake()
                }
                for w in &EP_RX_WAKERS {
                    w.wake()
                }

                return Poll::Ready(Event::Reset);
            }

            if IRQ_SUSPEND.load(Ordering::Acquire) {
                IRQ_SUSPEND.store(false, Ordering::Relaxed);
                return Poll::Ready(Event::Suspend);
            }

            Poll::Pending
        })
        .await
    }

    fn endpoint_set_stalled(&mut self, ep_addr: EndpointAddress, stalled: bool) {
        // This can race, so do a retry loop.
        let ep_index = ep_addr.index();
        match ep_addr.direction() {
            Direction::In => {
                ep::ep_tx_stall::<T>(ep_index as _, stalled);
                EP_TX_WAKERS[ep_index].wake();
            }
            Direction::Out => {
                ep::ep_rx_stall::<T>(ep_index as _, stalled);
                EP_RX_WAKERS[ep_index].wake();
            }
        }
    }

    fn endpoint_is_stalled(&mut self, ep_addr: EndpointAddress) -> bool {
        match ep_addr.direction() {
            Direction::In => ep::ep_tx_is_stalled::<T>(ep_addr.index() as _),
            Direction::Out => ep::ep_rx_is_stalled::<T>(ep_addr.index() as _),
        }
    }

    fn endpoint_set_enabled(&mut self, ep_addr: EndpointAddress, enabled: bool) {
        trace!("musb/bus/set_enabled: {:x} {}", ep_addr, enabled);
        let ep_index = ep_addr.index();


        if enabled {
            match ep_addr.direction() {
                Direction::Out => {
                    ep::ep_rx_enable::<T>(ep_index as _, &self.ep_confs[ep_index]);

                    EP_RX_ENABLED.fetch_or((1 << ep_index) as u16, Ordering::Release);
                    // Wake for `Endpoint::wait_enabled()`
                    EP_RX_WAKERS[ep_index].wake();
                }
                Direction::In => {
                    ep::ep_tx_enable::<T>(ep_index as _, &self.ep_confs[ep_index]);

                    EP_TX_ENABLED.fetch_or((1 << ep_index) as u16, Ordering::Release);
                    EP_TX_WAKERS[ep_index].wake();
                }
            }
        } else {
            match ep_addr.direction() {
                Direction::Out => {
                    ep::ep_rx_disable::<T>(ep_index as _);
                    EP_RX_ENABLED.fetch_and(!((1 << ep_index) as u16), Ordering::Release);
                }
                Direction::In => {
                    ep::ep_tx_disable::<T>(ep_index as _);
                    EP_TX_ENABLED.fetch_and(!((1 << ep_index) as u16), Ordering::Release);
                }
            }
        }
    }

    async fn enable(&mut self) {
        bus::bus_set_device_address::<T>(0);
        bus::init_usb_interrupts::<T>();

        bus::bus_connect::<T>(true);
    }
    
    async fn disable(&mut self) {
        bus::bus_connect::<T>(false);
    }

    async fn remote_wakeup(&mut self) -> Result<(), Unsupported> {
        Err(Unsupported)
    }
}
