OUTPUT_ARCH(riscv)
ENTRY(_start)

MEMORY
{
    RAM (rwx) : ORIGIN = 0x80200000, LENGTH = 128M
}

SECTIONS
{
    . = 0x80200000;
    
    .text : {
        *(.text._start)
        *(.text*)
    } > RAM
    
    .rodata : {
        *(.rodata*)
    } > RAM
    
    .vault : {
        __vault_start = .;
        *(.vault*)
        . = ALIGN(4K);
        __vault_end = .;
    } > RAM
    
    .data : {
        *(.data*)
    } > RAM
    
    .bss : {
        *(.bss*)
    } > RAM
    
    /DISCARD/ : {
        *(.comment)
        *(.note*)
        *(.eh_frame*)
    }
}
