// SPDX-License-Identifier: Apache-2.0

//! Minimal std replacement without any libc
//!
//! Instead it uses x86_64 Linux syscalls, but can be taken as a template
//! to implement it for other targets without `std` support.

#![deny(warnings)]
#![no_std]

#[cfg(test)]
extern crate std;

pub mod io;
pub mod macros;
pub mod process;
pub mod syscall;
