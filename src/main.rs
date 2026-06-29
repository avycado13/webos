#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(webos::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

#[cfg(target_arch = "x86_64")]
use core::panic::PanicInfo;
#[cfg(target_arch = "x86_64")]
use webos::println;


#[cfg(target_arch = "x86_64")]
use webos::task::{Task, executor::Executor};
#[cfg(target_arch = "x86_64")]
use bootloader::{BootInfo, entry_point};

#[cfg(target_arch = "x86_64")]
entry_point!(kernel_main);

#[cfg(target_arch = "x86_64")]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use webos::allocator;
    use webos::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    println!("Hello World{}", "!");
    webos::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    #[cfg(test)]
    test_main();

    // filesystem demo
    {
        use webos::fs::FS;
        let mut fs = FS.lock();
        fs.create("/hello.txt", b"hello from ramdisk!".to_vec());
        fs.create("/boot/config", b"verbose=1\n".to_vec());
        drop(fs);

        let fs = FS.lock();
        if let Some(data) = fs.read("/hello.txt") {
            if let Ok(s) = core::str::from_utf8(data) {
                println!("read /hello.txt: {}", s);
            }
        }
        println!("files:");
        for name in fs.list() {
            println!("  {}", name);
        }
    }

    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(webos::shell::run()));
    executor.run();
}



#[cfg(target_arch = "x86_64")]
async fn async_number() -> u32 {
    42
}

#[cfg(target_arch = "x86_64")]
async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}


#[cfg(all(not(test), target_arch = "x86_64"))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    webos::hlt_loop();
}

#[cfg(all(test, target_arch = "x86_64"))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    webos::test_panic_handler(info)
}


#[cfg(target_arch = "x86_64")]
#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
