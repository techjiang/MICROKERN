#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

use microkern_kernel::memory::BuddyAllocator;

const COM1: u16 = 0x3f8;
const PAGE_SIZE: usize = 4096;
const HEAP_FRAMES: usize = 16;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut heap = [0u8; PAGE_SIZE * HEAP_FRAMES];
    let mut allocator = match BuddyAllocator::new(heap.as_mut_ptr() as usize, heap.len(), 4) {
        Ok(allocator) => allocator,
        Err(_) => halt(),
    };

    serial_write_str("MICROKERN v1.0.0: boot ok\n");

    if let Some(frame) = allocator.allocate(0) {
        serial_write_str("MICROKERN: first frame allocated\n");
        allocator.free(frame, 0);
    }

    halt()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_write_str("MICROKERN PANIC: ");
    if let Some(message) = info.message() {
        let _ = core::fmt::write(&mut SerialWriter, *message);
    }
    serial_write_str("\n");
    halt()
}

fn serial_write_str(value: &str) {
    for byte in value.bytes() {
        serial_write_byte(byte);
    }
}

fn serial_write_byte(byte: u8) {
    unsafe {
        while (inb(COM1 + 5) & 0x20) == 0 {
            core::hint::spin_loop();
        }
        outb(COM1, byte);
    }
}

unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    asm!("in al, dx", in("dx") port, out("al") value, options(nostack, preserves_flags));
    value
}

unsafe fn outb(port: u16, value: u8) {
    asm!("out dx, al", in("dx") port, in("al") value, options(nostack, preserves_flags));
}

fn halt() -> ! {
    loop {
        unsafe { asm!("hlt", options(nomem, nostack)) };
    }
}

struct SerialWriter;

impl core::fmt::Write for SerialWriter {
    fn write_str(&mut self, value: &str) -> core::fmt::Result {
        serial_write_str(value);
        Ok(())
    }
}
