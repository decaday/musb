#![allow(non_snake_case)]
#![allow(unused)]
#![allow(non_camel_case_types)]

#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

cfg_if::cfg_if! {
    if #[cfg(not(feature = "prebuild"))] {
        include!(concat!(env!("OUT_DIR"), "/usb_regs.rs"));
        pub mod common {
            include!(concat!(env!("OUT_DIR"), "/common.rs"));
        }
        pub mod info {
            include!(concat!(env!("OUT_DIR"), "/info.rs"));
        }
    }
    else {
        #[cfg(feature = "builtin-py32f07x")] 
        include!("prebuilds/py32f07x/usb_regs.rs");
        #[cfg(feature = "builtin-py32f07x")] 
        pub mod info {
            include!("prebuilds/py32f07x/info.rs");
        }
        

        #[cfg(feature = "builtin-py32f403")]
        include!("prebuilds/py32f403/usb_regs.rs");
        #[cfg(feature = "builtin-py32f403")]
        pub mod info {
            include!("prebuilds/py32f403/info.rs");
        }

        #[cfg(feature = "builtin-std")]
        include!("prebuilds/std/usb_regs.rs");
        #[cfg(feature = "builtin-std")]
        pub mod info {
            include!("prebuilds/std/info.rs");
        }

        pub mod common {
            include!("prebuilds/common.rs");
        }
    }
}
