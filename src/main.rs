#![no_std]
#![no_main]

mod uart;

use core::{
    arch::{asm, global_asm},
    panic::PanicInfo,
};

use uart::Uart;

global_asm!(include_str!("boot.s"));

#[no_mangle]
extern "C" fn main() {
    Uart::init();
    search_primes(0);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        loop {
            asm!("wfi");
        }
    }
}

fn shutdown() {
    unsafe {
        core::ptr::write_volatile(0x10_0000 as *mut u32, 0x5555_u32);
    }
}

fn search_primes(cpu_index: i32) {
    let search_start = (1 << 47) - 1;
    'next_candidate: for candidate in (search_start..).step_by(2) {
        for divisor in 3..=approx_sqrt(candidate) {
            if candidate % divisor == 0 {
                continue 'next_candidate;
            }
        }

        println!("[{}] {}", cpu_index, candidate);
    }
}

fn approx_sqrt(n: u64) -> u64 {
    let mut x0 = n / 2;
    let mut x1 = (x0 + n / x0) / 2;
    while x1 < x0 {
        x0 = x1;
        x1 = (x0 + n / x0) / 2;
    }
    x0
}
