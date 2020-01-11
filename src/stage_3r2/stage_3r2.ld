ENTRY(start_32)
SECTIONS {
    . = 0x90000;                /* "stage3" will be loaded at phyisical addr */
    .first  : { *(.fisrt) }     /* Locate "stages" First Excutable code */
    .text   : { *(.text) }      /* Excutable code                       */
    .rodata : { *(.rodata) }    /* Constants (R/O)                      */
    .data   : { *(.data) }      /* Initialized data                     */
    _data_end = .;              /* The end of .data section             */
    .bss    : { *(.bss) }       /* Uninitialized data                   */
    _bss_end = .;               /* The end of .bss section              */
}