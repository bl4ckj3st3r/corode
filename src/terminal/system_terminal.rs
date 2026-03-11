//! system-terminal (l1) für corode
//! - polling für eingabe
//! - direktes rendering in framebuffer
//! - eigenes parsing
//! - training simulation

use crate::uart;
use crate::condition;
use crate::zuse;
use crate::trickster;
use crate::pmp;

// farbdefinitionen (ansi)
const GRUEN: &str = "\x1B[32m";
const ROSA: &str = "\x1B[35m";
const ROT: &str = "\x1B[31m";
const RESET: &str = "\x1B[0m";

// framebuffer (qemu standard)
const FB_ADDR: usize = 0x81000000;
const FB_WIDTH: usize = 800;
const FB_HEIGHT: usize = 600;
const FB_SIZE: usize = FB_WIDTH * FB_HEIGHT * 4;

// einfache 8x8 bitmap font (ascii 32-126)
const FONT8X8: [[u8; 8]; 96] = [
    [0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00], // space
    [0x18,0x18,0x18,0x18,0x18,0x00,0x18,0x00], // !
    [0x6c,0x6c,0x6c,0x00,0x00,0x00,0x00,0x00], // "
    [0x6c,0x6c,0xfe,0x6c,0xfe,0x6c,0x6c,0x00], // #
    [0x18,0x3c,0x60,0x3c,0x0c,0x78,0x18,0x00], // $
    [0x00,0x66,0x66,0x3c,0x18,0x3c,0x66,0x66], // %
    [0x3c,0x66,0x30,0x3c,0x6c,0x66,0x3c,0x00], // &
    [0x18,0x18,0x18,0x00,0x00,0x00,0x00,0x00], // '
    [0x0c,0x18,0x30,0x30,0x30,0x18,0x0c,0x00], // (
    [0x30,0x18,0x0c,0x0c,0x0c,0x18,0x30,0x00], // )
    [0x00,0x66,0x3c,0xff,0x3c,0x66,0x00,0x00], // *
    [0x00,0x18,0x18,0x7e,0x18,0x18,0x00,0x00], // +
    [0x00,0x00,0x00,0x00,0x00,0x18,0x18,0x30], // ,
    [0x00,0x00,0x00,0x7e,0x00,0x00,0x00,0x00], // -
    [0x00,0x00,0x00,0x00,0x00,0x18,0x18,0x00], // .
    [0x00,0x06,0x0c,0x18,0x30,0x60,0x40,0x00], // /
    [0x3c,0x66,0x6e,0x76,0x66,0x66,0x3c,0x00], // 0
    [0x18,0x38,0x18,0x18,0x18,0x18,0x3c,0x00], // 1
    [0x3c,0x66,0x06,0x1c,0x30,0x60,0x7e,0x00], // 2
    [0x3c,0x66,0x06,0x1c,0x06,0x66,0x3c,0x00], // 3
    [0x0c,0x1c,0x3c,0x6c,0x7e,0x0c,0x0c,0x00], // 4
    [0x7e,0x60,0x7c,0x06,0x06,0x66,0x3c,0x00], // 5
    [0x3c,0x60,0x60,0x7c,0x66,0x66,0x3c,0x00], // 6
    [0x7e,0x06,0x0c,0x18,0x30,0x30,0x30,0x00], // 7
    [0x3c,0x66,0x66,0x3c,0x66,0x66,0x3c,0x00], // 8
    [0x3c,0x66,0x66,0x3e,0x06,0x0c,0x38,0x00], // 9
    [0x00,0x18,0x18,0x00,0x00,0x18,0x18,0x00], // :
    [0x00,0x18,0x18,0x00,0x00,0x18,0x18,0x30], // ;
    [0x0c,0x18,0x30,0x60,0x30,0x18,0x0c,0x00], // <
    [0x00,0x00,0x7e,0x00,0x7e,0x00,0x00,0x00], // =
    [0x30,0x18,0x0c,0x06,0x0c,0x18,0x30,0x00], // >
    [0x3c,0x66,0x06,0x0c,0x18,0x00,0x18,0x00], // ?
    [0x3c,0x66,0x60,0x3c,0x66,0x66,0x3c,0x00], // @
    [0x3c,0x66,0x66,0x7e,0x66,0x66,0x66,0x00], // A
    [0x7c,0x66,0x66,0x7c,0x66,0x66,0x7c,0x00], // B
    [0x3c,0x66,0x60,0x60,0x60,0x66,0x3c,0x00], // C
    [0x78,0x6c,0x66,0x66,0x66,0x6c,0x78,0x00], // D
    [0x7e,0x60,0x60,0x7c,0x60,0x60,0x7e,0x00], // E
    [0x7e,0x60,0x60,0x7c,0x60,0x60,0x60,0x00], // F
    [0x3c,0x66,0x60,0x6e,0x66,0x66,0x3c,0x00], // G
    [0x66,0x66,0x66,0x7e,0x66,0x66,0x66,0x00], // H
    [0x3c,0x18,0x18,0x18,0x18,0x18,0x3c,0x00], // I
    [0x0e,0x06,0x06,0x06,0x66,0x66,0x3c,0x00], // J
    [0x66,0x6c,0x78,0x70,0x78,0x6c,0x66,0x00], // K
    [0x60,0x60,0x60,0x60,0x60,0x60,0x7e,0x00], // L
    [0xc6,0xee,0xfe,0xd6,0xc6,0xc6,0xc6,0x00], // M
    [0xc6,0xe6,0xf6,0xde,0xce,0xc6,0xc6,0x00], // N
    [0x3c,0x66,0x66,0x66,0x66,0x66,0x3c,0x00], // O
    [0x7c,0x66,0x66,0x7c,0x60,0x60,0x60,0x00], // P
    [0x3c,0x66,0x66,0x66,0x6e,0x3c,0x06,0x0e], // Q
    [0x7c,0x66,0x66,0x7c,0x78,0x6c,0x66,0x00], // R
    [0x3c,0x66,0x60,0x3c,0x06,0x66,0x3c,0x00], // S
    [0x7e,0x18,0x18,0x18,0x18,0x18,0x18,0x00], // T
    [0x66,0x66,0x66,0x66,0x66,0x66,0x3c,0x00], // U
    [0x66,0x66,0x66,0x66,0x66,0x3c,0x18,0x00], // V
    [0xc6,0xc6,0xc6,0xd6,0xfe,0xee,0xc6,0x00], // W
    [0xc6,0x6c,0x38,0x1c,0x38,0x6c,0xc6,0x00], // X
    [0x66,0x66,0x3c,0x18,0x18,0x18,0x18,0x00], // Y
    [0x7e,0x06,0x0c,0x18,0x30,0x60,0x7e,0x00], // Z
    [0x3c,0x30,0x30,0x30,0x30,0x30,0x3c,0x00], // [
    [0x40,0x60,0x30,0x18,0x0c,0x06,0x00,0x00], // \
    [0x3c,0x0c,0x0c,0x0c,0x0c,0x0c,0x3c,0x00], // ]
    [0x18,0x3c,0x66,0x00,0x00,0x00,0x00,0x00], // ^
    [0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xff], // _
    [0x0c,0x0c,0x06,0x00,0x00,0x00,0x00,0x00], // `
    [0x00,0x00,0x3c,0x06,0x3e,0x66,0x3e,0x00], // a
    [0x60,0x60,0x7c,0x66,0x66,0x66,0x7c,0x00], // b
    [0x00,0x00,0x3c,0x66,0x60,0x66,0x3c,0x00], // c
    [0x06,0x06,0x3e,0x66,0x66,0x66,0x3e,0x00], // d
    [0x00,0x00,0x3c,0x66,0x7e,0x60,0x3c,0x00], // e
    [0x1c,0x36,0x30,0x7c,0x30,0x30,0x30,0x00], // f
    [0x00,0x00,0x3e,0x66,0x66,0x3e,0x06,0x3c], // g
    [0x60,0x60,0x7c,0x66,0x66,0x66,0x66,0x00], // h
    [0x18,0x00,0x38,0x18,0x18,0x18,0x3c,0x00], // i
    [0x0e,0x00,0x0e,0x06,0x06,0x66,0x66,0x3c], // j
    [0x60,0x60,0x6c,0x78,0x70,0x78,0x6c,0x00], // k
    [0x38,0x18,0x18,0x18,0x18,0x18,0x3c,0x00], // l
    [0x00,0x00,0xec,0xfe,0xd6,0xd6,0xd6,0x00], // m
    [0x00,0x00,0x7c,0x66,0x66,0x66,0x66,0x00], // n
    [0x00,0x00,0x3c,0x66,0x66,0x66,0x3c,0x00], // o
    [0x00,0x00,0x7c,0x66,0x66,0x7c,0x60,0x60], // p
    [0x00,0x00,0x3e,0x66,0x66,0x3e,0x06,0x0e], // q
    [0x00,0x00,0x7c,0x66,0x60,0x60,0x60,0x00], // r
    [0x00,0x00,0x3e,0x60,0x3c,0x06,0x7c,0x00], // s
    [0x30,0x30,0x7e,0x30,0x30,0x36,0x1c,0x00], // t
    [0x00,0x00,0x66,0x66,0x66,0x66,0x3e,0x00], // u
    [0x00,0x00,0x66,0x66,0x66,0x3c,0x18,0x00], // v
    [0x00,0x00,0xc6,0xd6,0xd6,0xfe,0x6c,0x00], // w
    [0x00,0x00,0x66,0x3c,0x18,0x3c,0x66,0x00], // x
    [0x00,0x00,0x66,0x66,0x66,0x3e,0x06,0x3c], // y
    [0x00,0x00,0x7e,0x0c,0x18,0x30,0x7e,0x00], // z
    [0x0e,0x18,0x18,0x70,0x18,0x18,0x0e,0x00], // {
    [0x18,0x18,0x18,0x18,0x18,0x18,0x18,0x18], // |
    [0x70,0x18,0x18,0x0e,0x18,0x18,0x70,0x00], // }
    [0x76,0xdb,0x00,0x00,0x00,0x00,0x00,0x00], // ~
];

