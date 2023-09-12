#![no_std]
#![no_main]

use core::fmt::Write;

extern "C" {
    fn printk(fmt: *const core::ffi::c_char, ...) -> core::ffi::c_int;
    fn _exit(code: i32) -> !;
}

/// Represents the kernel debug output stream in RTEMS.
///
/// Uses the `printk` C function to print text.
struct Console;

impl core::fmt::Write for Console {
    fn write_str(&mut self, message: &str) -> core::fmt::Result {
        const FORMAT_STR: &core::ffi::CStr = {
            let Ok(s) = core::ffi::CStr::from_bytes_with_nul(b"%.*s\0") else {
                panic!()
            };
            s
        };
        if message.len() != 0 {
            unsafe {
                printk(FORMAT_STR.as_ptr(), message.len() as core::ffi::c_int, message.as_ptr());
            }
        }
        Ok(())
    }
}

/// A C-shim which the rtems demo code knows how to call on start-up.
///
/// Just jumps to [`rust_main`].
#[no_mangle]
pub extern "C" fn rust_entry() -> i32 {
    if let Err(e) = rust_main() {
        panic!("Main returned {:?}", e);
    } else {
        0
    }
    
}

/// The main function for our Rust program
fn rust_main() -> Result<(), core::fmt::Error> {
    let mut console = Console;
    writeln!(console, "Hello, this is Rust!")?;
    write!(console, "    ")?;
    for y in 0..10 {
        write!(console, "{:2} ", y)?;
    }
    writeln!(console)?;
    for x in 0..10 {
        write!(console, "{:2}: ", x)?;
        for y in 0..10 {
            write!(console, "{:2} ", x * y)?;
        }
        writeln!(console)?;
    }
    Ok(())
}

/// Called when a panic occurs.
#[panic_handler]
fn panic(panic: &core::panic::PanicInfo) -> ! {
    let mut console = Console;
    let _ = writeln!(console, "PANIC: {:?}", panic);
    unsafe {
        _exit(1);
    }
}

// End of file
