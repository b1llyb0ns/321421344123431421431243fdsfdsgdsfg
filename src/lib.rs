#![no_std]

use core::panic::PanicInfo;
use probe_macro::sandbox_probe as _sandbox_probe_not_invoked;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn user_entrypoint() -> u32 {
    7
}
