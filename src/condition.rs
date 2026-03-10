//! Bedingungsvariablen – für die Synchronisation zwischen Sidekernel-Komponenten.
//!
//! Dieses Modul würde einen Mechanismus bereitstellen, mit dem Sidekernel-Tasks
//! warten können, bis eine bestimmte Bedingung erfüllt ist. Dies ist entscheidend
//! für die nebenläufige Ausführung von Code in einer komplexeren Laufzeitumgebung.

pub struct Condition;

impl Condition {
    pub fn new() -> Self {
        Self
    }

    pub fn wait(&self) {
        // Blockiert den aufrufenden Task, bis `signal` aufgerufen wird.
    }

    pub fn signal(&self) {
        // Weckt einen wartenden Task auf.
    }
}
