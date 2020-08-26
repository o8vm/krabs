ENTRY(stage3)
ENTRY(hello)
SECTIONS {
    . = 0x6000;
    .first  : { *(.first)  }
    . = 0x6100;
    .param  : { KEEP(*(.param)) }
    . = 0x6110;
    .second : { KEEP(*(.second)) }
    .text   : { *(.text*)   }
    .rodata : { *(.rodata*) }
    .data   : { *(.data*)   }
    .bss    : { 
        _data_end = .; 
        *(.bss*) 
        _bss_end = .;
    }
    /DISCARD/ : { *(.eh_frame*) }
}