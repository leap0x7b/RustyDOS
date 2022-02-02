ENTRY(_start)

SECTIONS {
    dsk_stack = 0x0B00;
    . = dsk_stack;

    .text : {
        *(.text*)
    }

    .data : {
        *(.data*)
    }

    .rodata : {
        *(.rodata*)
    }

    .bss : {
        *(COMMON)
        *(.bss*)
    }

    /DISCARD/ : {
        *(.eh_frame*)
    }
}
