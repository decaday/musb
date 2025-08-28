#![no_std]

pub use musb::{MusbInstance, regs};

/// Describes the FIFO sizing strategy of the MUSB core.
#[derive(defmt::Format)]
pub enum FifoSizing {
    /// FIFO sizes are fixed at synthesis time.
    Static,
    /// FIFO sizes can be dynamically configured by software.
    Dynamic,
}

/// Describes the UTMI+ interface data width.
#[derive(defmt::Format)]
pub enum UtmiDataWidth {
    /// 8-bit data width.
    Bit8,
    /// 16-bit data width.
    Bit16,
    /// An unknown or unsupported value was read.
    Unknown(u8),
}

/// Holds the decoded configuration of the MUSB IP core.
///
/// This structure is populated by reading hardware registers.
pub struct Configuration {
    // --- Fields from CONFIGDATA register ---
    pub mprxe: bool,
    pub mptxe: bool,
    pub big_endian: bool,
    pub hbrxe: bool,
    pub hbtxe: bool,
    pub dyn_fifo_sizing: FifoSizing,
    pub soft_connect: bool,
    pub utmi_data_width: UtmiDataWidth,

     // --- Fields from FIFOSIZE registers ---
    pub tx_fifo_sizes: [u16; 16],
    pub rx_fifo_sizes: [u16; 16],

    // --- Fields from Additional Registers ---
    pub rx_endpoints: u8,
    pub tx_endpoints: u8,
    pub dma_channels: u8,
    pub ram_bits: u8,
    pub wtcon: u8,
    pub wtid: u8,
    pub vplen: u8,
    pub hs_eof1: u8,
    pub fs_eof1: u8,
    pub ls_eof1: u8,

    // --- Flags to track if registers read all-zeros ---
    epinfo_is_zero: bool,
    raminfo_is_zero: bool,
    linkinfo_is_zero: bool,
    vplen_is_zero: bool,
    hs_eof1_is_zero: bool,
    fs_eof1_is_zero: bool,
    ls_eof1_is_zero: bool,
}

impl Configuration {
    /// Reads the MUSB IP configuration by accessing its hardware registers.
    ///
    /// This function uses high-level field accessor methods and checks for all-zero
    /// register values to detect potentially masked hardware features.
    pub fn read<T: MusbInstance>() -> Self {
        let regs = T::regs();

        defmt::trace!("INDEX addr: 0x{:X}", &regs.index().as_ptr());
        // --- Read and Decode CONFIGDATA ---
        regs.index().write(|w| w.set_index(0));
        let configdata = regs.configdata().read();
        let mprxe = configdata.mprxe();
        let mptxe = configdata.mptxe();
        let big_endian = configdata.big_endian();
        let hbrxe = configdata.hbrxe();
        let hbtxe = configdata.hbtxe();
        let dyn_fifo_sizing = if configdata.dyn_fifo_sizing() {
            FifoSizing::Dynamic
        } else {
            FifoSizing::Static
        };
        let soft_connect = configdata.soft_con_e();
        let utmi_data_width = match configdata.utmi_data_width().into() {
            0 => UtmiDataWidth::Bit8,
            1 => UtmiDataWidth::Bit16,
            val => UtmiDataWidth::Unknown(val),
        };

                // --- Read and Decode FIFOSIZE for each endpoint (if static) ---
        let mut tx_fifo_sizes = [0u16; 16];
        let mut rx_fifo_sizes = [0u16; 16];

        if let FifoSizing::Static = dyn_fifo_sizing {
            for i in 0..=15 {
                regs.index().write(|w| w.set_index(i));
                let tx_nibble = regs.fifosize().read().tx_fifo_size();
                let rx_nibble = regs.fifosize().read().rx_fifo_size();
                // defmt::trace!("Endpoint {}: FIFOSIZE register = {:b}, TX nibble = 0x{:x}, RX nibble = 0x{:x}", i, regs.fifosize().read().0, tx_nibble, rx_nibble);

                tx_fifo_sizes[i as usize] = Self::decode_fifo_size_nibble(tx_nibble);
                
                if rx_nibble == 0xF {
                    // 0xF indicates the RX endpoint shares the TX FIFO
                    rx_fifo_sizes[i as usize] = u16::MAX; 
                } else {
                    rx_fifo_sizes[i as usize] = Self::decode_fifo_size_nibble(rx_nibble);
                }
            }
        }
        regs.index().write(|w| w.set_index(0));

        // --- Read and Decode Additional Configuration Registers ---

        // EPINFO (Address 0x78)
        let epinfo_reader = regs.epinfo().read();
        let epinfo_is_zero = epinfo_reader.0 == 0;
        let rx_endpoints = epinfo_reader.rx_end_points();
        let tx_endpoints = epinfo_reader.tx_end_points();

        // RAMINFO (Address 0x79)
        let raminfo_reader = regs.raminfo().read();
        let raminfo_is_zero = raminfo_reader.0 == 0;
        let dma_channels = raminfo_reader.dmachans();
        let ram_bits = raminfo_reader.ram_bits();

        // LINKINFO (Address 0x7A)
        let linkinfo_reader = regs.linkinfo().read();
        let linkinfo_is_zero = linkinfo_reader.0 == 0;
        let wtcon = linkinfo_reader.wtcon();
        let wtid = linkinfo_reader.wtid();

        // VPLEN (Address 0x7B)
        let vplen_reader = regs.vplen().read();
        let vplen_is_zero = vplen_reader.0 == 0;
        let vplen = vplen_reader.vplen();

        // HS_EOF1 (Address 0x7C)
        let hs_eof1_reader = regs.hs_eof1().read();
        let hs_eof1_is_zero = hs_eof1_reader.0 == 0;
        let hs_eof1 = hs_eof1_reader.hs_eof1();

        // FS_EOF1 (Address 0x7D)
        let fs_eof1_reader = regs.fs_eof1().read();
        let fs_eof1_is_zero = fs_eof1_reader.0 == 0;
        let fs_eof1 = fs_eof1_reader.fs_eof1();

        // LS_EOF1 (Address 0x7E)
        let ls_eof1_reader = regs.ls_eof1().read();
        let ls_eof1_is_zero = ls_eof1_reader.0 == 0;
        let ls_eof1 = ls_eof1_reader.ls_eof1();

        Self {
            mprxe, mptxe, big_endian, hbrxe, hbtxe, dyn_fifo_sizing,
            tx_fifo_sizes, rx_fifo_sizes,
            soft_connect, utmi_data_width,
            rx_endpoints, tx_endpoints, dma_channels, ram_bits,
            wtcon, wtid, vplen, hs_eof1, fs_eof1, ls_eof1,
            epinfo_is_zero, raminfo_is_zero, linkinfo_is_zero,
            vplen_is_zero, hs_eof1_is_zero, fs_eof1_is_zero, ls_eof1_is_zero,
        }
    }

