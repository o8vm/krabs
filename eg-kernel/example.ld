ENTRY(_start)
SECTIONS {
    . = 0x100000;              /* "stage3" will be loaded at 2nd page  */
    .first  : { *(.startup) }     /* Locate "stages" First Excutable code */
    .text   : { *(.text) }      /* Excutable code                       */
    .rodata : { *(.rodata) }    /* Constants (R/O)                      */
    .data   : { *(.data) }      /* Initialized data                     */
    _data_end = .;              /* The end of .data section             */
    .bss    : { *(.bss) }       /* Uninitialized data                   */
    _bss_end = .;               /* The end of .bss section              */
}