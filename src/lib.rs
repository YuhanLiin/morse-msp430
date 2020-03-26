#![no_std]

pub mod buffer;
pub mod morse;

use embedded_hal::{digital::v2::OutputPin, timer::CountDown};
use morse::Morse;
use nb::block;

const SHORT_INTERVAL: u16 = 1000;
const LONG_INTERVAL: u16 = SHORT_INTERVAL * 3;

#[inline]
pub fn blink_morse<T: CountDown, O: OutputPin>(c: u8, timer: &mut T, pin: &mut O)
where
    T::Time: From<u16>,
{
    for code in morse::byte_to_morse(c) {
        match code {
            Morse::Dot => {
                pin.set_high().ok();
                timer.start(SHORT_INTERVAL);
                block!(timer.wait()).ok();
            }

            Morse::Dash => {
                pin.set_high().ok();
                timer.start(LONG_INTERVAL);
                block!(timer.wait()).ok();
            }

            Morse::Space => {
                pin.set_low().ok();
                timer.start(SHORT_INTERVAL * 4); // Will wait for 3 after, so wait for 4 more for total of 7 after each word
                block!(timer.wait()).ok();
            }
        };
        pin.set_low().ok();
        timer.start(SHORT_INTERVAL);
        block!(timer.wait()).ok();
    }
    timer.start(SHORT_INTERVAL * 2); // Waited for 1 before, so wait 2 more for total of 3 after every byte
    block!(timer.wait()).ok();
}
