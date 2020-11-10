pub fn i2c_init() {
    // in i2c master mode, USII2C=1 USICKPL=1 USICKPH=0 and USIMST=1

}

pub fn i2c_tx() {
    // data is first loaded into USISRL
    // output is enabled by setting USIOE
    // transmission is started by writing 8 into USICNTx
    // after the transmission of all 8 bits, USIIFG is set

    // to recieve i2c ack, USIOE bit cleared with SW and USICNTx is loaded with 1
    // when USIIFG becomes set again the LSB of USISRL is the rx'd acknowledge
    // bit can be tested in software
}

pub fn i2c_rx() {
    // output must be disabled by clearing USIOE
    // prepare for reception by writing 8 into USICNTx
    // the USIIFG bit will be set after 8 clocks

    // To ack or nack MSB of shift register should be loaded with 0 or 1
    // set USIOE bit to enable output, and write 1 to USICNTx
    // as soon as MSB is shifted out USIIFG will be set
}

pub fn i2c_start() {
    // set the MSB of the shift register to 0
    // set USIGE and USIOE to 1 to present MSB of shift reg to SDA
    // clear USIGE to resume clocked latch function
    // and holds 0 on SDA until data is shifted out with SCL? is that right?
}

pub fn i2c_stop() {
    // clear msb of of shift reg and load 1 into USICNTx
    // this will generate a low pulse on SCL. SCL stops in the high state
    //
    // to generate low to high transition
    // MSB is set in shift reg and USICNTx is loaded with 1
    // set USIGE and USIOE to make the output latch transparent and
    // the MSB of USISRL releases SDA to the idle state.
    // clear USIGE to store the msb in the output latch and the output is disabled
    // by clearing USIOE
    // SDA remains high until start condition
}
