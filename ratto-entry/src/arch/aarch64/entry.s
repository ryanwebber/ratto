.section .text._start
.global _start
.type _start, function

_start:
    //--------------------------------------------------------------------------
    // Preserve boot protocol values
    //--------------------------------------------------------------------------
    // x0 = DTB physical address (per AArch64 boot protocol)
    mov     x20, x0

    //--------------------------------------------------------------------------
    // Boot core selection
    //--------------------------------------------------------------------------
    mrs     x0, MPIDR_EL1
    and     x0, x0, {CONST_CORE_ID_MASK}

    ldr     x1, BOOT_CORE_ID
    cmp     x0, x1
    b.ne    .L_parking_loop

    //--------------------------------------------------------------------------
    // Store DTB pointer for Rust
    //--------------------------------------------------------------------------
    adrp    x1, __dtb_ptr
    add     x1, x1, :lo12:__dtb_ptr
    str     x20, [x1]

    //--------------------------------------------------------------------------
    // Zero .bss
    //--------------------------------------------------------------------------
    adrp    x0, __bss_start
    add     x0, x0, :lo12:__bss_start

    adrp    x1, __bss_end_exclusive
    add     x1, x1, :lo12:__bss_end_exclusive

.L_bss_init_loop:
    cmp     x0, x1
    b.eq    .L_prepare_rust
    stp     xzr, xzr, [x0], #16
    b       .L_bss_init_loop

    //--------------------------------------------------------------------------
    // Prepare jump to Rust
    //--------------------------------------------------------------------------
.L_prepare_rust:
    // Set up the stack (boot core only)
    adrp    x0, __boot_core_stack_end_exclusive
    add     x0, x0, :lo12:__boot_core_stack_end_exclusive
    mov     sp, x0

    // Pass DTB pointer to Rust as argument (optional but nice)
    mov     x0, x20

    // Jump to Rust
    b       _start_rust

    //--------------------------------------------------------------------------
    // Non-boot cores park here
    //--------------------------------------------------------------------------
.L_parking_loop:
    wfe
    b       .L_parking_loop

.size _start, . - _start
