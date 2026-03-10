# corode-core

**Isolation statt Abstraktion. Determinismus statt Interrupt. Harmonie statt Krieg.**

[![Lizenz: GPL v3](https://img.shields.io/badge/Lizenz-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

---

`corode-core` ist kein weiteres Betriebssystem. Es ist der Versuch, die seit 80 Jahren etablierte Von-Neumann-Architektur zu Ende zu denken und ihre fundamentalen Probleme zu lösen. Anstatt Prozesse um knappe Ressourcen kämpfen zu lassen, etabliert `corode-core` eine beweisbare Ordnung, die vor der Ausführung mathematisch sicherstellt, dass keine Konflikte entstehen.

**Race Conditions sind hier nicht nur selten. Sie sind unmöglich.**

## Das Kernprinzip: Das Condition Paradigma

Alles im System ist eine **Condition**: ein isolierter, vollständig aufgelöster, deterministisch ausführbarer Zustand. Eine Condition existiert nur, wenn ihre Existenz formal bewiesen werden kann. Sie läuft in einem eigenen, durch Hardware (RISC-V PMP) geschützten Speicherbereich und verschwindet, wenn ihre Aufgabe erfüllt ist.

- **Kein Kampf:** Ressourcen werden nicht zur Laufzeit verhandelt, sondern zur Compile-Zeit im **Manifest** festgeschrieben.
- **Kein Chaos:** Es gibt keinen Scheduler, der Entscheidungen trifft. Das System fließt nach den im Manifest bewiesenen Regeln.
- **Kein Raten:** Der Z3 SMT Solver beweist die Korrektheit jeder Condition, bevor sie entsteht.

## Aktueller Status: Genesis

Das Projekt befindet sich im `Genesis`-Stadium. Der erste, lauffähige Code existiert in `main.rs` auf bare-metal RISC-V. Dieser "Genesis-Commit" beweist die grundlegende Machbarkeit des Konzepts:

- **Kompilierung:** `no_std` und `no_main` sind umgesetzt.
- **Hardware-Isolation:** Physische Speicherprotektion (PMP) ist konzeptionell verankert.
- **Stabilität:** Ein erster Trap-Handler ("Harlekin") fängt bereits Zugriffsverletzungen ab und verhindert den Systemabsturz.

## Die Roadmap

Die vollständige Vision ist in der [ROADMAP.md](ROADMAP.md) detailliert. Die nächsten Meilensteine sind:

1.  **Phase 1: Beweisbare Stabilität:** Vollständige Implementierung der PMP-isolierten Speicherarchitektur und des fälschungssicheren "Trickster"-Loggers.
2.  **Phase 2: Infrastruktur:** Aufbau der `Z3³`-Speicherlogistik und des `SLS`-Speichersystems für maximale CPU-Sättigung.
3.  **Phase 3: Intelligenz:** Entwicklung des "Quarantäne Neural Network" zur automatischen Fehleranalyse und des "Weightless Agent Training" für autonome Systemoptimierung.

## Die Philosophie: Hardware-Sozialdemokratie

Jede Condition hat das Recht auf Existenz, solange ihre Anforderungen die Systemstabilität nicht gefährden. Sie erhält ihren festen Platz, darf nicht mehr beanspruchen, ist aber sicher, dass sie ihren Raum bekommt. Das ist keine technische, sondern eine philosophische Entscheidung gegen das "Survival of the Fittest"-Prinzip klassischer Betriebssysteme.

Dieses Projekt ist Open Source (GPLv3) und wird von der **Trickster IT SE** vorangetrieben.
