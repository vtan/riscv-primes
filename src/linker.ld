ENTRY(_start);

. = 0x80000000;

SECTIONS
{
  .text : ALIGN(4K) {
    KEEP(*(.text.init))
    *(.text .text.*)
  }

  .rodata : ALIGN(4K) {
    *(.rodata .rodata.*)
  }

  .data : ALIGN(4K) {
    *(.data .data.*)
  }

  .bss : ALIGN(4K) {
    *(.bss .bss.*)
    . = ALIGN(4K);
    . += 4K;
    PROVIDE(stack_top = .);
  }
}
