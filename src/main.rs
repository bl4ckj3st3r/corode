#![no_std]
#![no_main]
#![feature(const_fn_trait_bound)]

// =============================================================================
// ⚙️ corode-core: src/main.rs
// =============================================================================

use core::panic::PanicInfo;
use core::arch::asm;

// =============================================================================
// I. Die Verfassung: PMP-Konstanten & Adress-Layout
// =============================================================================
const PMP_R: usize = 1 << 0; const PMP_W: usize = 1 << 1; const PMP_X: usize = 1 << 2;
const PMP_NAPOT: usize = 3 << 3; const PMP_L: usize = 1 << 7;

const ADDR_VEC_0_THRON: usize = 0x80000000;      // 64KB Kernel
const ADDR_VEC_1_WAECHTER: usize = 0x80010000;   // 4KB Harlekin
const ADDR_VEC_2_SCHREIBER: usize = 0x10000000;   // 1MB Logger
const ADDR_VEC_2_SCHREIBER_END: usize = ADDR_VEC_2_SCHREIBER + (1024 * 1024) -1; // Ende der Logger-Region
const ADDR_VEC_3_ORAKEL: usize = 0x10010000;    // 1MB Z3
const ADDR_VEC_5_EXIL: usize = 0x90000000;       // 256MB Turing-Band
const ADDR_VEC_6_BOTE: usize = 0x80020000;       // 64KB corode-net
const ADDR_VEC_7_AUGE: usize = 0xA0000000;       // 32MB corode-ui Framebuffer
const ADDR_VEC_8_ARENA0: usize = 0xB0000000;     // 128MB Condition-Sandkiste
const ADDR_VEC_9_LEHRER: usize = 0xC0000000;     // 1GB AI Sidekernel
const ADDR_VEC_10_TREIBER: usize = 0xD0000000;    // 1GB Linux Sidekernel
const ADDR_VEC_11_DIPLOMAT: usize = 0xE0000000;   // 1GB Unix Sidekernel
const ADDR_VEC_14_WERKZEUGE: usize = 0x0C000000; // MMIO Bereich
const ADDR_VEC_15_ZIEL: usize = 0xDEADBEEF;     // Einzelne, gesperrte Adresse

// =============================================================================
// II. Der Chronist: Trickster (Globale Singleton Instanz)
// =============================================================================
struct Trickster {
    cursor: *mut u8,
    start_addr: usize,
    end_addr: usize,
}

// Globale, zustandsbehaftete Instanz des Loggers.
// `unsafe` ist hier notwendig, weil wir eine globale, veränderliche Ressource in
// einer Bare-Metal-Umgebung verwalten.
static mut GLOBAL_TRICKSTER: Trickster = Trickster {
    cursor: ADDR_VEC_2_SCHREIBER as *mut u8,
    start_addr: ADDR_VEC_2_SCHREIBER,
    end_addr: ADDR_VEC_2_SCHREIBER_END,
};

impl Trickster {
    // Holt eine `unsafe` veränderliche Referenz auf den globalen Logger.
    fn get_global() -> &'static mut Trickster {
        unsafe { &mut GLOBAL_TRICKSTER }
    }
    
    // Private Log-Funktion, die die zirkuläre Pufferlogik implementiert.
    fn log_byte(&mut self, byte: u8) {
        unsafe {
            // Prüfen, ob der Cursor das Ende erreicht hat.
            if self.cursor as usize >= self.end_addr {
                self.cursor = self.start_addr as *mut u8; // Zurück zum Anfang springen.
            }
            core::ptr::write_volatile(self.cursor, byte);
            self.cursor = self.cursor.add(1);
        }
    }

    // Öffentliche API zum Loggen einer Zeichenkette.
    fn log(&mut self, message: &str) {
        for &byte in message.as_bytes() {
            self.log_byte(byte);
        }
    }
    
    // Öffentliche API zum Loggen einer Hex-Zahl.
    fn log_hex(&mut self, n: usize) {
        let mut temp = n;
        let mut buffer = [0u8; 16];
        let mut i = 15;

        self.log("0x");

        if temp == 0 {
            self.log_byte(b'0');
            return;
        }

        loop {
            let digit = (temp % 16) as u8;
            buffer[i] = if digit < 10 { digit + b'0' } else { digit - 10 + b'a' };
            temp /= 16;
            if temp == 0 { break; }
            i -= 1;
        }
        
        for &byte in &buffer[i..] {
            self.log_byte(byte);
        }
    }
}

