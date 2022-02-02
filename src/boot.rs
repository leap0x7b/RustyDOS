use core::arch::global_asm;
global_asm!(
".section .text",
".align 2",
".int 0x55aa55aa",

".global _start",
"_start:",
"    mov ax, cs",
"    mov ds, ax",
"    mov es, ax",
"    mov ss, ax",
"    mov sp, 0xb00",
"    sti",
"    call kmain",
);
