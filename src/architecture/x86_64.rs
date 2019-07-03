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
