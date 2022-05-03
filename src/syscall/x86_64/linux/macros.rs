// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_export]
macro_rules! linux_syscall {
    ($nr:expr) => {
        $crate::syscall::os::linux::syscall0($nr as isize)
    };

    ($nr:expr, $a1:expr) => {
        $crate::syscall::os::linux::syscall1($nr as isize, $a1 as usize)
    };

    ($nr:expr, $a1:expr, $a2:expr) => {
        $crate::syscall::os::linux::syscall2($nr as isize, $a1 as usize, $a2 as usize)
    };

    ($nr:expr, $a1:expr, $a2:expr, $a3:expr) => {
        $crate::syscall::os::linux::syscall3($nr as isize, $a1 as usize, $a2 as usize, $a3 as usize)
    };

    ($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr) => {
        $crate::syscall::os::linux::syscall4(
            $nr as isize,
            $a1 as usize,
            $a2 as usize,
            $a3 as usize,
            $a4 as usize,
        )
    };

    ($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr) => {
        $crate::syscall::os::linux::syscall5(
            $nr as isize,
            $a1 as usize,
            $a2 as usize,
            $a3 as usize,
            $a4 as usize,
            $a5 as usize,
        )
    };

    ($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {
        $crate::syscall::os::linux::syscall6(
            $nr as isize,
            $a1 as usize,
            $a2 as usize,
            $a3 as usize,
            $a4 as usize,
            $a5 as usize,
            $a6 as usize,
        )
    };

    ($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr) => {
        $crate::syscall::os::linux::syscall7(
            $nr as isize,
            $a1 as usize,
            $a2 as usize,
            $a3 as usize,
            $a4 as usize,
            $a5 as usize,
            $a6 as usize,
            $a7 as usize,
        )
    };
}
