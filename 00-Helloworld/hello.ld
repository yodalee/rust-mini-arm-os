_stack = 0x0;
SECTIONS {
  . = 0x0;
  .text :
  {
    KEEP(*(isr_vector))
    *(.text)
  }
}
