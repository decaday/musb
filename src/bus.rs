use super::*;

use crate::regs::vals::EndpointDirection;

/// USB bus.
pub struct Bus<'d, T: MusbInstance> {
    pub(super) phantom: PhantomData<&'d mut T>,
    pub(super) ep_confs: [EndPointConfig; EP_COUNT],
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
                // for index in 1..EP_COUNT {
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
        let reg = T::regs();
        let ep_index = ep_addr.index();
        if ep_index != 0 {
            reg.index().write(|w| w.set_index(ep_index as _));
        }
        match ep_addr.direction() {
            Direction::In => {
                if ep_index == 0 {
                    // usb_ep0_state = USB_EP0_STATE_STALL;

                    reg.csr0l().write(|w| {
                        w.set_send_stall(stalled);
                        if stalled { w.set_serviced_rx_pkt_rdy(true); }
                    });

                    // while !reg.csr0l().read().sent_stall() {}
                }
                else {
                    reg.txcsrl().write(|w| {
                        w.set_send_stall(stalled);
                        if !stalled {
                            w.set_sent_stall(false);
                            w.set_clr_data_tog(true);
                        }
                    });
                    // while !reg.txcsrl().read().sent_stall() {}             
                }
                EP_TX_WAKERS[ep_addr.index()].wake();
            }
            Direction::Out => {
                if ep_index == 0 {
                    // usb_ep0_state = USB_EP0_STATE_STALL;

                    reg.csr0l().write(|w| {
                        w.set_send_stall(stalled);
                        if stalled { w.set_serviced_rx_pkt_rdy(true); }
                    });
                    // while !reg.csr0l().read().sent_stall() {}
                }
                else {
                    reg.rxcsrl().write(|w| {
                        w.set_send_stall(stalled);
                        if !stalled {
                            w.set_sent_stall(false);
                            w.set_clr_data_tog(true);
                        }
                    });
                    // while !reg.rxcsrl().read().sent_stall() {}   
                }
                EP_TX_WAKERS[ep_addr.index()].wake();
                EP_RX_WAKERS[ep_addr.index()].wake();
            }
        }
    }

    fn endpoint_is_stalled(&mut self, ep_addr: EndpointAddress) -> bool {
        let reg = T::regs();
        let ep_index = ep_addr.index();
        if ep_index != 0 {
            reg.index().write(|w| w.set_index(ep_index as _));
        }

        if ep_index == 0 {
            // TODO: py32 offiial CherryUsb port returns false directly for EP0
            reg.csr0l().read().send_stall()
        } else {
            match ep_addr.direction() {
                Direction::In => reg.txcsrl().read().send_stall(),
                Direction::Out => reg.rxcsrl().read().send_stall(),
            }
        }
    }

    fn endpoint_set_enabled(&mut self, ep_addr: EndpointAddress, enabled: bool) {
        trace!("set_enabled {:x} {}", ep_addr, enabled);
        let ep_index = ep_addr.index();
        
        if enabled {
            T::regs().index().write(|w| w.set_index(ep_index as u8));
            match ep_addr.direction() {
                Direction::Out => {
                    if ep_index == 0 {
                        T::regs().intrtxe().modify(|w| 
                            w.set_ep_txe(1, true))
                    } else {
                        T::regs().intrrxe().modify(|w| 
                            w.set_ep_rxe(ep_index, true)
                        );
                    }
                    
                    // T::regs().rxcsrh().write(|w| {
                    //     w.set_auto_clear(true);
                    // });
    
                    T::regs().rxmaxp().write(|w|
                        w.set_maxp(self.ep_confs[ep_index].rx_max_fifo_size_btyes)
                    );
    
                    T::regs().rxcsrl().write(|w| {
                        w.set_clr_data_tog(true);
                    });
    
                    //TODO: DMA
    
                    if self.ep_confs[ep_index].ep_type == EndpointType::Isochronous {
                        T::regs().rxcsrh().write(|w| {
                            w.set_iso(true);
                        });
                    }
    
                    if T::regs().rxcsrl().read().rx_pkt_rdy() {
                        T::regs().rxcsrl().modify(|w| 
                            w.set_flush_fifo(true)
                        );
                    }
                    
                    let flags = EP_RX_ENABLED.load(Ordering::Acquire) | ep_index as u16;
                    EP_RX_ENABLED.store(flags, Ordering::Release);
                    // Wake `Endpoint::wait_enabled()`
                    EP_RX_WAKERS[ep_index].wake();
                }
                Direction::In => {
                    if ep_index == 0 {
                        T::regs().intrtxe().modify(|w| 
                            w.set_ep_txe(1, true))
                    } else {
                        T::regs().intrtxe().modify(|w| 
                            w.set_ep_txe(ep_index, true)
                        );
                    }
    
                    // T::regs().txcsrh().write(|w| {
                    //     w.set_auto_set(true);
                    // });
    
                    // TODO: DMA
    
                    T::regs().txmaxp().write(|w|
                        w.set_maxp(self.ep_confs[ep_index].tx_max_fifo_size_btyes)
                    );
    
                    T::regs().txcsrl().write(|w| {
                        w.set_clr_data_tog(true);
                    });
    
                    if self.ep_confs[ep_index].ep_type == EndpointType::Isochronous {
                        T::regs().txcsrh().write(|w| {
                            w.set_iso(true);
                        });
                    }
                    T::regs().txcsrh().write(|w| w.set_mode(EndpointDirection::TX));
    
                    if T::regs().txcsrl().read().fifo_not_empty() {
                        T::regs().txcsrl().modify(|w|    
                            w.set_flush_fifo(true)
                        );
                    }

                    let flags = EP_TX_ENABLED.load(Ordering::Acquire) | ep_index as u16;
                    EP_TX_ENABLED.store(flags, Ordering::Release);
                    // Wake `Endpoint::wait_enabled()`
                    EP_TX_WAKERS[ep_index].wake();
                }
            }
        }
        else {
            // py32 offiial CherryUsb port does nothing when disable an endpoint
            match ep_addr.direction() {
                Direction::Out => {
                    let flags = EP_RX_ENABLED.load(Ordering::Acquire) & !(ep_index as u16);
                    EP_RX_ENABLED.store(flags, Ordering::Release);
                }
                Direction::In => {
                    let flags = EP_TX_ENABLED.load(Ordering::Acquire) & !(ep_index as u16);
                    EP_TX_ENABLED.store(flags, Ordering::Release);
                }
            }
        }
    }

    async fn enable(&mut self) {
        T::regs().intrusb().write(|w| {
            w.set_reset(true);
            w.set_suspend(true);
            w.set_resume(true);
        });
    }
    async fn disable(&mut self) {}

    async fn remote_wakeup(&mut self) -> Result<(), Unsupported> {
        Err(Unsupported)
    }
}