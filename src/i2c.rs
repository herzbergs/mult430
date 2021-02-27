use msp430g2231::{USI};

// references to user guide in the following code always refers to the
// MSP430x2xx Family User's Guide rev I

// user guide describesi2c mode master config in 14.2.4

pub fn i2c_init(i2c_hw : &USI) {
    //TODO clock configuration

    // user guide section 17.3.1 says always assert reset when configuring ucsi, not
    // usi used by this unit, but it seems like it might apply here too
    i2c_hw.usictl0.modify(|_, w| w.usiswrst().set_bit());

    // in i2c master mode, USII2C=1 USICKPL=1 USICKPH=0 and USIMST=1
    i2c_hw.usictl1.modify(|_, w| w.usii2c().set_bit()
                                  .usickph().clear_bit());

    i2c_hw.usickctl.modify(|_, w| w.usickpl().set_bit());

    // we will enable SDA and SCL
    i2c_hw.usictl0.modify(|_, w| w.usipe7().set_bit()
                                  .usipe6().set_bit()
                                  .usimst().set_bit());

    i2c_hw.usictl0.modify(|_, w| w.usiswrst().clear_bit());
}

pub fn i2c_tx(i2c_hw : &USI, data : u8) -> bool {
    // data is first loaded into USISRL
    i2c_hw.usisrl.write(|w| unsafe { w.bits(data) });

    // output is enabled by setting USIOE
    i2c_hw.usictl0.modify(|_,w| w.usioe().set_bit());
    // transmission is started by writing 8 into USICNTx
    i2c_hw.usicnt.modify(|_, w| w.usicnt3().set_bit());

    // after the transmission of all 8 bits, USIIFG is set TODO actually check this
    let _flag = i2c_hw.usictl1.read().usiifg().bit_is_set();

    // to recieve i2c ack, USIOE bit cleared with SW and USICNTx is loaded with 1
    i2c_hw.usictl0.modify(|_, w| w.usioe().clear_bit());
    i2c_hw.usicnt.modify(|_, w| w.usicnt0().set_bit());
    // when USIIFG becomes set again the LSB of USISRL is the rx'd acknowledge
    let _flag = i2c_hw.usictl1.read().usiifg().bit_is_set();
    // bit can be tested in software
    return _flag;
}

pub fn i2c_rx(i2c_hw : &USI) -> u8{
    // output must be disabled by clearing USIOE
    i2c_hw.usictl0.modify(|_,w| w.usioe().clear_bit());
    // prepare for reception by writing 8 into USICNTx
    i2c_hw.usicnt.modify(|_, w| w.usicnt3().set_bit());
    // the USIIFG bit will be set after 8 clocks
    let _flag = i2c_hw.usictl1.read().usiifg().bit_is_set();

    let data = i2c_hw.usisrl.read().bits();

    // To ack or nack MSB of shift register should be loaded with 0 or 1
    i2c_hw.usisrl.write(|w| unsafe { w.bits(0) });
    // set USIOE bit to enable output, and write 1 to USICNTx
    i2c_hw.usictl0.modify(|_, w| w.usioe().set_bit());
    i2c_hw.usicnt.modify(|_, w| w.usicnt0().set_bit());
    // as soon as MSB is shifted out USIIFG will be set
    let _flag = i2c_hw.usictl1.read().usiifg().bit_is_set();

    return data;
}

pub fn i2c_start(i2c_hw : &USI) {
    // set the MSB of the shift register to 0
    i2c_hw.usisrl.write(|w| unsafe { w.bits(0) });
    // set USIGE and USIOE to 1 to present MSB of shift reg to SDA
    i2c_hw.usictl0.modify(|_,w| w.usioe().set_bit()
                                 .usige().set_bit());
    // clear USIGE to resume clocked latch function
    i2c_hw.usictl0.modify(|_,w| w.usige().clear_bit());
    // and holds 0 on SDA until data is shifted out with SCL? is that right?
}

pub fn i2c_stop(i2c_hw : &USI) {
    // is it best to set OE here too?
    // clear msb of of shift reg and load 1 into USICNTx
    i2c_hw.usisrl.write(|w| unsafe { w.bits(0) });
    i2c_hw.usicnt.modify(|_, w| w.usicnt0().set_bit());
    // this will generate a low pulse on SCL. SCL stops in the high state

    // to generate low to high transition
    // MSB is set in shift reg and USICNTx is loaded with 1
    i2c_hw.usisrl.write(|w| unsafe { w.bits(0) });
    i2c_hw.usicnt.modify(|_, w| w.usicnt0().set_bit());
    // set USIGE and USIOE to make the output latch transparent and
    i2c_hw.usictl0.modify(|_,w| w.usioe().set_bit()
                                 .usige().set_bit());
    // the MSB of USISRL releases SDA to the idle state.
    // clear USIGE to store the msb in the output latch and the output is disabled
    // by clearing USIOE
    i2c_hw.usictl0.modify(|_,w| w.usioe().clear_bit()
                                 .usige().clear_bit());
    // SDA remains high until start condition
}
