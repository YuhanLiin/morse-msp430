#![no_main]
#![no_std]

use msp430_rt::entry;
use msp430fr2x5x_hal as _;
use panic_msp430 as _;

#[entry]
fn main() -> ! {
    loop {}
}
