ENTRY(stage4)
SECTIONS {
    . = 0x30000;                /* "stage4" will be loaded at phyisical addr */
    .first  : { *(.first) }     /* Locate "stage4" First Excutable code */
    .text   : { *(.text) }      /* Excutable code                       */
    .rodata : { *(.rodata) }    /* Constants (R/O)                      */
    .data   : { *(.data) }      /* Initialized data                     */
    _data_end = .;              /* The end of .data section             */
    .bss    : { *(.bss) }       /* Uninitialized data                   */
    _bss_end = .;               /* The end of .bss section              */
    /DISCARD/ : { *(.eh_frame*) }
}