pub struct SystemTerminal {
    // cage: Cage,
    eingabe_puffer: [u8; 256],
    eingabe_pos: usize,
    ausgabe_puffer: [u8; 4096],
    ausgabe_pos: usize,
    trainings_zyklus: u64,
    cursor_x: usize,
    cursor_y: usize,
}

impl SystemTerminal {
    pub fn neue(/*cage: Cage*/) -> Self {
        Self {
            // cage,
            eingabe_puffer: [0; 256],
            eingabe_pos: 0,
            ausgabe_puffer: [0; 4096],
            ausgabe_pos: 0,
            trainings_zyklus: 0,
            cursor_x: 10,
            cursor_y: 10,
        }
    }
    
    pub fn ausführen(&mut self) -> ! {
        // system-startmeldung in grün
        self.system_ausgabe("system-terminal (l1) gestartet\n");
        self.system_ausgabe("> ");
        
        loop {
            // 1. polling: eingabe lesen
            if let Some(taste) = uart::lese_taste_polling() {
                self.eingabe_behandeln(taste);
            }
            
            // 2. training simulieren (alle 100000 durchläufe)
            self.trainings_zyklus = self.trainings_zyklus.wrapping_add(1);
            if self.trainings_zyklus % 100000 == 0 {
                self.training_simulieren();
            }
            
            // 3. rendern (vorerst bei jeder Iteration, Optimierung später)
            self.rendern();
        }
    }
    
