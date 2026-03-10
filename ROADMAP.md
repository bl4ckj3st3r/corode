# corode-core Master Roadmap

**Status: 05. März 2026**
**Fassung: 2.0 (Wiedergeburt)**

---

## Phase 0: Das Fundament - Philosophie & Genesis

- [x] **Condition Paradigma definieren:** "Everything is a Condition." Die philosophische Grundlage ist gelegt.
- [x] **Historische Einordnung:** Abgrenzung von Von Neumann und Turing ist klar definiert.
- [x] **Hardware-Sozialdemokratie formulieren:** Das politische Modell des Systems ist etabliert.
- [x] **Genesis-Commit erstellen:** Der erste Code (`main.rs`) wurde geschrieben und auf GitHub veröffentlicht. Das Projekt existiert.
- [x] **Grundlegenden Toolchain-Setup:** `no_std`, `no_main`, bare-metal RISC-V Kompilierung ist funktional.

---

## Phase 1: Der Kern - Beweisbare Stabilität

- [x] **PMP (Physical Memory Protection) als Sicherheitsbasis:** Das Konzept der Hardware-Isolation ist als Kernkomponente verankert.
- [x] **Trap-Handler "Harlekin" implementieren (POC):** Ein fundamentaler Trap-Handler in `main.rs` fängt bereits Zugriffsverletzungen ab.
- [ ] **PMP Header Cage entwerfen:** Das Konzept zur physischen Isolierung von Speicherbereichen muss noch in ein konkretes Hardware-Layout überführt werden.
- [ ] **PMP Vektor Map ausarbeiten:** Detaillierte Planung der Speicherregionen und ihrer Zugriffsrechte.
- [ ] **Logger "Trickster" implementieren:** Ein passiver, nicht-intrusiver Logger, der Systemereignisse fälschungssicher protokolliert.
- [ ] **4-Bit Breaker (Hardware-Proof):** Physischer Bau des sichtbaren PMP-Zustands-Beweises (LEDs, Kupferlanes).
- [ ] **Z3 SMT Solver Integration:** Anbindung des Z3 Solvers zur formalen Verifikation von Conditions *vor* ihrer Erzeugung.

---

## Phase 2: Die Infrastruktur - Effizienz & Fluss

- [ ] **Z3³ Speicherlogistik entwickeln:**
    - [ ] **Zuse (Deterministisches Memory Management):** Implementierung des BlockID-basierten Speichersystems. Kein Heap.
    - [ ] **Zorro III (Kontrollierte Asynchronität):** Implementierung des "Tiefschlaf"-Mechanismus für wartende Conditions.
- [ ] **SLS (Saturation Line Storage) implementieren:** Aufbau der einheitlichen Speicherhierarchie von Cache bis NVMe.
- [ ] **"Turing-Band" entwickeln:** Implementierung des Mechanismus zur Auslagerung und fehlerfreien Rehydrierung von inaktiven Conditions.
- [ ] **Manifest-Generator bauen (Trainingsmodus):** Entwicklung des Tools, das zur Compile-Zeit das System analysiert (WCET, Speicher) und die unveränderliche "Manifest"-Datei erstellt.

---

## Phase 3: Die Intelligenz - Autonome Anpassung & Heilung

- [ ] **"Quarantäne Neural Network" entwerfen & implementieren:**
    - [ ] **Konzept:** Ein separates, neuronales Netz, das fehlerhafte Conditions (die im Training durch den Z3-Beweis fallen) analysiert.
    - [ ] **Ziel:** Das Netz lernt aus den Fehlern und schlägt Modifikationen am Code oder den Parametern vor, um die formale Verifikation zu bestehen.
    - [ ] **Implementierung:** Aufbau der "Quarantäne"-Umgebung, in der fehlerhafte Conditions sicher analysiert werden können.
- [ ] **"Weightless Agent Training" Framework entwickeln:**
    - [ ] **Konzept:** Ein System für "L0-Agenten" – winzige, KI-gesteuerte Programme, die direkt auf der Hardware leben.
    - [ ] **"Weightless":** Die Agenten selbst haben keinen Zustand. Ihr "Wissen" ist in der Topologie des Netzwerks und den Interaktionsregeln codiert, nicht in internen Variablen. Sie sind reine Funktion.
    - [ ] **Ziel:** Autonome Systemoptimierung, Hardware-Monitoring und selbstständige Anpassung an veränderte Bedingungen (z.B. thermische Probleme, Hardware-Ausfälle).
    - [ ] **Implementierung:** Schaffung der Infrastruktur, um diese Agenten sicher und isoliert auszuführen.

---

## Phase 4: Das Ökosystem - Erweiterung & Anwendung

- [ ] **Sidekernels entwickeln:** Konzept und Implementierung für isolierte Kernel-Instanzen, die sicher miteinander kommunizieren können.
- [ ] **FPGA-Prototyp erstellen:** Portierung des corode-core auf eine anpassbare FPGA-Hardware-Plattform.
- [ ] **Enterprise Security Token (TRUMP) Prototyp:**
    - [ ] **Konzept:** "The Remote Unbreakable Master Passport". Ein auf corode-core basierender, MCU-basierter Sicherheitstoken.
    - [ ] **Abgrenzung:** Closed-Source, kommerzielles Produkt.
    - [ ] **Implementierung:** Bau eines Hardware-Prototypen mit WireGuard P2P-Kommunikation.
- [ ] **QEMU Demo erstellen:** Erstellung einer einfach zu nutzenden Demo-Umgebung für Raspberry Pi Emulation.
- [ ] **Dokumentation finalisieren:** Erstellung einer vollständigen, öffentlichen Dokumentation für Nutzer und Entwickler.

---
