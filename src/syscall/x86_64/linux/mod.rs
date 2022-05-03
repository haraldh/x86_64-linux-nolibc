// SPDX-License-Identifier: Apache-2.0

//! This library was built for x86-64 Linux.

pub mod macros;
pub mod nr;

use core::arch::asm;

#[inline(always)]
pub unsafe fn syscall0(nr: isize) -> isize {
    let ret: isize;

    asm!(
    "syscall",
    inlateout("rax") nr => ret,
    lateout("rcx") _, // clobbered
    lateout("r11") _, // clobbered
    );

    ret
}

pub unsafe fn syscall1(nr: isize, a1: usize) -> isize {
    let ret: isize;

    asm!(
    "syscall",
    inlateout("rax") nr => ret,
    in("rdi") a1,
    lateout("rcx") _, // clobbered
    lateout("r11") _, // clobbered
    );

    ret
}

pub unsafe fn syscall2(nr: isize, a1: usize, a2: usize) -> isize {
    let ret: isize;

    asm!(
    "syscall",
    inlateout("rax") nr => ret,
    in("rdi") a1,
    in("rsi") a2,
    lateout("rcx") _, // clobbered
    lateout("r11") _, // clobbered
    );

    ret
}

pub unsafe fn syscall3(nr: isize, a1: usize, a2: usize, a3: usize) -> isize {
    let ret: isize;

    asm!(
    "syscall",
    inlateout("rax") nr => ret,
    in("rdi") a1,
    in("rsi") a2,
    in("rdx") a3,
    lateout("rcx") _, // clobbered
    lateout("r11") _, // clobbered
    );

    ret
}

pub unsafe fn syscall4(nr: isize, a1: usize, a2: usize, a3: usize, a4: usize) -> isize {
    let ret: isize;

    asm!(
    "syscall",
    inlateout("rax") nr => ret,
    in("rdi") a1,
    in("rsi") a2,
    in("rdx") a3,
    in("r10") a4,
    lateout("rcx") _, // clobbered
    lateout("r11") _, // clobbered
    );

    ret
}

pub unsafe fn syscall5(nr: isize, a1: usize, a2: usize, a3: usize, a4: usize, a5: usize) -> isize {
    let ret: isize;

    asm!(
    "syscall",
    inlateout("rax") nr => ret,
    in("rdi") a1,
    in("rsi") a2,
    in("rdx") a3,
    in("r10") a4,
    in("r8") a5,
    lateout("rcx") _, // clobbered
    lateout("r11") _, // clobbered
    );

    ret
}

pub unsafe fn syscall6(
    nr: isize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
) -> isize {
    let ret: isize;

    asm!(
    "syscall",
    inlateout("rax") nr => ret,
    in("rdi") a1,
    in("rsi") a2,
    in("rdx") a3,
    in("r10") a4,
    in("r8") a5,
    in("r9") a6,
    lateout("rcx") _, // clobbered
    lateout("r11") _, // clobbered
    );

    ret
}
