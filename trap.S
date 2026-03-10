.section .text
.global trap_entry
.align 2

trap_entry:
    # Sichern des kompletten Kontexts, damit der Harlekin volle Sicht hat.
    # Wir allozieren Platz für alle 32 Register + mstatus, mepc, mcause, mtval.
    addi sp, sp, -36*8

    sd ra, 0*8(sp)    # x1
    # sp (x2) wird nicht gesichert, da wir es gerade verändert haben.
    sd gp, 2*8(sp)    # x3
    sd tp, 3*8(sp)    # x4
    sd t0, 4*8(sp)    # x5
    sd t1, 5*8(sp)    # x6
    sd t2, 6*8(sp)    # x7
    sd s0, 7*8(sp)    # x8
    sd s1, 8*8(sp)    # x9
    sd a0, 9*8(sp)    # x10
    sd a1, 10*8(sp)   # x11
    sd a2, 11*8(sp)   # x12
    sd a3, 12*8(sp)   # x13
    sd a4, 13*8(sp)   # x14
    sd a5, 14*8(sp)   # x15
    sd a6, 15*8(sp)   # x16
    sd a7, 16*8(sp)   # x17
    sd s2, 17*8(sp)   # x18
    sd s3, 18*8(sp)   # x19
    sd s4, 19*8(sp)   # x20
    sd s5, 20*8(sp)   # x21
    sd s6, 21*8(sp)   # x22
    sd s7, 22*8(sp)   # x23
    sd s8, 23*8(sp)   # x24
    sd s9, 24*8(sp)   # x25
    sd s10, 25*8(sp)  # x26
    sd s11, 26*8(sp)  # x27
    sd t3, 27*8(sp)   # x28
    sd t4, 28*8(sp)   # x29
    sd t5, 29*8(sp)   # x30
    sd t6, 30*8(sp)   # x31
    
    # CSRs sichern
    csrr t0, mstatus
    csrr t1, mepc
    csrr t2, mcause
    csrr t3, mtval
    sd t0, 32*8(sp)
    sd t1, 33*8(sp)
    sd t2, 34*8(sp)
    sd t3, 35*8(sp)

    # Trap-Frame-Pointer (a0) an den Rust-Handler übergeben.
    mv a0, sp

    # Den Rust-Handler aufrufen.
    call harlekin_trap_handler

    # Wiederherstellen der CSRs
    ld t0, 32*8(sp)
    ld t1, 33*8(sp)
    csrw mstatus, t0
    csrw mepc, t1

    # Register wiederherstellen.
    ld ra, 0*8(sp)
    # sp wird nicht wiederhergestellt.
    ld gp, 2*8(sp)
    ld tp, 3*8(sp)
    ld t0, 4*8(sp)
    ld t1, 5*8(sp)
    ld t2, 6*8(sp)
    ld s0, 7*8(sp)
    ld s1, 8*8(sp)
    ld a0, 9*8(sp)
    # a0-a7, s2-s11, t3-t6 werden nach Bedarf wiederhergestellt, 
    # hier aber der Einfachheit halber komplett.
    ld a1, 10*8(sp)
    ld a2, 11*8(sp)
    ld a3, 12*8(sp)
    ld a4, 13*8(sp)
    ld a5, 14*8(sp)
    ld a6, 15*8(sp)
    ld a7, 16*8(sp)
    ld s2, 17*8(sp)
    ld s3, 18*8(sp)
    ld s4, 19*8(sp)
    ld s5, 20*8(sp)
    ld s6, 21*8(sp)
    ld s7, 22*8(sp)
    ld s8, 23*8(sp)
    ld s9, 24*8(sp)
    ld s10, 25*8(sp)
    ld s11, 26*8(sp)
    ld t3, 27*8(sp)
    ld t4, 28*8(sp)
    ld t5, 29*8(sp)
    ld t6, 30*8(sp)

    # Stack-Pointer wiederherstellen
    addi sp, sp, 36*8

    # Rückkehr vom Trap
    mret
