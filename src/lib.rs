#![cfg_attr(target_arch = "x86_64", no_std)]
#![cfg_attr(all(test, target_arch = "x86_64"), no_main)]
#![feature(custom_test_frameworks)]
#![cfg_attr(target_arch = "x86_64", feature(abi_x86_interrupt))]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

#[cfg(target_arch = "x86_64")]
use core::panic::PanicInfo;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "x86_64")]
pub mod allocator;
pub mod fs;
pub mod platform;
pub mod shell;
pub mod task;

/// Prints to the console (VGA on x86, stub on other targets).
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::platform::_print(format_args!($($arg)*)));
}

/// Prints to the console with a trailing newline.
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

// On x86, serial and VGA both go through platform::_print (writes to both).
// These aliases exist so existing call sites don't need to change.
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => ($crate::platform::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(concat!($fmt, "\n"), $($arg)*));
}

#[cfg(target_arch = "x86_64")]
pub mod gdt;
#[cfg(target_arch = "x86_64")]
pub mod interrupts;
#[cfg(target_arch = "x86_64")]
pub mod memory;

#[cfg(target_arch = "x86_64")]
pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}
pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    #[cfg(target_arch = "x86_64")]
    exit_qemu(QemuExitCode::Success);
    #[cfg(not(target_arch = "x86_64"))]
    loop {}
}

pub fn test_panic_handler(info: &core::panic::PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    #[cfg(target_arch = "x86_64")]
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

#[cfg(target_arch = "x86_64")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

#[cfg(target_arch = "x86_64")]
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn wasm_start() {
    platform::wasm32::init_canvas();
    println!("WebOS v0.1.0");
    println!("Type 'help' for available commands.");
    println!();
    platform::wasm32::start_shell();
}

pub fn hlt_loop() -> ! {
    #[cfg(target_arch = "x86_64")]
    loop {
        x86_64::instructions::hlt();
    }
    #[cfg(not(target_arch = "x86_64"))]
    loop {}
}

#[cfg(all(test, target_arch = "x86_64"))]
use bootloader::{BootInfo, entry_point};

#[cfg(all(test, target_arch = "x86_64"))]
entry_point!(test_kernel_main);

#[cfg(all(test, target_arch = "x86_64"))]
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    init();
    test_main();
    hlt_loop();
}

#[cfg(all(test, target_arch = "x86_64"))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    test_panic_handler(info)
}
