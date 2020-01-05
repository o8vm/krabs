MEMORY {
    body    :   ORIGIN =   0, LENGTH = 444
    size    :   ORIGIN = 444, LENGTH = 2
}

ENTRY(_start);

EXTERN(STAGE2_SISE);

SECTIONS {
    .text   :   { *(.startup) *(.text) }    > body      /* Excutable code */
    .rodata :   { *(.rodata*) } > body      /* Constants (R/O) */
    .data   :   { *(.data) }    > body      /* Initialized data */
    .bss    :   { *(.bss) }     > body      /* Unitnitialized data */
    .size   :   { KEEP(*(.size*)) }    > size      /* Boot parameter table */
}