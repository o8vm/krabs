MEMORY {
    stage3    :   ORIGIN = 0x6000, LENGTH = (0x9000 - 2 - 0x6000)
    sign      :   ORIGIN = (0x9000 - 2), LENGTH = 2
}

ENTRY(stage3)
SECTIONS {
    . = 0x6000;
    .first  : { *(.fisrt)  } > stage3
    .text   : { *(.text)   } > stage3
    .rodata : { *(.rodata) } > stage3
    .data   : { *(.data)   } > stage3
    .bss    : { 
        _data_end = .; 
        *(.bss) 
        _bss_end = .;
    }                        > stage3
    .sign   : {
        SHORT(0x1111)
    }                        > sign
    . = 0x9000;
    _start_32 = .;
}