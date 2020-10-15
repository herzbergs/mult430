#![no_main]
#![no_std]
#![feature(abi_msp430_interrupt)]

extern crate panic_msp430;

use core::cell::Cell;
use msp430::interrupt as mspint;
use once_cell::unsync::OnceCell;
use msp430_rt::entry;
use msp430g2231::{interrupt, TIMER_A2};
use msp430g2231::port_1_2::P1OUT;

use msp430_atomic::{AtomicU16, AtomicBool};

mod pulses;
use pulses::{SyncSignal, SyncKind};

// The compiler will emit calls to the abort() compiler intrinsic if debug assertions are
// enabled (default for dev profile). MSP430 does not actually have meaningful abort() support
// so for now, we create our own in each application where debug assertions are present.
#[no_mangle]
extern "C" fn abort() -> ! {
    panic!();
}

fn setup_sysclk(clk_hw : msp430g2231::SYSTEM_CLOCK) {
    // this is taken from datasheet for 21MHz operation
    clk_hw.bcsctl1.modify(|_, w| w.rsel0().set_bit()
                                  .rsel1().set_bit()
                                  .rsel2().set_bit()
                                  .rsel3().set_bit());

    clk_hw.dcoctl.modify(|_, w| w.dco0().set_bit()
                                 .dco1().set_bit()
                                 .dco2().set_bit());

    // a clk
    clk_hw.bcsctl3.modify(|_, w| w.lfxt1s().lfxt1s_2());
    clk_hw.bcsctl1.modify(|_, w| w.diva().diva_1());
}

fn setup_port(p12 : msp430g2231::PORT_1_2) {
    // set up P1.0 for output (start low)
    p12.p1out.modify(|_, w| w.p0().clear_bit());
    p12.p1dir.modify(|_, w| w.p0().set_bit());

    // set up p1.6 for CCR1B input
    p12.p1sel.modify(|_, w| w.p6().set_bit());
    p12.p1dir.modify(|_, w| w.p6().clear_bit());
}

fn setup_timer(timer : TIMER_A2) {
    // some dumb default for period register
    timer.taccr0.write(|w| unsafe { w.bits(0xff) });

    // aclk source and continuous mode
    timer.tactl.modify(|_, w|  w.tassel().tassel_1()
                                .mc().mc_2() );

    // rising edge capture mode
    // CCIxA capture input select
    // Asynchronus capture
    // capture mode set
    // capture compare interrupt enable
    timer.tacctl1.modify(|_, w| w.cm().cm_1()
                                 .ccis().ccis_0()
                                 .scs().clear_bit()
                                 .cap().set_bit()
                                 .ccie().set_bit());

    // compare interrupt enable
    timer.tacctl0.modify(|_, w| w.ccie().set_bit());
}

fn update_ui() {

}

#[entry]
fn main() -> ! {
    let hw = msp430g2231::Peripherals::take().unwrap();

    // Disable watchdog
    let wd = hw.WATCHDOG_TIMER;
    wd.wdtctl.write(|w| {
        unsafe { w.bits(0x5A00) } // password
        .wdthold().set_bit()
    });

    setup_sysclk(hw.SYSTEM_CLOCK);
    setup_port(hw.PORT_1_2);
    setup_timer(hw.TIMER_A2);

    unsafe { mspint::enable(); }

    loop {
        update_ui();
    }
}

// There are two timer interrupts, Capture Timer (TIMERA1) and Counter Timer (TIMERA0)
// Capture Timer is triggered by the external sync pin going high
// Counter Timer is triggered by the timer reaching a certain value
//
// Capture timer will:
//   - measure the external sync
//   - set the output high for output pulses that line up with an input pulse after division
//     but ignoring multiplication. I call this a hard sync.
//   - set up Counter Timer values for the next set of non hard sync outputs
//
// Counter Timer will:
//   - toggle the output pin
//   - set up Counter Timer values to produce non-50% duty cycles
//
// This could be done either with direct register writes or messages passed to main

// This interrupt fires when the external sync signal goes high
// if its a '0' tick need to set bit high
// and capture times for clock multiplication
#[interrupt]
fn TIMERA1() {

}

// toggle that darn bit, and then update or cause to be updated the timer value

// This interrupt is driven by calculations performed above, and produces
// the pulses that do NOT align exactly with the input signal
#[interrupt]
fn TIMERA0() {

}
