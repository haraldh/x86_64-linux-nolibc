#![no_std]
#![no_main]
#![feature(naked_functions, asm_sym)]

use x86_64_linux_nolibc::process::exit;

rcrt1::x86_64_linux_startup!(
    fn _start() -> ! {
        exit(main())
    }
);

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    exit(255)
}

fn main() -> i32 {
    1
}
