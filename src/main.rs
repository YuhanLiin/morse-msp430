#![no_main]
#![no_std]
#![feature(abi_msp430_interrupt)]

use core::cell::RefCell;
use embedded_hal::digital::v2::*;
use embedded_hal::prelude::*;
use morse_msp430::{blink_morse, buffer, detect_morse, morse::FsmState};
use msp430::interrupt::{enable, free, CriticalSection, Mutex};
use msp430_rt::entry;
use msp430fr2x5x_hal::{
    clock::*, fram::Fram, gpio::Batch, pac, pac::interrupt, pmm::Pmm, rtc::Rtc, serial::*,
    watchdog::Wdt,
};
use panic_msp430 as _;

static UART: Mutex<RefCell<Option<(Tx<pac::E_USCI_A1>, Rx<pac::E_USCI_A1>)>>> =
    Mutex::new(RefCell::new(None));
pub static BUFFER: Mutex<RefCell<buffer::Buffer>> = Mutex::new(RefCell::new(buffer::Buffer::new()));

#[entry]
fn main(cs: CriticalSection) -> ! {
    let periph = pac::Peripherals::take().unwrap();

    Wdt::constrain(periph.WDT_A);

    let mut fram = Fram::new(periph.FRCTL);
    let aclk = ClockConfig::new(periph.CS)
        .mclk_dcoclk(DcoclkFreqSel::_8MHz, MclkDiv::_1)
        .smclk_off()
        .aclk_refoclk()
        .freeze(&mut fram);
    let pmm = Pmm::new(periph.PMM);
    let mut p4 = Batch::new(periph.P4)
        .config_pin1(|p| p.pullup())
        .split(&pmm);
    let p1 = Batch::new(periph.P1)
        .config_pin0(|p| p.to_output())
        .split(&pmm);
    let mut led = p1.pin0;
    let mut rtc = Rtc::new(periph.RTC);

    let (tx, mut rx) = SerialConfig::new(
        periph.E_USCI_A1,
        BitOrder::LsbFirst,
        BitCount::EightBits,
        StopBits::OneStopBit,
        // Launchpad UART-to-USB converter doesn't handle parity, so we don't use it
        Parity::NoParity,
        Loopback::NoLoop,
        9600,
    )
    .use_aclk(&aclk)
    .split(p4.pin3.to_alternate1(), p4.pin2.to_alternate1());

    //rx.enable_rx_interrupts();
    UART.borrow(&cs).replace(Some((tx, rx)));
    enable_safe(cs);

    let mut state = FsmState::Start;
    let mut button = p4.pin1;
    led.set_high().ok();
    loop {
        let (c, space) = detect_morse(&mut state, &mut rtc, &mut button, &mut led);
        if c != 0 {
            free(|cs| {
                let mut buffer = BUFFER.borrow(cs).borrow_mut();
                buffer.push(c).unwrap();
                if space {
                    buffer.push(b' ').unwrap();
                };
                UART.borrow(cs)
                    .borrow_mut()
                    .as_mut()
                    .unwrap()
                    .0
                    .enable_tx_interrupts();
            });
        }
        //if let Ok(c) = free(|cs| BUFFER.borrow(cs).borrow_mut().pop()) {
        //blink_morse(c, &mut rtc, &mut led);
        //}
    }
}

fn enable_safe(_cs: CriticalSection) {
    unsafe { enable() };
}

#[interrupt]
fn EUSCI_A1(cs: CriticalSection) {
    static mut CHAR: u8 = 0;

    let mut uart = UART.borrow(&cs).borrow_mut();
    let (ref mut tx, ref mut rx) = uart.as_mut().unwrap();

    let mut buf = BUFFER.borrow(&cs).borrow_mut();
    let c = buf.pop().unwrap();
    tx.write(c).ok();
    if buf.is_empty() {
        tx.disable_tx_interrupts()
    };
    //if let Ok(c) | Err(nb::Error::Other(RecvError::Overrun(c))) = rx.read() {
    //tx.enable_tx_interrupts();
    //*CHAR = c;
    //BUFFER.borrow(&cs).borrow_mut().push(c).unwrap();
    //}

    //if tx.write(*CHAR).is_ok() {
    //tx.disable_tx_interrupts();
    //}
}
