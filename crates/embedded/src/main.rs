#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget generally not safe"
)]
#![deny(clippy::large_stack_frames)]

extern crate alloc;

use alloc::string::String;
use esp_hal::{
    clock::CpuClock,
    main,
    time::{Duration, Instant},
    uart::{Config as UartConfig, Uart},
};
use eta_core::basic;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! { loop {} }

esp_bootloader_esp_idf::esp_app_desc!();

fn sleep_ms(ms: u64) {
    let t0 = Instant::now();
    while t0.elapsed() < Duration::from_millis(ms) {}
}

fn write_all(uart: &mut Uart<'static, esp_hal::Blocking>, mut buf: &[u8]) {
    while !buf.is_empty() {
        match uart.write(buf) {
            Ok(0) => sleep_ms(1), /* full? */
            Ok(n) => buf = &buf[n..],
            Err(_) => sleep_ms(1) /* idk bro */
        }
    }
}

#[allow(
    clippy::large_stack_frames,
    reason = "main good for larger buffer allocs"
)]
#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    /* reclaim boot 2 ram as heap */
    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 139264);

    let mut uart = Uart::new(
        peripherals.UART0,
        UartConfig::default().with_baudrate(115_200),
    )
    .unwrap() /* else we dont care */
    .with_tx(peripherals.GPIO43)
    .with_rx(peripherals.GPIO44);

    write_all(&mut uart, b"\r\nEta Calculus on Embedded device!\r\n> ");

    let mut input = String::new();
    let mut output = String::new();
    input.reserve(256);
    output.reserve(256);

    let mut rx = [0u8; 64];

    loop {
        let n = match uart.read(&mut rx) {
            Ok(n) => n,
            Err(_) => 0,
        };

        if n == 0 {
            sleep_ms(1);
            continue;
        }

        for b in rx[..n].iter().copied() {
            match b {
                // Submit line
                b'\r' | b'\n' => {
                    write_all(&mut uart, b"\r\n");

                    output.clear();
                    basic::execute(&mut output, input.chars().into_iter());

                    write_all(&mut uart, output.as_bytes());
                    write_all(&mut uart, b"\r\n> ");

                    input.clear();
                }

                /* bksp or del */
                0x08 | 0x7F => {
                    if !input.is_empty() {
                        input.pop();
                        /* serial term erase */
                        write_all(&mut uart, b"\x08 \x08");
                    }
                }

                /* tab or ascii */
                b'\t' | 0x20..=0x7E => {
                    input.push(b as char);
                    write_all(&mut uart, &[b]); // local echo
                }

                /* control bytes */
                _ => {}
            }
        }
    }
}
