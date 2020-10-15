** mult430

Mult430 is a tool to multiply and divide the rate of pulses that are used to sync the tempo of some music production equiptment. There are existing tools to do this, including those that are completely in hardware, but this tool provides an interface and form factor that may be more familiar.

** Project Goals

This is a project first and foremost for me to play around and explore. Both the rust language, which is new to me, on limited hardware, and with music. I anticipate first controlling the device with a uart based interface, and then later moving to a character display and physical buttons.

** Parts

MSP430g2231
2 minijacks
16x2 Character Display
3 buttons

** Instructions

To build, execute 'cargo build -Zbuild-std=core --release'. Steps to flash the chip will vary, but for me it is '/usr/local/bin/mspdebug rf2500 'prog target/msp430-none-elf/release/mult430'.

