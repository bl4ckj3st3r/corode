# corode-core: Das Architektonische Manifest

**Fassung: 2.0 (Wiedergeburt)**

---

## 1. Das übergeordnete Paradigma: Hardware-Sozialdemokratie

Die Architektur definiert einen radikalen Paradigmenwechsel. Das klassische Von-Neumann-Modell basiert auf Konkurrenz: Prozesse kämpfen um Speicher und Rechenzeit, was zu Komplexität, Sicherheitslücken und Ineffizienz führt. Unser Modell ersetzt diesen Kampf durch eine **Hardware-Sozialdemokratie**. Die zentrale These lautet: **"Everything is an isolated condition"**. Jede Einheit im System – sei es ein Prozess, ein Speicherbereich oder ein Kommunikationskanal – wird als eigenständige, isolierte "Condition" betrachtet. Diese Conditions konkurrieren nicht, sondern existieren gleichberechtigt in ihrer vordefinierten Rolle, erhalten genau die benötigten Ressourcen zugeteilt und kommunizieren nur über definierte, sichere Kanäle.

## 2. Architektonische Grundprinzipien (Die 8 Säulen)

Die gesamte Architektur stützt sich auf acht Kernprinzipien, die das Verhalten des Systems deterministisch festlegen:

1.  **Isolation statt Abstraktion:** Anstatt Funktionen in undurchsichtigen Softwareschichten zu verstecken, wird jede Condition logisch und physikalisch isoliert. Dies eliminiert Seiteneffekte und unerwünschte Abhängigkeiten.
2.  **Harmonie statt Krieg:** Das System basiert nicht auf Wettbewerb ("Survival of the fittest"), sondern auf Koexistenz. Jede Condition hat eine inhärente Rolle, die ihren Ressourcenbedarf und ihre Priorität definiert.
3.  **Determinismus statt Interrupt:** Das Verhalten ist vollständig vorhersagbar. Gleiche Eingaben führen unter gleichen Bedingungen stets zu gleichen Ausgaben und Laufzeiten.
4.  **Block-ID statt Pointer:** Der klassische, sicherheitskritische Speicherzeiger (Pointer) wird abgeschafft. Jede Condition greift auf Speicher über eine eindeutige **Block-ID** und einen Offset zu.
5.  **Sättigung statt Überlast:** Ein Vollauslastung von 90-100% führt nicht zu einem Absturz. Das System schichtet um, versetzt unwichtige Blöcke in den Tiefschlaf und hält kritische Ressourcen aktiv.
6.  **Ultra-Microkernel:** Der Kernel ist drastisch reduziert ("nur ein paar Bits"). Seine einzige Aufgabe ist die eines "Türstehers" für die Isolation der Conditions.
7.  **Bedarf statt Gier:** Ressourcenzuteilung erfolgt nicht durch Beantragung, sondern durch Zuweisung basierend auf dem im Trainingsmodus ermittelten Bedarf einer Condition.
8.  **Hardware-Sozialdemokratie:** Die ultimative Konsequenz: Jede Condition ist in ihrer Rolle gleichberechtigt, erhält garantiert ihren benötigten Platz und kann keine Ressourcen über ihren Bedarf hinaus beanspruchen.

## 3. Technische Umsetzung auf RISC-V (Der "Z3³"-Kern)

-   **Physischer Speicherschutz (PMP) als Fundament:** Das Herzstück der Hardware-Isolation. Jede Condition wird einer oder mehreren PMP-Regionen zugeordnet, sodass ein Zugriff auf fremden Speicher physikalisch unmöglich ist.
-   **Erweiterte Speicherlogistik (Z3³):** Definiert Speicher in festen Blöcken, die über eine eindeutige **Block-ID** adressiert werden. Macht herkömmliche Speicherzeiger überflüssig.
-   **Der Harlekin-Kernel:** Minimaler Kernel (Proof-of-Concept in Rust), der PMP-Regionen setzt und einen Trap-Handler ("Harlekin") installiert, um Zugriffsverletzungen abzufangen.
-   **L0-Agenten:** Unabhängige Mikrocontroller (z.B. STM32), die als "physikalische Intelligenz" auf dem Mainboard Strom, Temperatur und Zugriffsmuster überwachen und autonom eingreifen können.
-   **Sidekernels:** Isolierte Umgebungen für die notwendige Kommunikation zwischen Conditions, oft über sichere Protokolle wie WireGuard.

## 4. Systemdynamik: Vom Training zur Sättigung

1.  **Trainingsmodus:** Neue Conditions werden in einer Sandbox analysiert, um ihr Laufzeitverhalten und ihren exakten Ressourcenbedarf (Profil) zu ermitteln.
2.  **Deterministische Zuweisung:** Basierend auf diesem Profil wird der Condition bei einem produktiven Start exakt die benötigte Anzahl an Speicherblöcken und Rechenzeit zugewiesen.
3.  **Sättigungsmanagement:** Bei hoher Systemlast werden Conditions mit niedriger Priorität in einen Tiefschlaf-Zustand versetzt, um kritische Operationen nicht zu beeinträchtigen.

## 5. Komponenten und Implementierungsstatus

| Komponente | Funktion | Implementierungsstatus |
| :--- | :--- | :--- |
| **Z3³-Speicherlogistik** | Deterministische Speicherverwaltung mit Block-ID | Konzept, erster Code |
| **PMP (Physical Memory Protection)** | Hardware-Isolation pro Condition | **Verfügbar in RISC-V** |
| **Harlekin-Kernel** | Minimaler Kernel mit Trap-Handler | Proof-of-Concept (Rust) |
| **Orakel (OxiZ/Z3)** | Host-Tool für formale Verifikation | Proof-of-Concept (Rust) |
| **L0-Agenten** | Physikalische Überwachung (Strom, Temperatur) | Konzept, Hardware-Prototyp geplant |
| **Sidekernels** | Isolierte Umgebungen für Kommunikation | In Entwicklung |
| **4bit Breaker** | Hardware-Impulsgeber für deterministische Tests | In Entwicklung (durch "CTO Kevin") |
