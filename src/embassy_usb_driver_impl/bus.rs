use super::*;

use crate::alloc_endpoint::EndpointConfig;
use crate::common_impl;
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

            let regs = T::regs();

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

                regs.power().write(|w| w.set_suspend_mode(true));
                // for index in 1..ENDPOINTS_NUM {
                //     regs.index().write(|w| w.set_index(index as _));
                //     regs.txcsrl().modify(|w| w.set_flush_fifo(true));
                // }

                trace!("RESET");

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
                common_impl::ep_tx_stall::<T>(ep_index as _, stalled);
                EP_TX_WAKERS[ep_addr.index()].wake();
            }
            Direction::Out => {
                common_impl::ep_rx_stall::<T>(ep_index as _, stalled);
                EP_TX_WAKERS[ep_addr.index()].wake();
                EP_RX_WAKERS[ep_addr.index()].wake();
            }
        }
    }

    fn endpoint_is_stalled(&mut self, ep_addr: EndpointAddress) -> bool {
        match ep_addr.direction() {
            Direction::In => common_impl::ep_tx_is_stalled::<T>(ep_addr.index() as _),
            Direction::Out => common_impl::ep_rx_is_stalled::<T>(ep_addr.index() as _),
        }
    }

    fn endpoint_set_enabled(&mut self, ep_addr: EndpointAddress, enabled: bool) {
        trace!("set_enabled {:x} {}", ep_addr, enabled);
        let ep_index = ep_addr.index();

        if enabled {
            T::regs().index().write(|w| w.set_index(ep_index as u8));
            match ep_addr.direction() {
                Direction::Out => {
                    common_impl::ep_rx_enable::<T>(ep_index as _, &self.ep_confs[ep_index]);

                    let flags = EP_RX_ENABLED.load(Ordering::Acquire) | (1 << ep_index) as u16;
                    EP_RX_ENABLED.store(flags, Ordering::Release);
                    // Wake `Endpoint::wait_enabled()`
                    EP_RX_WAKERS[ep_index].wake();
                }
                Direction::In => {
                    common_impl::ep_tx_enable::<T>(ep_index as _, &self.ep_confs[ep_index]);

                    let flags = EP_TX_ENABLED.load(Ordering::Acquire) | (1 << ep_index) as u16;
                    EP_TX_ENABLED.store(flags, Ordering::Release);
                    // Wake `Endpoint::wait_enabled()`
                    EP_TX_WAKERS[ep_index].wake();
                }
            }
        } else {
            // py32 official CherryUsb port does nothing when disable an endpoint
            match ep_addr.direction() {
                Direction::Out => {
                    let flags = EP_RX_ENABLED.load(Ordering::Acquire) & !((1 << ep_index) as u16);
                    EP_RX_ENABLED.store(flags, Ordering::Release);
                }
                Direction::In => {
                    let flags = EP_TX_ENABLED.load(Ordering::Acquire) & !((1 << ep_index) as u16);
                    EP_TX_ENABLED.store(flags, Ordering::Release);
                }
            }
        }
    }

    async fn enable(&mut self) {
        common_impl::bus_enable::<T>();
    }
    async fn disable(&mut self) {}

    async fn remote_wakeup(&mut self) -> Result<(), Unsupported> {
        Err(Unsupported)
    }
}
