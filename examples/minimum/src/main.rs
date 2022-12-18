#![no_std]
#![no_main]
#![feature(asm_experimental_arch, alloc_error_handler)]

extern crate alloc;

use core::{alloc::Layout, fmt::Write, panic::PanicInfo};

use rosalina::{os::OS, DOLPHIN_HLE};

#[no_mangle]
extern "C" fn main() -> ! {
    let _os = OS::init();

    loop {}
}

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    write!(unsafe { &mut DOLPHIN_HLE }, "{}", info).ok();
    loop {}
}

#[alloc_error_handler]
fn alloc_handler(layout: Layout) -> ! {
    write!(
        unsafe { &mut DOLPHIN_HLE },
        "Failed to allocate item with \n Size: {}\n, Align: {}\n",
        layout.size(),
        layout.align()
    )
    .ok();
    loop {}
}
