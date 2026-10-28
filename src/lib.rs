#![no_std]

use core::panic::PanicInfo;
use probe_macro::sandbox_probe;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[sandbox_probe]
#[no_mangle]
pub extern "C" fn user_entrypoint() -> u32 {
    7
}