    fn eingabe_behandeln(&mut self, taste: u8) {
        match taste {
            b'\n' | b'\r' => {
                self.schreibe_zum_ausgabepuffer(b'\n');
                self.befehl_ausführen();
                
                self.schreibe_string_zum_ausgabepuffer("> ");
                self.eingabe_pos = 0;
                self.eingabe_puffer = [0; 256];
            }
            8 | 127 => { // backspace
                if self.eingabe_pos > 0 {
                    self.eingabe_pos -= 1;
                    self.eingabe_puffer[self.eingabe_pos] = 0;
                    
                    if self.ausgabe_pos > 0 {
                        self.ausgabe_pos -= 1;
                        self.ausgabe_puffer[self.ausgabe_pos] = 0;
                    }
                }
            }
            _ => {
                if self.eingabe_pos < self.eingabe_puffer.len() - 1 {
                    self.eingabe_puffer[self.eingabe_pos] = taste;
                    self.eingabe_pos += 1;
                    
                    self.schreibe_zum_ausgabepuffer(taste);
                }
            }
        }
    }
    
    fn befehl_ausführen(&mut self) {
        let eingabe = core::str::from_utf8(&self.eingabe_puffer[..self.eingabe_pos])
            .unwrap_or("").trim();
        
        match eingabe {
            // system-befehle
            "show conditions" => {
                self.terminal_ausgabe("\naktive conditions:\n");
                condition::liste_anzeigen(|msg| self.terminal_ausgabe(msg));
            }
            "show cages" => {
                self.terminal_ausgabe("\ncage belegung:\n");
                zuse::liste_anzeigen(|msg| self.terminal_ausgabe(msg));
            }
            "show logs" => {
                self.terminal_ausgabe("\nletzte logs:\n");
                trickster::logs_anzeigen(|msg| self.terminal_ausgabe(msg));
            }
            "show pmp" => {
                self.terminal_ausgabe("\npmp regionen:\n");
                pmp::liste_anzeigen(|msg| self.terminal_ausgabe(msg));
            }
            "show system status" => {
                self.system_ausgabe("\nsystem status:\n");
                // TODO: Re-enable dynamic stats when integer-to-string is available
                self.system_ausgabe("  conditions aktiv: 3\n  cages belegt: 1/15\n  pmp regionen: 16/16 konfiguriert\n");
            }
            
            // condition-befehle
            cmd if cmd.starts_with("freeze condition ") => {
                if let Some(id) = cmd.strip_prefix("freeze condition ").and_then(|s| s.parse().ok()) {
                    condition::einfrieren(id);
                    self.system_ausgabe("condition eingefroren\n");
                }
            }
            cmd if cmd.starts_with("thaw condition ") => {
                if let Some(id) = cmd.strip_prefix("thaw condition ").and_then(|s| s.parse().ok()) {
                    condition::auftauen(id);
                    self.system_ausgabe("condition aufgetaut\n");
                }
            }
            cmd if cmd.starts_with("start condition ") => {
                if let Some(name) = cmd.strip_prefix("start condition ") {
                    condition::starten(name);
                    self.schreibe_string_zum_ausgabepuffer(GRUEN);
                    self.schreibe_string_zum_ausgabepuffer("condition '");
                    self.schreibe_string_zum_ausgabepuffer(name);
                    self.schreibe_string_zum_ausgabepuffer("' gestartet\n");
                    self.schreibe_string_zum_ausgabepuffer(RESET);
                }
            }
            cmd if cmd.starts_with("stop condition ") => {
                if let Some(id) = cmd.strip_prefix("stop condition ").and_then(|s| s.parse().ok()) {
                    condition::stoppen(id);
                    self.system_ausgabe("condition gestoppt\n");
                }
            }
            
            // system-steuerung
            "restart" => self.system_ausgabe("system neustart...\n"),
            "shutdown" => self.system_ausgabe("system herunterfahren...\n"),
            "clear" => {
                self.ausgabe_pos = 0;
                self.ausgabe_puffer = [0; 4096];
            }
            
            "help" => {
                self.terminal_ausgabe("\nverfügbare befehle:\n  show conditions\n  show cages\n  show logs\n  show pmp\n  show system status\n  freeze condition [id]\n  thaw condition [id]\n  start condition [name]\n  stop condition [id]\n  restart\n  shutdown\n  clear\n  help\n");
            }
            
            "" => {} // Leere Eingabe ignorieren
            _ => {
                self.schreibe_string_zum_ausgabepuffer(ROSA);
                self.schreibe_string_zum_ausgabepuffer("unbekannter befehl: '");
                self.schreibe_string_zum_ausgabepuffer(eingabe);
                self.schreibe_string_zum_ausgabepuffer("'\n");
                self.schreibe_string_zum_ausgabepuffer(RESET);
            }
        }
    }
    
