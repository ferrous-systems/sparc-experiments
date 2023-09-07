#![no_std]
#![no_main]

use core::fmt::Write;

extern "C" {
    fn putchar(ch: i32);
    fn _exit(code: i32) -> !;
}

/// Represents the standard-output available in tsim.
///
/// Uses the `putchar` C function to print text.
struct Console;

impl core::fmt::Write for Console {
    fn write_str(&mut self, message: &str) -> core::fmt::Result {
        for b in message.bytes() {
            unsafe {
                putchar(b as i32);
            }
        }
        Ok(())
    }
}

/// A C-shim which libc knows how to call on start-up.
///
/// Just jumps to [`rust_main`].
#[no_mangle]
pub extern "C" fn main() -> i32 {
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
    panic!("I am a panic");
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
