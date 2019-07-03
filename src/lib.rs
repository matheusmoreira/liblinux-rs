#![feature(asm)]

#[cfg(target_arch = "x86_64")]
pub fn system_call(number : usize, _1 : usize, _2 : usize, _3 : usize, _4 : usize, _5 : usize, _6 : usize) -> usize {
    let mut return_value;

    unsafe {
        asm!("syscall"
           : "={rax}" (return_value)
           : "{rax}" (number), "{rdi}" (_1), "{rsi}" (_2), "{rdx}" (_3), "{r10}" (_4), "{r8}" (_5), "{r9}" (_6)
           : "rcx", "r8", "r9", "r10" "r11", "cc", "memory"
           : "volatile"
           );
    }

    return_value
}
