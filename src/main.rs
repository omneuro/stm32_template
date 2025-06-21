#![no_std]
#![no_main]

// We need to use cortex_m here so the linker can find it's critical-section, but we import is as
// an underscore since we don't directly use any of its symbols.
use cortex_m as _;

// The entry import gives us a macro to indicate where the microcontroller should start the
// program, and also sets up the FPU on our Cortex-M4F.
use cortex_m_rt::entry;

// Provides functions that allow printing back to probe-rs.
use rtt_target::{rtt_init_print, rprintln};
use panic_halt as _;
use panic_rtt_target as _;

// Start here, and don't ever return from this function.
#[entry]
fn main() -> ! {

    // Do nothing, forever.
    loop {}
}
