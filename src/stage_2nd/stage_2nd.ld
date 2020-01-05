ENTRY(stage2)
EXTERN(PARAM);
SECTIONS {
    . = 0x200;
    .table  :   { KEEP(*(.table)) } 
    .text   :   { *(.startup) *(.text) }
    .rodata :   { *(.rodata*) }
    .data   :   { *(.data) }
    .bss    :   { *(.bss) }
}