// =============================================================================
// III. Der letzte Ausweg: Panic Handler
// =============================================================================
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { loop {} }

// =============================================================================
// IV. Der Souverän: _start (Eintrittspunkt)
// =============================================================================

// Der Eierschalensollbruchstellenverursacher: Ein pseudo-zufälliger Trigger
struct Lcg { state: u32 }
impl Lcg {
    fn new(seed: u32) -> Self { Lcg { state: seed } }
    fn next(&mut self) -> u32 {
        self.state = self.state.wrapping_mul(1103515245).wrapping_add(12345);
        self.state
    }
}

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    let trickster = Trickster::get_global();
    asm!("csrw mtvec, {}", in(reg) harlekin_trap_handler as usize);
    setup_pmp_vectors();
    
    // Initialisiere den Zufallsgenerator (der Seed ist hier noch statisch)
    let mut rng = Lcg::new(0xCAFEF00D); 
    let random_value = rng.next();

    // Seltene Chance (ca. 1 zu 65536) das Easter Egg auszulösen
    if (random_value >> 16) == 0xDEAD {
        trickster.log("CHAOS-ANOMALIE ENTDECKT! \n  >> Eierschalensollbruchstellenverursacher AKTIVIERT! <<\n");
        trickster.log("INITIIERE OOC-SELBSTANGRIFF...\n");
        core::ptr::write_volatile(ADDR_VEC_15_ZIEL as *mut u32, 0x1337);
        // Die Erfolgsmeldung wird nun vom Harlekin geloggt, da er den Angriff abfängt.
    } else {
        trickster.log("PMP-Schilde oben. Selbsttest ohne Anomalie. System stabil.\n");
    }

    loop {}
}

// =============================================================================
// V. Der Wächter: Harlekin Trap Handler
// =============================================================================
#[no_mangle]
#[repr(align(4))]
pub unsafe extern "C" fn harlekin_trap_handler() {
    let trickster = Trickster::get_global();
    let mcause: usize;
    let mepc: usize;
    let mtval: usize;
    asm!("csrr {}, mcause", out(reg) mcause);
    asm!("csrr {}, mepc", out(reg) mepc);
    asm!("csrr {}, mtval", out(reg) mtval);

    trickster.log("\n**********************************************\n");
    trickster.log(">> HARLEKIN FÄNGT EINDRINGLING AB! <<\n");
    trickster.log("   URSACHE (mcause): "); trickster.log_hex(mcause);
    // Decode mcause to provide a human-readable reason
    match mcause {
        5 => trickster.log(" (Load access fault)\n"),
        7 => trickster.log(" (Store/AMO access fault)\n"),
        _ => trickster.log(" (Unknown trap cause)\n"),
    }
    trickster.log("   ORT (mepc):     "); trickster.log_hex(mepc); trickster.log(" (Befehl, der Fehler auslöste)\n");
    trickster.log("   ZIEL (mtval):   "); trickster.log_hex(mtval); trickster.log(" (Verbotene Speicheradresse)\n");
    trickster.log("**********************************************\n");

    // Überspringe die fehlerhafte Anweisung und setze die Ausführung fort.
    // Der Harlekin heilt das System, indem er den illegalen Schreibversuch ignoriert.
    asm!("csrw mepc, {}", in(reg) (mepc + 4));
}

