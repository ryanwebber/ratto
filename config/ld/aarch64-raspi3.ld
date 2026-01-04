/* SPDX-License-Identifier: MIT OR Apache-2.0 */

__rpi_phys_dram_start_addr      = 0x00000000;
__rpi_phys_binary_load_addr     = 0x00080000;

ENTRY(_start)

PHDRS
{
    segment_code PT_LOAD FLAGS(5); /* RX */
    segment_data PT_LOAD FLAGS(6); /* RW */
}

SECTIONS
{
    /* Kernel load address */
    . = __rpi_phys_binary_load_addr;
    __kernel_start = .;

    /***********************************************************************************************
     * Code + RO Data
     ***********************************************************************************************/
    .text : ALIGN(4K)
    {
        KEEP(*(.text._start))
        *(.text._start_arguments)
        *(.text._start_rust)
        *(.text*)
    } :segment_code

    .rodata : ALIGN(8)
    {
        *(.rodata*)
    } :segment_code

    /***********************************************************************************************
     * Data (must survive .bss zeroing!)
     ***********************************************************************************************/
    .data : ALIGN(8)
    {
        *(.data*)

        /* Boot-time data */
        __dtb_ptr = .;
        . += 8;
    } :segment_data

    /***********************************************************************************************
     * BSS (zeroed by early assembly)
     ***********************************************************************************************/
    .bss (NOLOAD) : ALIGN(16)
    {
        __bss_start = .;
        *(.bss*)
        *(COMMON)
        . = ALIGN(16);
        __bss_end_exclusive = .;
    } :segment_data

    /***********************************************************************************************
     * Boot Core Stack (not zeroed)
     ***********************************************************************************************/
    .boot_core_stack (NOLOAD) : ALIGN(16)
    {
        __boot_core_stack_start = .;
        . += 0x4000; /* 16 KiB stack */
        __boot_core_stack_end_exclusive = .;
    }

    __kernel_end = ALIGN(4K);

    /***********************************************************************************************
     * Misc
     ***********************************************************************************************/
    .got : { *(.got*) }
    ASSERT(SIZEOF(.got) == 0, "Relocation support not expected")

    /DISCARD/ : { *(.comment*) }
}
