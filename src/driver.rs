use super::*;

/// MUSB driver.
pub struct MusbDriver<'d, T: MusbInstance> {
    phantom: PhantomData<&'d mut T>,
    alloc: [EndpointData; ENDPOINTS_NUM],
}

impl<'d, T: MusbInstance> MusbDriver<'d, T> {
    /// Create a new USB driver.
    pub fn new() -> Self {
        let regs = T::regs();
        
        regs.index().write(|w| w.set_index(0));
        
        // Initialize the bus so that it signals that power is available
        BUS_WAKER.wake();

        Self {
            phantom: PhantomData,
            alloc: [EndpointData {
                ep_conf: EndPointConfig {
                    ep_type: EndpointType::Bulk,
                    tx_max_fifo_size_dword: 1,
                    rx_max_fifo_size_dword: 1,
                },
                used_in: false,
                used_out: false,
            }; ENDPOINTS_NUM],
        }
    }

    pub fn alloc_endpoint<D: Dir>(
        &mut self,
        ep_type: EndpointType,
        max_packet_size: u16,
        interval_ms: u8,
        is_ep0: bool,
    ) -> Result<Endpoint<'d, T, D>, driver::EndpointAllocError> {
        trace!(
            "allocating type={:?} mps={:?} interval_ms={}, dir={:?}",
            ep_type,
            max_packet_size,
            interval_ms,
            D::dir()
        );


        let index = if is_ep0 {
            Some((0, &mut self.alloc[0]))
        }
        else {
            self.alloc.iter_mut().enumerate().find(|(i, ep)| {
                if *i == 0 {
                    return false; // reserved for control pipe
                }
                let used = ep.used_out || ep.used_in;
                
                #[cfg(all(not(feature = "allow-ep-shared-fifo"), feature = "_ep-shared-fifo"))]
                if used { return false }

                #[cfg(not(feature = "_equal-fifo-size"))]
                if ((max_packet_size + 7) / 8) as u8 > MAX_FIFO_SIZE_DWPRD[*i] {
                    return false;
                }

                #[cfg(feature = "_equal-fifo-size")]
                if ((max_packet_size + 7) / 8) as u8 > MAX_FIFO_SIZE_DWPRD {
                    panic!("max_packet_size > MAX_FIFO_SIZE");
                }

                let used_dir = match D::dir() {
                    Direction::Out => ep.used_out,
                    Direction::In => ep.used_in,
                };
                !used || (ep.ep_conf.ep_type == ep_type && !used_dir)
            })
        };

        let (index, ep) = match index {
            Some(x) => x,
            None => return Err(EndpointAllocError),
        };

        ep.ep_conf.ep_type = ep_type;
        

        T::regs().index().write(|w| w.set_index(index as u8));
        match D::dir() {
            Direction::Out => {
                self::assert!(!ep.used_out);
                ep.used_out = true;

                ep.ep_conf.rx_max_fifo_size_dword = calc_max_fifo_size_dword(max_packet_size);
            }
            Direction::In => {
                self::assert!(!ep.used_in);
                ep.used_in = true;

                ep.ep_conf.tx_max_fifo_size_dword = calc_max_fifo_size_dword(max_packet_size);
            }
        };

        Ok(Endpoint {
            _phantom: PhantomData,
            info: EndpointInfo {
                addr: EndpointAddress::from_parts(index, D::dir()),
                ep_type,
                max_packet_size,
                interval_ms,
            },
        })
    }

    pub fn start(mut self, control_max_packet_size: u16) -> (crate::Bus<'d, T>, crate::ControlPipe<'d, T>) {
        let ep_out = self
            .alloc_endpoint(EndpointType::Control, control_max_packet_size, 0, true)
            .unwrap();
        let ep_in = self
            .alloc_endpoint(EndpointType::Control, control_max_packet_size, 0, true)
            .unwrap();
        
        trace!("enabled");

        let mut ep_confs = [EndPointConfig {
            ep_type: EndpointType::Bulk,
            tx_max_fifo_size_dword: 1,
            rx_max_fifo_size_dword: 1,
        }; ENDPOINTS_NUM];
        
        for i in 0..ENDPOINTS_NUM {
            ep_confs[i] = self.alloc[i].ep_conf;
        }

        (
            Bus {
                phantom: PhantomData,
                ep_confs,
                inited: false,
            },
            ControlPipe {
                _phantom: PhantomData,
                max_packet_size: control_max_packet_size,
                ep_out,
                ep_in,
            },
        )
    }
}

// impl<'d, T: MusbInstance> driver::Driver<'d> for Driver<'d, T> {
//     type EndpointOut = Endpoint<'d, T, Out>;
//     type EndpointIn = Endpoint<'d, T, In>;
//     type ControlPipe = ControlPipe<'d, T>;
//     type Bus = Bus<'d, T>;

//     fn alloc_endpoint_in(
//         &mut self,
//         ep_type: EndpointType,
//         max_packet_size: u16,
//         interval_ms: u8,
//     ) -> Result<Self::EndpointIn, driver::EndpointAllocError> {
//         self.alloc_endpoint(ep_type, max_packet_size, interval_ms, false)
//     }

//     fn alloc_endpoint_out(
//         &mut self,
//         ep_type: EndpointType,
//         max_packet_size: u16,
//         interval_ms: u8,
//     ) -> Result<Self::EndpointOut, driver::EndpointAllocError> {
//         self.alloc_endpoint(ep_type, max_packet_size, interval_ms, false)
//     }
// }