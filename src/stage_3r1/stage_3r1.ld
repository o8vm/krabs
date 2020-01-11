ENTRY(stage3)
SECTIONS {
    . = 0x6000;
    .first  : { *(.fisrt)  }
    .text   : { *(.text)   }
    .rodata : { *(.rodata) }
    .data   : { *(.data)   }
    .bss    : { 
        _data_end = .; 
        *(.bss) 
        _bss_end = .;
    }
}