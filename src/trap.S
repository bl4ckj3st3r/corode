# src/trap.S - RISC-V Trap-Handler

.section .text.trap_entry
.global trap_entry
.align 4 # Funktion auf 4-Byte-Grenze ausrichten
trap_entry:
    # Sichern des kompletten Kontexts auf dem Stack.
    # Wir legen 32 Register à 8 Bytes ab (insgesamt 256 Bytes).
    addi sp, sp, -256
    sd ra, 0(sp)       # x1
    # sp (x2) wird nicht gesichert, da wir es gerade verändert haben.
    sd gp, 16(sp)      # x3
    sd tp, 24(sp)      # x4
    sd t0, 32(sp)      # x5
    sd t1, 40(sp)      # x6
    sd t2, 48(sp)      # x7
    sd s0, 56(sp)      # x8
    sd s1, 64(sp)      # x9
    sd a0, 72(sp)      # x10
    sd a1, 80(sp)      # x11
    sd a2, 88(sp)      # x12
    sd a3, 96(sp)      # x13
    sd a4, 104(sp)     # x14
    sd a5, 112(sp)     # x15
    sd a6, 120(sp)     # x16
    sd a7, 128(sp)     # x17
    sd s2, 136(sp)     # x18
    sd s3, 144(sp)     # x19
    sd s4, 152(sp)     # x20
    sd s5, 160(sp)     # x21
    sd s6, 168(sp)     # x22
    sd s7, 176(sp)     # x23
    sd s8, 184(sp)     # x24
    sd s9, 192(sp)     # x25
    sd s10, 200(sp)    # x26
    sd s11, 208(sp)    # x27
    sd t3, 216(sp)     # x28
    sd t4, 224(sp)     # x29
    sd t5, 232(sp)     # x30
    sd t6, 240(sp)     # x31

    # CSRs sichern
    csrr t0, mcause
    csrr t1, mepc
    csrr t2, mtval
    csrr t3, mstatus
    sd t0, 248(sp) # mcause
    sd t1, 192(sp) # mepc
    sd t2, 200(sp) # mtval
    sd t3, 208(sp) # mstatus


    # Übergabe des Trap-Frame-Pointers (aktueller sp) an die Rust-Funktion.
    mv a0, sp
    call rust_trap_handler

    # Kontext wiederherstellen
    ld ra, 0(sp)
    ld gp, 16(sp)
    ld tp, 24(sp)
    ld t0, 32(sp)
    ld t1, 40(sp)
    ld t2, 48(sp)
    ld s0, 56(sp)
    ld s1, 64(sp)
    ld a0, 72(sp)
    ld a1, 80(sp)
    ld a2, 88(sp)
    ld a3, 96(sp)
    ld a4, 104(sp)
    ld a5, 112(sp)
    ld a6, 120(sp)
    ld a7, 128(sp)
    ld s2, 136(sp)
    ld s3, 144(sp)
    ld s4, 152(sp)
    ld s5, 160(sp)
    ld s6, 168(sp)
    ld s7, 176(sp)
    ld s8, 184(sp)
    ld s9, 192(sp)
    ld s10, 200(sp)
    ld s11, 208(sp)
    ld t3, 216(sp)
    ld t4, 224(sp)
    ld t5, 232(sp)
    ld t6, 240(sp)
    
    # CSRs wiederherstellen
    ld t0, 248(sp) # mcause
    ld t1, 192(sp) # mepc
    ld t2, 200(sp) # mtval
    ld t3, 208(sp) # mstatus
    csrw mcause, t0
    csrw mepc, t1
    csrw mtval, t2
    csrw mstatus, t3

    addi sp, sp, 256
    
    # Rückkehr aus dem Trap
    mret
