#![cfg(target_os = "linux")]
#![no_std]
#![feature(asm)]

pub mod architecture;

#[macro_export]
macro_rules! system_call {
    ($number:expr) => (
        ::linux::architecture::target::system_call_0($number);
    );

    ($number:expr, $_1:expr) => (
        ::linux::architecture::target::system_call_1($number, $_1);
    );

    ($number:expr, $_1:expr, $_2:expr) => (
        ::linux::architecture::target::system_call_2($number, $_1, $_2);
    );

    ($number:expr, $_1:expr, $_2:expr, $_3:expr) => (
        ::linux::architecture::target::system_call_3($number, $_1, $_2, $_3);
    );

    ($number:expr, $_1:expr, $_2:expr, $_3:expr, $_4:expr) => (
        ::linux::architecture::target::system_call_4($number, $_1, $_2, $_3, $_4);
    );

    ($number:expr, $_1:expr, $_2:expr, $_3:expr, $_4:expr, $_5:expr) => (
        ::linux::architecture::target::system_call_5($number, $_1, $_2, $_3, $_4, $_5);
    );

    ($number:expr, $_1:expr, $_2:expr, $_3:expr, $_4:expr, $_5:expr, $_6:expr) => (
        ::linux::architecture::target::system_call_6($number, $_1, $_2, $_3, $_4, $_5, $_6);
    );
}
