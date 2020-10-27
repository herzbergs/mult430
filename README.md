## mult430

Mult430 is a tool to multiply and divide the rate of pulses that are used to sync the tempo of some music production equiptment. There are existing tools to do this, including those that are completely in hardware, but this tool provides an interface and form factor that may be more familiar.

### Project Goals

This is a project first and foremost for me to play around and explore. Both the rust language, which is new to me, and with limited hardware. And I suppose with music. I anticipate first controlling the with a fixed divisor chosen at compile time, and then later moving to a character display and physical buttons.

### Parts

MSP430g2231

2 minijacks

16x2 Character Display

3 buttons

### Resources

This project uses two timers, one of which with a capture input pin, the USI (for the display), and 3 or 4 GPIO pins. Pretty much at the limit of pins on the MSP430g2231. This chip doesn't have a Uart. If I decide I want one for convenience's sake, porting to a more full featured chip should be easy. This is just what I had lying around.

### Instructions

To build, execute 'cargo build -Zbuild-std=core --release'. Steps to flash the chip will vary, but for me it is '/usr/local/bin/mspdebug rf2500 'prog target/msp430-none-elf/release/mult430'.

This project uses the peripheral access crate for msp430g2231. This is an auto generated crate currently unavailible on crates.io. I have not yet uploaded a version of it because I think it could use quite a bit of hand tweaking. The main issue with the default svd is fields are provided as individual bits.
