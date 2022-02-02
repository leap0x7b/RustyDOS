use core::arch::asm;
use core::fmt::{Error, Write};

pub struct Console;

impl Console {
    pub fn putc(c: u8) {
        unsafe {
            asm!(
                "int 10h",
                in("ax") u16::from_be_bytes([0x0e, c]),
                in("bh") 0u8,
            );
        }
    }
}

impl Write for Console {
    fn write_str(&mut self, out: &str) -> Result<(), Error> {
        for c in out.bytes() {
            Self::putc(c);
        }
        Ok(())
    }
}
