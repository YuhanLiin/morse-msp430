#![no_std]

pub mod buffer;
pub mod morse;

use embedded_hal::{
    digital::v2::{InputPin, OutputPin},
    timer::CountDown,
};
use morse::{FsmState, Morse};
use msp430fr2x5x_hal::{
    gpio::{Input, IntrPortNum, Pin, PinNum, Pullup},
    rtc::{Rtc, RtcClockSrc},
};
use nb::block;
use void::ResultVoidExt;

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

pub fn detect_morse<R: RtcClockSrc, P: IntrPortNum, N: PinNum>(
    state: &mut FsmState,
    rtc: &mut Rtc<R>,
    input: &mut Pin<P, N, Input<Pullup>>,
) -> (u8, bool) {
    let mut c = 0;
    let mut pressed = input.is_low().void_unwrap();
    let mut space = false;
    if pressed {
        input.select_rising_edge_trigger(); // Wait for release
    } else {
        input.select_falling_edge_trigger(); // Wait for press
    }

    let mut iters = 0;
    let mut last_transition = 0;
    loop {
        rtc.start(SHORT_INTERVAL);

        loop {
            if rtc.wait().is_ok() {
                break;
            } else if input.wait_for_ifg().is_ok() {
                let diff = iters - last_transition;
                last_transition = iters;

                let morse = determine_morse_code(diff);
                if pressed {
                    match morse {
                        Morse::Dot => c = state.next(Morse::Dot),
                        Morse::Dash | Morse::Space => c = state.next(Morse::Dash),
                    }
                } else {
                    match morse {
                        Morse::Dash | Morse::Space => c = state.next(Morse::Space),
                        _ => {}
                    }
                }
                if c != 0 {
                    return (c, space);
                }

                pressed = !pressed;
                if pressed {
                    input.select_rising_edge_trigger(); // Wait for release
                } else {
                    input.select_falling_edge_trigger(); // Wait for press
                }
            }
        }

        iters += 1;
        if iters - last_transition > 7 {
            let m = if pressed {
                Morse::Dash
            } else {
                space = true;
                Morse::Space
            };
            c = state.next(m);
            break;
        }
    }
    (c, space)
}

fn determine_morse_code(diff: u16) -> Morse {
    if diff <= 3 {
        Morse::Dot
    } else if diff <= 7 {
        Morse::Dash
    } else {
        Morse::Space
    }
}
