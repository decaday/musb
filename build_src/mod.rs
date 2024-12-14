cfg_if::cfg_if! {
    if #[cfg(not(feature = "prebuild"))] {
        pub mod build_serde;
        pub mod fieldset;
        pub mod block;
        pub mod profile;
        pub mod gen;
    }
}
pub mod feature;