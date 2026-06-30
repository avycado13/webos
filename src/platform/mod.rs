#[cfg(target_arch = "x86_64")]
pub mod x86_64;

#[cfg(target_arch = "x86_64")]
pub use x86_64::console::_print;
#[cfg(target_arch = "x86_64")]
pub use x86_64::console::backspace;

#[cfg(target_arch = "wasm32")]
pub mod wasm32;

#[cfg(target_arch = "wasm32")]
pub use wasm32::canvas::_print;

#[cfg(target_arch = "wasm32")]
pub use wasm32::canvas::backspace;
