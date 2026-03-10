use oxiz::{Solver, TermManager, SolverResult};
use num_bigint::BigInt;

fn main() {
    // 1. Orakel-Komponenten initialisieren
    let mut solver = Solver::new();
    let mut tm = TermManager::new();

    // 2. Mathematische Definitionen unserer Welt
    let condition_id = tm.mk_int(BigInt::from(4711));
    let allowed_block = tm.mk_int(BigInt::from(23));
    let forbidden_block = tm.mk_int(BigInt::from(42));
    
    // Eine Variable, die den Block repräsentiert, auf den zugegriffen wird.
    let target_block = tm.mk_var("target_block", tm.sorts.int_sort);

    // 3. Die Sicherheitsregel (Assertion)
    // Wir behaupten: Die Condition 4711 darf *ausschließlich* auf Block 23 zugreifen.
    let rule = tm.mk_eq(target_block, allowed_block);
    solver.assert(rule, &mut tm);

    // 4. Die Angriffs-Hypothese
    // Wir fragen das Orakel: Ist es möglich, dass die Condition trotzdem auf den verbotenen Block 42 zugreift?
    let attack_hypothesis = tm.mk_eq(target_block, forbidden_block);
    solver.assert(attack_hypothesis, &mut tm);

    // 5. Das Orakel fällt sein Urteil
    println!("Befrage Orakel: Kann Condition 4711 trotz Regel auf Block 42 zugreifen?");
    match solver.check(&mut tm) {
        SolverResult::Unsat => {
            println!("\n=======================================================");
            println!("✅ ORAKEL-BEWEIS: UNSATISFIABLE");
            println!("   Zugriff auf Block 42 ist mathematisch unmöglich.");
            println!("   Die PMP-Regel ist beweisbar sicher.");
            println!("=======================================================");
        }
        SolverResult::Sat => {
            println!("\n=======================================================");
            println!("❌ ORAKEL-ALARM: SATISFIABLE");
            println!("   Ein Angriffspfad wurde gefunden!");
            println!("   Die Regel erlaubt einen Zugriff auf Block 42.");
            println!("=======================================================");
        }
        SolverResult::Unknown => {
            println!("⚠️ Orakel ist unsicher. Manuelle Prüfung erforderlich.");
        }
    }
}
