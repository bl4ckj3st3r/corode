from z3 import *

def prove_memory_isolation(arena_start, arena_size, conditions):
    """
    Beweist formale Speicherisolation für Conditions in corode-core.
    arena_start: Basisadresse der Condition-Arena (0xB0000000)
    arena_size: Größe der Arena (128MB)
    conditions: Liste von (id, base, size)
    """
    s = Solver()
    
    # 1. Definiere die Arena-Grenzen
    arena_end = arena_start + arena_size
    
    # 2. Prüfe jede Condition
    for cond_id, base, size in conditions:
        # Bedingung 1: Muss innerhalb der Arena liegen
        s.add(base >= arena_start)
        s.add(base + size <= arena_end)
        
        # Bedingung 2: Größe muss positiv sein
        s.add(size > 0)
        
        # Bedingung 3: Keine Überlappung mit anderen Conditions
        for other_id, other_base, other_size in conditions:
            if cond_id == other_id:
                continue
            # Eine Condition darf nicht im Speicher einer anderen liegen
            # (base1, base1+size1) und (base2, base2+size2) müssen disjunkt sein
            s.add(Or(base + size <= other_base, other_base + other_size <= base))

    # 3. Das Orakel befragt Z3
    if s.check() == sat:
        print(">>> ORAKEL: Der Speicherplan ist beweisbar sicher! <<<")
        print("Manifest kann signiert werden.")
    else:
        print(">>> ORAKEL: ALARM! Ressourcen-Konflikt entdeckt! <<<")
        print("Z3 kann keine sichere Lösung finden.")

# Beispiel 1: Sicherer Plan
print("Test 1: Sicherer Plan")
prove_memory_isolation(0xB0000000, 128*1024*1024, [
    (1, 0xB0000000, 0x1000), # Cond 1: 4KB am Anfang
    (2, 0xB0001000, 0x2000)  # Cond 2: 8KB direkt danach
])

# Beispiel 2: Konflikt (Überlappung)
print("\nTest 2: Konflikt (Überlappung)")
prove_memory_isolation(0xB0000000, 128*1024*1024, [
    (1, 0xB0000000, 0x1000), # Cond 1: 4KB
    (2, 0xB0000800, 0x1000)  # Cond 2: 4KB (startet aber mitten in Cond 1!)
])