// =============================================================================
// VI. Die Schmiede: PMP Vektor-Setup
// =============================================================================
#[inline(never)]
unsafe fn setup_pmp_vectors() {
    // Vektor 0: Thron (R-X, 64KB)
    asm!("csrw pmpaddr0, {}", in(reg) (ADDR_VEC_0_THRON >> 2) | ((32*1024)-1) );
    asm!("csrw pmpcfg0, {}", in(reg) PMP_NAPOT | PMP_R | PMP_X | PMP_L);
    // Vektor 1: Wächter (R-X, 4KB)
    asm!("csrw pmpaddr1, {}", in(reg) (ADDR_VEC_1_WAECHTER >> 2) | ((2*1024)-1) );
    asm!("csrw pmpcfg1, {}", in(reg) PMP_NAPOT | PMP_R | PMP_X | PMP_L);
    // Vektor 2: Schreiber (RW-, 1MB)
    asm!("csrw pmpaddr2, {}", in(reg) (ADDR_VEC_2_SCHREIBER >> 2) | ((512*1024)-1) );
    asm!("csrw pmpcfg2, {}", in(reg) PMP_NAPOT | PMP_R | PMP_W | PMP_L);
    // Vektor 3: Orakel (R-X, 1MB)
    asm!("csrw pmpaddr3, {}", in(reg) (ADDR_VEC_3_ORAKEL >> 2) | ((512*1024)-1) );
    asm!("csrw pmpcfg3, {}", in(reg) PMP_NAPOT | PMP_R | PMP_X | PMP_L);
    // Vektor 4: Gesetzbuch (geschützt durch Lock-Bits der anderen)
    asm!("csrw pmpaddr4, {}", in(reg) 0);
    asm!("csrw pmpcfg4, {}", in(reg) 0);
    // Vektor 5: Exil (RW-, 256MB)
    asm!("csrw pmpaddr5, {}", in(reg) (ADDR_VEC_5_EXIL >> 2) | ((128*1024*1024)-1) );
    asm!("csrw pmpcfg5, {}", in(reg) PMP_NAPOT | PMP_R | PMP_W | PMP_L);
    // Vektor 6: Bote (RWX, 64KB)
    asm!("csrw pmpaddr6, {}", in(reg) (ADDR_VEC_6_BOTE >> 2) | ((32*1024)-1) );
    asm!("csrw pmpcfg6, {}", in(reg) PMP_NAPOT | PMP_R | PMP_W | PMP_X | PMP_L);
    // Vektor 7: Auge (RW-, 32MB)
    asm!("csrw pmpaddr7, {}", in(reg) (ADDR_VEC_7_AUGE >> 2) | ((16*1024*1024)-1) );
    asm!("csrw pmpcfg7, {}", in(reg) PMP_NAPOT | PMP_R | PMP_W | PMP_L);
    // Vektor 8: Arena 0 (---, 128MB, ungesperrt)
    asm!("csrw pmpaddr8, {}", in(reg) (ADDR_VEC_8_ARENA0 >> 2) | ((64*1024*1024)-1) );
    asm!("csrw pmpcfg8, {}", in(reg) PMP_NAPOT);
    // Vektor 9: Lehrer (RWX, 1GB, ungesperrt)
    asm!("csrw pmpaddr9, {}", in(reg) (ADDR_VEC_9_LEHRER >> 2) | ((512*1024*1024)-1) );
    asm!("csrw pmpcfg9, {}", in(reg) PMP_NAPOT | PMP_R | PMP_W | PMP_X);
    // Vektor 10: Treiber (RWX, 1GB, ungesperrt)
    asm!("csrw pmpaddr10, {}", in(reg) (ADDR_VEC_10_TREIBER >> 2) | ((512*1024*1024)-1) );
    asm!("csrw pmpcfg10, {}", in(reg) PMP_NAPOT | PMP_R | PMP_W | PMP_X);
    // Vektor 11: Diplomat (RWX, 1GB, ungesperrt)
    asm!("csrw pmpaddr11, {}", in(reg) (ADDR_VEC_11_DIPLOMAT >> 2) | ((512*1024*1024)-1) );
    asm!("csrw pmpcfg11, {}", in(reg) PMP_NAPOT | PMP_R | PMP_W | PMP_X);
    // Vektor 12: Arena 1 (ungenutzt)
    asm!("csrw pmpaddr12, {}", in(reg) 0);
    asm!("csrw pmpcfg12, {}", in(reg) 0);
    // Vektor 13: Vorhof (ungenutzt)
    asm!("csrw pmpaddr13, {}", in(reg) 0);
    asm!("csrw pmpcfg13, {}", in(reg) 0);
    // Vektor 14: Werkzeuge (RW-, MMIO, ungesperrt)
    asm!("csrw pmpaddr14, {}", in(reg) (ADDR_VEC_14_WERKZEUGE >> 2) | ((8*1024*1024)-1) );
    asm!("csrw pmpcfg14, {}", in(reg) PMP_NAPOT | PMP_R | PMP_W);
    // Vektor 15: Ziel (R--, 4-Byte)
    asm!("csrw pmpaddr15, {}", in(reg) ADDR_VEC_15_ZIEL >> 2);
    asm!("csrw pmpcfg15, {}", in(reg) PMP_NAPOT | PMP_R | PMP_L);
}
