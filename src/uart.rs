use core::fmt::{Result, Write};

const REGISTER_BASE: *mut u8 = 0x1000_0000 as _;

pub struct Uart;

impl Uart {
    pub fn init() {
        unsafe {
            REGISTER_BASE.offset(3).write_volatile(0b11); // word size = 8
            REGISTER_BASE.offset(2).write_volatile(1); // enable fifo
        }
    }

    pub fn write(byte: u8) {
        unsafe {
            REGISTER_BASE.write_volatile(byte);
        }
    }
}

impl Write for Uart {
    fn write_str(&mut self, string: &str) -> Result {
        for byte in string.bytes() {
            Self::write(byte);
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! print
{
	($($args:tt)+) => ({
			use core::fmt::Write;
			let _ = write!($crate::uart::Uart, $($args)+);
	});
}
#[macro_export]
macro_rules! println
{
	() => ({
		print!("\n")
	});
	($fmt:expr) => ({
		print!(concat!($fmt, "\n"))
	});
	($fmt:expr, $($args:tt)+) => ({
		print!(concat!($fmt, "\n"), $($args)+)
	});
}
