#[cfg(target_arch = "x86_64")]
pub mod x86_64;

#[cfg(target_arch = "x86_64")]
pub use x86_64::console::_print;
#[cfg(target_arch = "x86_64")]
pub use x86_64::console::backspace;

#[cfg(target_arch = "wasm32")]
pub mod wasm32;

#[cfg(target_arch = "wasm32")]
pub fn _print(args: core::fmt::Arguments) {
    use alloc::string::String;
    use core::fmt::Write;
    let mut s = String::new();
    let _ = s.write_fmt(args);
    wasm32::log(&s);
}
