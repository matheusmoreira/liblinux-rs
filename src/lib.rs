#![no_std]
#![feature(asm)]

#[cfg(target_arch = "x86_64")]
pub unsafe fn system_call_0(number: usize) -> usize {
    let mut return_value;

    asm!("syscall"
       : "={rax}" (return_value)
       : "{rax}" (number)
       : "rcx", "r8", "r9", "r10" "r11", "cc", "memory"
       : "volatile"
       );

    return_value
}

#[cfg(target_arch = "x86_64")]
pub unsafe fn system_call_1(number: usize, _1: usize) -> usize {
    let mut return_value;

    asm!("syscall"
       : "={rax}" (return_value)
       : "{rax}" (number), "{rdi}" (_1)
       : "rcx", "r8", "r9", "r10" "r11", "cc", "memory"
       : "volatile"
       );

    return_value
}

#[cfg(target_arch = "x86_64")]
pub unsafe fn system_call_2(number: usize, _1: usize, _2: usize) -> usize {
    let mut return_value;

    asm!("syscall"
       : "={rax}" (return_value)
       : "{rax}" (number), "{rdi}" (_1), "{rsi}" (_2)
       : "rcx", "r8", "r9", "r10" "r11", "cc", "memory"
       : "volatile"
       );

    return_value
}

#[cfg(target_arch = "x86_64")]
pub unsafe fn system_call_3(number: usize, _1: usize, _2: usize, _3: usize) -> usize {
    let mut return_value;

    asm!("syscall"
       : "={rax}" (return_value)
       : "{rax}" (number), "{rdi}" (_1), "{rsi}" (_2), "{rdx}" (_3)
       : "rcx", "r8", "r9", "r10" "r11", "cc", "memory"
       : "volatile"
       );

    return_value
}

#[cfg(target_arch = "x86_64")]
pub unsafe fn system_call_4(number: usize, _1: usize, _2: usize, _3: usize, _4: usize) -> usize {
    let mut return_value;

    asm!("syscall"
       : "={rax}" (return_value)
       : "{rax}" (number), "{rdi}" (_1), "{rsi}" (_2), "{rdx}" (_3), "{r10}" (_4)
       : "rcx", "r8", "r9", "r10" "r11", "cc", "memory"
       : "volatile"
       );

    return_value
}

#[cfg(target_arch = "x86_64")]
pub unsafe fn system_call_5(number: usize, _1: usize, _2: usize, _3: usize, _4: usize, _5: usize) -> usize {
    let mut return_value;

    asm!("syscall"
       : "={rax}" (return_value)
       : "{rax}" (number), "{rdi}" (_1), "{rsi}" (_2), "{rdx}" (_3), "{r10}" (_4), "{r8}" (_5)
       : "rcx", "r8", "r9", "r10" "r11", "cc", "memory"
       : "volatile"
       );

    return_value
}

#[cfg(target_arch = "x86_64")]
pub unsafe fn system_call_6(number: usize, _1: usize, _2: usize, _3: usize, _4: usize, _5: usize, _6: usize) -> usize {
    let mut return_value;

    asm!("syscall"
       : "={rax}" (return_value)
       : "{rax}" (number), "{rdi}" (_1), "{rsi}" (_2), "{rdx}" (_3), "{r10}" (_4), "{r8}" (_5), "{r9}" (_6)
       : "rcx", "r8", "r9", "r10" "r11", "cc", "memory"
       : "volatile"
       );

    return_value
}

macro_rules! system_call {
    ($number:expr) => (
        system_call_0($number);
    );

    ($number:expr, $_1:expr) => (
        system_call_1($number, $_1);
    );

    ($number:expr, $_1:expr, $_2:expr) => (
        system_call_2($number, $_1, $_2);
    );

    ($number:expr, $_1:expr, $_2:expr, $_3:expr) => (
        system_call_3($number, $_1, $_2, $_3);
    );

    ($number:expr, $_1:expr, $_2:expr, $_3:expr, $_4:expr) => (
        system_call_4($number, $_1, $_2, $_3, $_4);
    );

    ($number:expr, $_1:expr, $_2:expr, $_3:expr, $_4:expr, $_5:expr) => (
        system_call_5($number, $_1, $_2, $_3, $_4, $_5);
    );

    ($number:expr, $_1:expr, $_2:expr, $_3:expr, $_4:expr, $_5:expr, $_6:expr) => (
        system_call_6($number, $_1, $_2, $_3, $_4, $_5, $_6);
    );
}