    /// Decodes a 4-bit FIFOSIZE nibble into the corresponding size in bytes.
    /// According to the spec, values 3-13 correspond to 2^n bytes.
    /// Other values mean the endpoint is not configured.
    fn decode_fifo_size_nibble(nibble: u8) -> u16 {
        match nibble {
            3..=13 => 1u16 << nibble,
            _ => 0, // Not configured or invalid
        }
    }

    /// Prints the decoded configuration information using `defmt`.
    pub fn print_defmt(&self) {
        defmt::info!("--- MUSB Core Configuration ---");

        // CONFIGDATA Details
        defmt::info!("Core Configuration (from CONFIGDATA):");
        defmt::info!("  - Automatic Bulk Packet Amalgamation (MPRxE): {}", self.mprxe);
        defmt::info!("  - Automatic Bulk Packet Splitting (MPTxE): {}", self.mptxe);
        // ... (other CONFIGDATA fields remain the same)
        if self.big_endian {
            defmt::warn!("  - Endianness: Big Endian. (Documentation states this should be Little Endian)");
        } else {
            defmt::info!("  - Endianness: Little Endian");
        }
        defmt::info!("  - High-Bandwidth Rx ISO Endpoint Support (HBRxE): {}", self.hbrxe);
        defmt::info!("  - High-Bandwidth Tx ISO Endpoint Support (HBTxE): {}", self.hbtxe);
        defmt::info!("  - FIFO Sizing: {}", self.dyn_fifo_sizing);
        if self.soft_connect {
            defmt::info!("  - Connection Type: Soft Connect/Disconnect");
        } else {
            defmt::warn!("  - Connection Type: Hard-wired. (Documentation states this should be Soft Connect)");
        }
        defmt::info!("  - UTMI+ Data Width: {}", self.utmi_data_width);

        // FIFO Size Details
        defmt::info!("Endpoint FIFO Configuration (from FIFOSIZE):");
        match self.dyn_fifo_sizing {
            FifoSizing::Dynamic => {
                defmt::info!("  - Dynamic FIFO Sizing is enabled. The FIFOSIZE register is not applicable.");
            }
            FifoSizing::Static => {
                let mut found_any = false;
                for i in 1..=15 {
                    let tx_size = self.tx_fifo_sizes[i];
                    let rx_size = self.rx_fifo_sizes[i];

                    if tx_size > 0 || rx_size > 0 {
                        found_any = true;
                        if tx_size > 0 {
                            if rx_size == u16::MAX {
                                defmt::info!("  - Endpoint {}: TX FIFO = {=u16} bytes, RX FIFO = (Shared with TX)", i, tx_size);
                            } else if rx_size > 0 {
                                defmt::info!("  - Endpoint {}: TX FIFO = {=u16} bytes, RX FIFO = {=u16} bytes", i, tx_size, rx_size);
                            } else {
                                defmt::info!("  - Endpoint {}: TX FIFO = {=u16} bytes, RX FIFO = (Not configured)", i, tx_size);
                            }
                        } else {
                            // This case handles when only RX is configured (tx_size is 0)
                            defmt::info!("  - Endpoint {}: TX FIFO = (Not configured), RX FIFO = {=u16} bytes", i, rx_size);
                        }
                    }
                }
                if !found_any {
                    defmt::warn!("  - No configured static FIFOs found for endpoints 1-15.");
                }
            }
        }

        // EPINFO Details
        defmt::info!("Endpoint Configuration (from EPINFO):");
        if self.epinfo_is_zero {
            defmt::warn!("  - The EPINFO register returned all zeros. It may be unimplemented or masked by the vendor.");
        }
        defmt::info!("  - Implemented TX Endpoints: {}", self.tx_endpoints);
        defmt::info!("  - Implemented RX Endpoints: {}", self.rx_endpoints);

        // RAMINFO Details
        defmt::info!("RAM and DMA Configuration (from RAMINFO):");
        if self.raminfo_is_zero {
            defmt::warn!("  - The RAMINFO register returned all zeros. It may be unimplemented or masked by the vendor.");
        }
        defmt::info!("  - Implemented DMA Channels: {}", self.dma_channels);
        defmt::info!("  - RAM Address Bus Width: {} bits", self.ram_bits);
        

        // LINKINFO Details
        defmt::info!("Timing and Delay Configuration (from LINKINFO):");
        if self.linkinfo_is_zero {
             // Default is 0x5C, so 0 is suspicious and worth a warning.
            defmt::warn!("  - The LINKINFO register returned all zeros. It may be unimplemented or masked by the vendor.");
        }
        let wtcon_us = (self.wtcon as f32) * 0.5333;
        let wtid_ms = (self.wtid as f32) * 4.369;
        defmt::info!("  - Connection Wait Time (WTCON): {} (~{=f32} us)", self.wtcon, wtcon_us);
        defmt::info!("  - ID Reading Wait Time (WTID): {} (~{=f32} ms)", self.wtid, wtid_ms);
        
        // VPLEN Details
        defmt::info!("VBUS Pulsing Charge Duration (VPLEN):");
        if self.vplen_is_zero {
            // Default is 0x3C
            defmt::warn!("  - The VPLEN register returned all zeros. It may be unimplemented or masked by the vendor.");
        }
        let vplen_ms = (self.vplen as f32) * 0.5461;
        defmt::info!("  - Value: {} (~{=f32} ms)", self.vplen, vplen_ms);
        

        // EOF Timing Details
        defmt::info!("End-of-Frame (EOF) Gap Configuration:");
        if self.hs_eof1_is_zero { // Default 0x80
             defmt::warn!("  - High-Speed (HS_EOF1): Register returned all zeros. May be unimplemented or masked.");
        }
        let hs_eof1_us = (self.hs_eof1 as f32) * 0.1333;
        defmt::info!("  - High-Speed (HS_EOF1): {} (~{=f32} us)", self.hs_eof1, hs_eof1_us);
        
        if self.fs_eof1_is_zero { // Default 0x77
            defmt::warn!("  - Full-Speed (FS_EOF1): Register returned all zeros. May be unimplemented or masked.");
        }
        let fs_eof1_us = (self.fs_eof1 as f32) * 0.5333;
        defmt::info!("  - Full-Speed (FS_EOF1): {} (~{=f32} us)", self.fs_eof1, fs_eof1_us);
        
        if self.ls_eof1_is_zero { // Default 0x72
             defmt::warn!("  - Low-Speed (LS_EOF1): Register returned all zeros. May be unimplemented or masked.");
        }
        let ls_eof1_us = (self.ls_eof1 as f32) * 1.067;
        defmt::info!("  - Low-Speed (LS_EOF1): {} (~{=f32} us)", self.ls_eof1, ls_eof1_us);
        

        defmt::info!("--- End of Configuration ---");
    }
}
