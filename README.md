# CORODE

**CORODE** is an experimental deterministic, state-oriented system architecture.

It is built around a simple idea:

**prepare → prove → admit → execute**

Instead of treating runtime as a giant self-managing chaos machine, CORODE separates soft preparation from hard execution truth.

## Pipeline

1. **Sidekernel / Preparing space**  
   Manifest creation, dependency resolution, comparison, logging, retraining, graph building.

2. **Pre-kernel boundary**  
   The point where the system becomes hard.

3. **Allocator**  
   Assigns deterministic Cage IDs.

4. **Solver**  
   Checks whether a manifest is valid enough for a condition to exist.

5. **Kernel**  
   Intentionally dumb. Only assigns runtime role classes:
   - L0 – system-critical
   - L1 – pipeline-bearing
   - L2 – engines / system services
   - L3 – general computing

6. **Orchestrator**  
   Keeps cores, execution lanes, and saturation pipelines meaningfully occupied.

7. **Execution**  
   Only execution carries hard runtime truth.

## CorodeQ

**CorodeQ** is the external working space of the system.

It handles four areas:

- Training / Preparing
- Quarantine
- Smart Logging
- Comparing

Its agents are simple, temporary, and forgetful:

- they work on split tasks
- update weighted knowledge
- then forget again

**Agents forget. The weighted space remembers.**

## Design direction

CORODE explores:

- explicit state transitions
- weighted state spaces
- deepsleep / resume logic
- bounded responsibility domains
- deterministic admission
- energy-aware execution
- alternative runtime design beyond standard POSIX assumptions

## Repository status

This repository is an active experimental architecture and implementation space.

It currently contains:

- core source files
- sidekernel work
- examples
- technical documentation
- roadmap material

## Why RISC-V

RISC-V is interesting here as a long-term open foundation for experimenting with:

- alternative execution models
- deterministic resource handling
- sidekernel / sidecore separation
- explicit runtime roles

---

**CORODE is a system where conditions are prepared, proven, admitted, role-assigned, and only then executed.**