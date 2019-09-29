// TODO: Removed extern crate (Rust 2018)
// extern crate gfx_hal as hal;

#[cfg(feature = "dx11")]
pub use gfx_backend_dx11::Instance;

#[cfg(feature = "dx12")]
pub use gfx_backend_dx12::Instance;

#[cfg(feature = "metal")]
pub use gfx_backend_metal::Instance;

#[cfg(feature = "vulkan")]
pub use gfx_backend_vulkan::Instance;

#[cfg(not(any(
    feature = "dx11",
    feature = "dx12",
    feature = "metal",
    feature = "vulkan"
)))]
compile_error!("No backend provided. Use `--feature [dx11|dx12|metal|vulkan]` to select your given backend");