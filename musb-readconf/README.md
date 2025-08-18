# `musb-readconf`

A `no_std` ~~and not-so-useful~~ utility crate for reading and interpreting the hardware configuration registers of a Mentor Graphics MUSB IP core.

This tool is designed to help embedded developers verify the specific hardware features and configuration of the MUSB IP as implemented by a particular chip vendor.

~~But, Useless.~~ Most Venders masked  several following registers like `CONFIGDATA`.

## Features

1. Reads MUSB configuration registers like `CONFIGDATA`, `EPINFO`, and `RAMINFO`.
2. Prints the decoded values in a human-readable format using the `defmt`.

3. Warns if register values are all-zero or contradict the official documentation, which may indicate vendor-specific modifications.

## Usage

Here is an example of how to use this library: TODO

### 1\. Implement the `MusbInstance` Trait

You need to create a "bridge" struct that connects your HAL's USB peripheral to the `MusbInstance` trait expected by the library.

```rust
use musb_readconf::{Configuration, MusbInstance, regs};

struct MyUsbInstance;
impl MusbInstance for MyUsbInstance {
    fn regs() -> regs::Usb {
        unsafe { *(0x5004_7000 as *const regs::Usb) }
    }
}
```

### 2\. Read and Print the Configuration

In your `main` function, initialize your hardware, then call the library to read and print the configuration.

```rust
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // ...

    defmt::info!("Reading MUSB IP Core Configuration...");
    let musb_config = Configuration::read::<MyUsbInstance>();

    // Print the detailed configuration report.
    musb_config.print_defmt();
}
```

## Example Output

Running the code will produce detailed output via `defmt`, clearly listing the MUSB core's features and highlighting potential issues.

```text
INFO --- MUSB Core Configuration ---
INFO Core Configuration (from CONFIGDATA):
INFO   - Automatic Bulk Packet Amalgamation (MPRxE): true
INFO   - Automatic Bulk Packet Splitting (MPTxE): true
INFO   - Endianness: Little Endian
INFO   - High-Bandwidth Rx ISO Endpoint Support (HBRxE): false
INFO   - High-Bandwidth Tx ISO Endpoint Support (HBTxE): false
INFO   - FIFO Sizing: Dynamic
INFO   - Connection Type: Soft Connect/Disconnect
INFO   - UTMI+ Data Width: Bit8
INFO Endpoint Configuration (from EPINFO):
INFO   - Implemented TX Endpoints: 5
INFO   - Implemented RX Endpoints: 5
INFO RAM and DMA Configuration (from RAMINFO):
WARN   - The RAMINFO register returned all zeros. It may be unimplemented or masked by the vendor.
INFO Timing and Delay Configuration (from LINKINFO):
INFO   - Connection Wait Time (WTCON): 12 (~6.399 us)
INFO   - ID Reading Wait Time (WTID): 12 (~52.428 ms)
INFO VBUS Pulsing Charge Duration (VPLEN):
INFO   - Value: 60 (~32.766 ms)
INFO End-of-Frame (EOF) Gap Configuration:
INFO   - High-Speed (HS_EOF1): 128 (~17.062 us)
INFO   - Full-Speed (FS_EOF1): 119 (~63.462 us)
INFO   - Low-Speed (LS_EOF1): 114 (~121.638 us)
INFO --- End of Configuration ---
```

## License

This project is under Apache License, Version 2.0 ([LICENSE](../LICENSE) or <http://www.apache.org/licenses/LICENSE-2.0>).