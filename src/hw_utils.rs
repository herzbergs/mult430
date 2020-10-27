use msp430g2231::{SYSTEM_CLOCK, PORT_1_2, TIMER_A2};


pub fn setup_sysclk(clk_hw : &SYSTEM_CLOCK) {
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

pub fn setup_port(p12 : &PORT_1_2) {
    // set up P1.0 for output (start low)
    p12.p1out.modify(|_, w| w.p0().clear_bit());
    p12.p1dir.modify(|_, w| w.p0().set_bit());

    // set up p1.6 for CCR1B input
    p12.p1sel.modify(|_, w| w.p6().set_bit());
    p12.p1dir.modify(|_, w| w.p6().clear_bit());
}

pub fn setup_timer(timer : &TIMER_A2) {
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
}