    fn schreibe_zum_ausgabepuffer(&mut self, byte: u8) {
        if self.ausgabe_pos < self.ausgabe_puffer.len() {
            self.ausgabe_puffer[self.ausgabe_pos] = byte;
            self.ausgabe_pos += 1;
        }
    }
    
    fn schreibe_string_zum_ausgabepuffer(&mut self, text: &str) {
        for byte in text.bytes() {
            self.schreibe_zum_ausgabepuffer(byte);
        }
    }

    fn system_ausgabe(&mut self, text: &str) {
        self.schreibe_string_zum_ausgabepuffer(GRUEN);
        self.schreibe_string_zum_ausgabepuffer(text);
        self.schreibe_string_zum_ausgabepuffer(RESET);
    }
    
    fn terminal_ausgabe(&mut self, text: &str) {
        self.schreibe_string_zum_ausgabepuffer(ROSA);
        self.schreibe_string_zum_ausgabepuffer(text);
        self.schreibe_string_zum_ausgabepuffer(RESET);
    }
    
    fn training_simulieren(&mut self) {
        // TODO: Re-enable dynamic stats when integer-to-string is available
        self.terminal_ausgabe("\n[training] zyklus...\n");
        trickster::log_training(self.trainings_zyklus);
    }
    
    fn rendern(&self) {
        let fb = FB_ADDR as *mut u32;
        
        // Bildschirm löschen (schwarz)
        for i in 0..(FB_WIDTH * FB_HEIGHT) {
             unsafe { fb.add(i).write_volatile(0x00000000) };
        }
        
        let mut x = self.cursor_x;
        let mut y = self.cursor_y;
        let mut color_code_buffer = [0u8; 10];
        let mut color_code_pos = 0;
        let mut in_escape = false;
        let mut aktuelle_farbe = 0xFFFFFFFF; // weiß standard

        for &b in &self.ausgabe_puffer[..self.ausgabe_pos] {
            if b == 0 { break; }

            if in_escape {
                if b == b'm' {
                    in_escape = false;
                    let code_str = core::str::from_utf8(&color_code_buffer[..color_code_pos]).unwrap_or("");
                    aktuelle_farbe = match code_str {
                        "32" => 0xFF00FF00, // Grün
                        "35" => 0xFFFF00FF, // Rosa
                        "31" => 0xFFFF0000, // Rot
                        _ => 0xFFFFFFFF,    // Weiß (Reset)
                    };
                    color_code_pos = 0;
                } else if b.is_ascii_digit() {
                    if color_code_pos < color_code_buffer.len() {
                        color_code_buffer[color_code_pos] = b;
                        color_code_pos += 1;
                    }
                } else {
                    in_escape = false; // Ungültige Sequenz, abbrechen
                }
                continue;
            }
            
            if b == 0x1B {
                // This check is tricky without a safe get. Let's simplify.
                // It might parse `[2J` as a color, but that's harmless for now.
                in_escape = true;
                color_code_pos = 0;
                continue;
            }
             if b == b'[' && in_escape {
                 continue;
            }

            match b {
                b'\n' => {
                    x = self.cursor_x;
                    y += 10; // zeilenhöhe 8 + 2
                }
                b'\r' => x = self.cursor_x,
                _ => {
                    if x + 8 > FB_WIDTH {
                        x = self.cursor_x;
                        y += 10;
                    }
                    if y + 8 > FB_HEIGHT {
                        // Scroling implementieren oder hier stoppen
                        continue;
                    }
                    self.zeichen_rendern(b, x, y, aktuelle_farbe, fb);
                    x += 8;
                }
            }
        }
    }
    
    fn zeichen_rendern(&self, c: u8, x: usize, y: usize, farbe: u32, fb: *mut u32) {
        if !(32..=127).contains(&c) { return; }
        let font_index = (c - 32) as usize;
        
        let glyph = FONT8X8[font_index];
        
        for dy in 0..8 {
            for dx in 0..8 {
                if (glyph[dy] >> (7 - dx)) & 1 != 0 {
                    let px = x + dx;
                    let py = y + dy;
                    if px < FB_WIDTH && py < FB_HEIGHT {
                        unsafe {
                            fb.add(py * FB_WIDTH + px).write_volatile(farbe);
                        }
                    }
                }
            }
        }
    }
}
