// SPDX-License-Identifier: Apache-2.0

//! A module for working with processes.

use crate::eprintln;

use core::arch::asm;
use core::fmt;

/// Termination
pub trait Termination {
    /// Is called to get the representation of the value as status code.
    /// This status code is returned to the operating system.
    fn report(self) -> ExitCode;
}

impl Termination for () {
    #[inline]
    fn report(self) -> ExitCode {
        ExitCode::SUCCESS.report()
    }
}

impl Termination for ExitCode {
    #[inline]
    fn report(self) -> ExitCode {
        self
    }
}

impl<E: fmt::Debug> Termination for Result<(), E> {
    fn report(self) -> ExitCode {
        match self {
            Ok(()) => ().report(),
            Err(err) => {
                eprintln!("Error: {:?}", err);
                ExitCode::FAILURE.report()
            }
        }
    }
}

/// The ExitCode
pub struct ExitCode(i32);

impl ExitCode {
    pub const SUCCESS: ExitCode = ExitCode(0);
    pub const FAILURE: ExitCode = ExitCode(-1);
}

impl ExitCode {
    #[inline]
    pub fn to_i32(self) -> i32 {
        self.0
    }
}

impl From<u8> for ExitCode {
    /// Construct an exit code from an arbitrary u8 value.
    fn from(code: u8) -> Self {
        ExitCode(code as _)
    }
}

#[inline]
pub fn exit(status: i32) -> ! {
    #[allow(non_upper_case_globals)]
    const SYS_exit: isize = 60;

    unsafe {
        asm!(
            "syscall",
            in("rax") SYS_exit,
            in("rdi") status,
            options(noreturn),
        )
    }
}
