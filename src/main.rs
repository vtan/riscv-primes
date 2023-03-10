#![no_std]
#![no_main]
#![feature(sync_unsafe_cell)]

mod mutex;
mod uart;

use core::{
    arch::{asm, global_asm},
    panic::PanicInfo,
    sync::atomic::{AtomicBool, Ordering},
};

use mutex::Mutex;
use uart::Uart;

global_asm!(include_str!("boot.s"));

static INITIALIZED: AtomicBool = AtomicBool::new(false);
static NEXT_CANDIDATE: Mutex<u64> = Mutex::new((1 << 63) - 1);

#[no_mangle]
extern "C" fn main(cpu_index: i32, stack_top: u64) {
    Uart::init();
    println!("Initialized");
    INITIALIZED.store(true, Ordering::SeqCst);

    println!("[{}] Stack top is {:x}", cpu_index, stack_top);

    search_primes(cpu_index);
}

#[no_mangle]
extern "C" fn main_secondary(cpu_index: i32, stack_top: u64) {
    while !INITIALIZED.load(Ordering::SeqCst) {
        core::hint::spin_loop()
    }
    println!("[{}] Stack top is {:x}", cpu_index, stack_top);

    search_primes(cpu_index);
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
    'next_candidate: loop {
        let candidate = {
            let mut next = NEXT_CANDIDATE.lock();
            let candidate = *next;
            *next += 2;
            candidate
        };
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
