#![allow(non_snake_case)]
#![allow(unused)]
#![allow(non_camel_case_types)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

cfg_if::cfg_if! {
    if #[cfg(not(feature = "prebuild"))] {
        pub mod regs {
            include!(concat!(env!("OUT_DIR"), "/regs.rs"));
        }
        pub mod common {
            include!(concat!(env!("OUT_DIR"), "/common.rs"));
        }
        include!(concat!(env!("OUT_DIR"), "/_generated.rs"));
    }
    else {
        pub mod common {
            include!("prebuilds/common.rs");
        }
        
        #[cfg(feature = "builtin-py32f07x")]
        pub mod regs {
            include!("prebuilds/py32f07x/regs.rs");
        }
        #[cfg(feature = "builtin-py32f07x")]
        include!("prebuilds/py32f07x/_generated.rs");

        #[cfg(feature = "builtin-sf32lb52x")]
        pub mod regs {
            include!("prebuilds/sf32lb52x/regs.rs");
        }
        #[cfg(feature = "builtin-sf32lb52x")]
        include!("prebuilds/sf32lb52x/_generated.rs");

        #[cfg(feature = "builtin-py32f403")]
        pub mod regs {
            include!("prebuilds/py32f403/regs.rs");
        }
        #[cfg(feature = "builtin-py32f403")]
        include!("prebuilds/py32f403/_generated.rs");

        #[cfg(feature = "builtin-std-8bep-2048")]
        pub mod regs {
            include!("prebuilds/std-8bep-2048/regs.rs");
        }
        #[cfg(feature = "builtin-std-8bep-2048")]
        include!("prebuilds/std-8bep-2048/_generated.rs");

        #[cfg(feature = "builtin-readconf")]
        pub mod regs {
            include!("prebuilds/readconf/regs.rs");
        }
        #[cfg(feature = "builtin-readconf")]
        include!("prebuilds/readconf/_generated.rs");
    }
}
