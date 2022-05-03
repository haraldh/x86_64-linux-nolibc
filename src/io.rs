// SPDX-License-Identifier: Apache-2.0

//! Traits, helpers, and type definitions for core I/O functionality.

use crate::linux_syscall;
use crate::syscall::os::linux::nr::WRITE;

use core::fmt;
use core::result::Result;

#[inline]
pub fn write(fd: i32, buf: *const u8, count: usize) -> Result<isize, i32> {
    #[allow(non_upper_case_globals)]
    let ret = unsafe { linux_syscall!(WRITE, fd, buf, count) };

    if ret >= 0 {
        Ok(ret)
    } else {
        Err(-ret as i32)
    }
}

struct StdIO<const FD: i32>;

impl<const FD: i32> fmt::Write for StdIO<FD> {
    #[inline(always)]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write(FD, s.as_ptr(), s.len()).map_err(|_| fmt::Error)?;
        Ok(())
    }
}

#[doc(hidden)]
#[inline(always)]
pub fn _eprint(args: fmt::Arguments<'_>) {
    use core::fmt::Write;
    StdIO::<2>.write_fmt(args).unwrap();
}

#[doc(hidden)]
#[inline(always)]
pub fn _print(args: fmt::Arguments<'_>) {
    use core::fmt::Write;
    StdIO::<1>.write_fmt(args).unwrap();
}
