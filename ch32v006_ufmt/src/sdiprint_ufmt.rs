mod regs {
    pub const DEBUG_DATA0_ADDRESS: *mut u32 = 0xE00000F4 as *mut u32;
    pub const DEBUG_DATA1_ADDRESS: *mut u32 = 0xE00000F8 as *mut u32;
}

pub struct SDIPrintUfmt;

impl SDIPrintUfmt {
    pub fn enable() {
        unsafe {
            // Enable SDI print
            core::ptr::write_volatile(regs::DEBUG_DATA0_ADDRESS, 0);
            qingke::riscv::asm::delay(100000);
        }
    }

    #[inline]
    fn is_busy() -> bool {
        unsafe { core::ptr::read_volatile(regs::DEBUG_DATA0_ADDRESS) != 0 }
    }

    fn write_bytes(&mut self, s: &[u8]) {
        let mut data = [0u8; 8];
        for chunk in s.chunks(7) {
            data[1..chunk.len() + 1].copy_from_slice(chunk);
            data[0] = chunk.len() as u8;

            // data1 is the last 4 bytes of data
            let data1 = u32::from_le_bytes(data[4..].try_into().unwrap());
            let data0 = u32::from_le_bytes(data[..4].try_into().unwrap());

            while SDIPrintUfmt::is_busy() {}

            unsafe {
                core::ptr::write_volatile(regs::DEBUG_DATA1_ADDRESS, data1);
                core::ptr::write_volatile(regs::DEBUG_DATA0_ADDRESS, data0);
            }
        }
    }

    fn write_str(&mut self, s: &str) {
        self.write_bytes(s.as_bytes())
    }
}

impl core::fmt::Write for SDIPrintUfmt {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_str(s);
        Ok(())
    }
}

#[derive(Debug)]
pub struct SDIError {}

impl ufmt::uWrite for SDIPrintUfmt {
    type Error = SDIError;

    #[inline]
    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        // match self.write_str(s) {
        //     Ok(_) => Ok(()),
        //     Err(_) => Err(SDIError{}),
        // }
        self.write_str(s);
        Ok(())
    }

    #[inline]
    fn write_char(&mut self, ch: char) -> Result<(), Self::Error> {
        let mut buffer = [0u8; 4];
        // match self.write_bytes(ch.encode_utf8(&mut buffer).as_bytes()){
        //     Ok(_) => Ok(()),
        //     Err(_) => Err(SDIError{}),
        // }
        self.write_bytes(ch.encode_utf8(&mut buffer).as_bytes());
        Ok(())
    }
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            use core::writeln;

            writeln!(&mut SDIPrintUfmt, $($arg)*).unwrap();
        }
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            use core::write;

            write!(&mut SDIPrintUfmt, $($arg)*).unwrap();
        }
    }
}

#[macro_export]
macro_rules! uprintln {
    ($($arg:tt)*) => {
        {
            ufmt::uwriteln!(&mut SDIPrintUfmt, $($arg)*).unwrap();
        }
    }
}

#[macro_export]
macro_rules! uprint {
    ($($arg:tt)*) => {
        {
            ufmt::uwrite!(&mut SDIPrintUfmt, $($arg)*).unwrap();
        }
    }
}
