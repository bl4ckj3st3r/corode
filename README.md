# CORODE

**CORODE** is an experimental deterministic, state-oriented system architecture focused on explicit preparation, proof, admission, and execution.

It is built around the idea that runtime should not be a giant self-managing chaos machine.  
Instead, work should be prepared first, proven next, admitted in a controlled way, and only then executed.

## Core idea

CORODE is based on a simple principle:

**prepare → prove → admit → execute**

This means:

- no silent existence of conditions
- no implicit runtime truth
- no uncontrolled mixing of preparation, execution, logging, learning, and recovery
- no assumption that everything observed should immediately become valid system state

A condition must first be structurally prepared, then proven valid enough to exist, then assigned its runtime role, and only then executed.

## Why

Modern systems often spend a large amount of time on self-management:

- scheduling noise
- runtime overhead
- fragmentation
- background work
- unnecessary rendering and compatibility logic
- constant interrupt and housekeeping pressure

CORODE explores a different direction:

- smaller bounded responsibility domains
- explicit state transitions
- deterministic admission
- weighted state spaces
- execution with minimal ambiguity
- energy-aware runtime behavior
- deepsleep and resume as first-class concepts

The goal is not maximum peak performance at any cost.

The goal is a **respectful computation space** where the machine can spend more time computing and less time fighting itself.

---

# Architecture overview

## 1. Sidekernel / Preparing space

This is the soft outer workspace.

Here, the system can:

- prepare manifests
- resolve dependencies
- compare structures
- retrain and refactor
- containerize or virtualize if needed
- build graphs
- collect and update weighted knowledge

At this stage, things are still soft.  
Nothing here automatically becomes runtime truth.

## 2. Pre-kernel boundary

This is the point where the system becomes hard.

The pre-kernel boundary separates preparation from real admission into the runtime domain.

## 3. Allocator

The allocator assigns deterministic **Cage IDs**.

A Cage ID is not the semantic identity of a condition.  
It is the deterministic runtime/deepsleep holder assigned to it.

## 4. Solver

The solver checks the manifest and decides whether a condition is valid enough to exist.

If the manifest fails, the condition does not exist.

This is one of the central principles of CORODE:

**Only pre-proven conditions are existent.**

## 5. Kernel

The kernel is intentionally **dumb**.

It does not try to be an all-knowing runtime brain.  
It only assigns runtime role classes:

- **L0** – system-critical
- **L1** – pipeline-bearing
- **L2** – engines / system services
- **L3** – general computing, non-essential to core system integrity

Once the role is assigned, the condition becomes active.

## 6. Orchestrator

The orchestrator is responsible for execution flow.

It does not know everything about the system.  
It only knows enough to keep:

- cores
- execution lanes
- saturation line storage pipelines

meaningfully occupied.

It sorts conditions by priority and flow, but it is not a naive priority scheduler.

For example, an `L3` condition may be executed before an `L0` condition if that improves saturation and avoids idle gaps without harming system integrity.

## 7. Execution

Only execution carries hard runtime truth.

Preparation may suggest.  
Comparison may observe.  
Logging may collect.  
Training may improve.  
But execution is where the system actually becomes real.

---

# CorodeQ

**CorodeQ** is the external working space of the system.

It is not the main runtime.  
It is a separate system-service domain with symmetric clusters of simple specialists.

CorodeQ is responsible for four main areas:

- **Training / Preparing**
- **Quarantine**
- **Smart Logging**
- **Comparing**

These agents are intentionally simple, bounded, and forgetful.

They do not carry private long-term truth.  
They work on small split tasks, update weighted knowledge, and then forget.

This keeps memory usage controlled and avoids giant agent brains with hidden state.

## CorodeQ principles

- agents are specialized
- agents work in temporary teams per task
- agents update weighted knowledge
- agents forget after the task is complete
- truth does not live inside the agents
- truth remains bound to execution and valid runtime admission

In short:

**agents forget, the weighted space remembers**

---

# State model

CORODE is centered around **conditions**.

A condition is the actual semantic unit of the system.

Important distinction:

- **Condition** = semantic identity
- **weighted state space** = dynamic state of the condition
- **Cage ID** = deepsleep/runtime holder
- **manifest** = proof and structure basis
- **runtime form** = rebuildable execution form

This allows refactoring without losing condition identity.

A condition can be rebuilt for runtime while still remaining the same condition.

---

# Design principles

- deterministic admission
- explicit state transitions
- small responsibility domains
- no hidden truth
- weighted state handling
- deepsleep instead of wasteful constant activity
- preparation outside runtime
- execution as final truth
- horizontal scaling of bounded parts instead of giant central runtime knowledge
- energy as primary resource concern

---

# RISC-V direction

CORODE is being explored with **RISC-V** as a long-term foundation because of its openness and suitability for architectural experimentation.

This includes interest in:

- alternative execution models
- deterministic resource handling
- sidecore / sidekernel separation
- explicit runtime roles
- possible future hardware designs for more controlled execution and resource visibility

At the same time, CORODE is not tied to one ISA conceptually.  
The architectural logic comes first.  
ISA-specific realization comes later.

---

# Current project areas

CORODE is connected to several active directions, including:

- deterministic execution architecture
- weighted condition/state systems
- deepsleep/resume behavior
- sidekernel workspaces
- a modernized planning/proof/execution model inspired by Plankalkül
- WireGuard/P2P state machine work
- workflow and care-oriented operating system ideas
- smart dock / hardware trust anchor concepts
- Paula language and execution systems as early CORODE software bodies

---

# Status

This is an active experimental architecture and research project.

It is not a finished product.  
It is a living system model, execution philosophy, and software/hardware research direction.

---

# Philosophy

CORODE is based on a simple belief:

> Not everything that appears should automatically exist.  
> Not everything that exists should automatically execute.  
> And not everything that executes should carry the same role.

The system should remain legible, bounded, and worthy of the machine it runs on.

---

# Short definition

**CORODE is a deterministic, state-oriented execution architecture where conditions are prepared, proven, admitted, role-assigned, and only then executed. It separates soft preparation from hard runtime truth and uses weighted state spaces, deepsleep, and bounded responsibility domains to reduce runtime chaos and overhead.**