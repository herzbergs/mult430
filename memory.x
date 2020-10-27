MEMORY
{
  /* These values are correct for the msp430g2531 device.
     for interrupt vectors plus reset vector and the end of the first 64kB
     of address space. */
  RAM : ORIGIN = 0x0200, LENGTH = 0x0080
  ROM : ORIGIN = 0xF800, LENGTH = 0x07C0
  VECTORS : ORIGIN = 0xFFE0, LENGTH = 0x20
}
