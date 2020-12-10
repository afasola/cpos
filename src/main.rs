#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Questa linea e' stampata con la mia println! macro --> {}", "println!(\"Questa linea e' stampata con la mia println! macro --> {}\", \"\");");
    panic!("Spacco tutto!!!! {}", "panic!(\"Spacco tutto!!!! {}\", \"\");");
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

