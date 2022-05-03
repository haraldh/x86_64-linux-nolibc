#![no_std]
#![no_main]
#![feature(naked_functions, asm_sym)]

use x86_64_linux_nolibc as std;

use std::eprintln;
use std::process::{exit, Termination};

rcrt1::x86_64_linux_startup!(
    fn _start() -> ! {
        exit(main().report().to_i32())
    }
);

#[panic_handler]
fn panic(info: &core::panic::PanicInfo<'_>) -> ! {
    eprintln!("{}\n", info);
    exit(255)
}

fn main() -> Result<(), i32> {
    panic!("Hello World!");
}
