# CORODE: An Experimental Computing Architecture and Deterministic OS Paradigm

CORODE is a research project exploring a deterministic computing architecture for the RISC-V instruction set. It is not merely a kernel, but a holistic paradigm designed from first principles to achieve predictable performance, robust security, and low-level resource efficiency. The system is implemented in Rust and adheres to a `no_std` environment.

---

## 1. Motivation & Problem Statement

Modern general-purpose operating systems are defined by inherent non-determinism, complex abstractions (e.g., virtual memory, preemptive schedulers), and consequently, vast attack surfaces. This leads to unpredictable performance under load and a constant, reactive struggle to mitigate security vulnerabilities.

CORODE investigates an alternative path by enforcing determinism and simplicity at the architectural level. We posit that for a significant class of applications—particularly in embedded systems, critical infrastructure, and high-security contexts—the complexity of traditional OS models is not a prerequisite, but a liability.

## 2. The CORODE Computation Architecture

It is crucial to understand CORODE not as a kernel, but as a **computing architecture**. The software component, the *core*, is a minimal implementation and enforcer of a rule-set called the **Corode Computation Model (CCM)**. This model redefines the fundamental relationship between hardware, software, and execution.

## 3. Architecture Overview

- **Minimalist Rust Core:** A small, `no_std` core that runs directly on RISC-V hardware.
- **Hardware-Enforced Isolation:** Security boundaries are enforced directly by the RISC-V Physical Memory Protection (PMP) unit, not by complex software layers.
- **Heapless Memory Model:** The system forgoes a traditional dynamic heap in favor of the `ZuseAllocator`, a deterministic block manager.
- **Condition-Based Execution:** Execution is modeled as a sequence of deterministic state transitions driven by `Conditions`.
- **Sidekernel for Complexity:** Non-deterministic or complex services (e.g., networking, filesystems) are offloaded to a separate, less-privileged `Sidekernel`.

## 4. System Ontology

The CORODE model is built on a precise vocabulary:

- **Condition:** The fundamental unit of execution. A `Condition` represents a potential, discrete state transition in the system, triggered by hardware events or internal logic. It replaces the concept of processes or threads.
- **Cage:** A fixed-size, PMP-isolated memory region. `Cages` are the only form of allocatable memory in the core system.
- **Role:** A set of permissions defining which `Cages` a `Condition` can access and what operations it is permitted to perform.
- **State:** The complete, measurable configuration of the system at any given moment, defined by the set of all `Cages` and the current `Conditions`.

## 5. Memory Architecture: The ZuseAllocator & Cages

CORODE rejects the notion of a dynamic heap. Memory is managed by the **ZuseAllocator**, a deterministic block allocator named in honor of Konrad Zuse.

- **Allocation:** The allocator does not manage arbitrary byte-sized requests. It allocates and deallocates entire `Cages`.
- **Predictability:** This model eliminates memory fragmentation and non-deterministic allocation times. Memory operations become predictable and verifiable.
- **Isolation:** Each `Cage` is an atomic unit of memory protected by the RISC-V PMP.

## 6. Security Model: PMP-Enforced Default-Deny

The primary security principle is **Default-Deny**, enforced at the hardware level.

- **PMP as Arbiter:** All memory access is forbidden unless explicitly permitted by a PMP configuration. A `Condition` attempting to access memory outside its assigned `Cages` triggers a hardware trap, not a software-handled page fault.
- **Minimal Attack Surface:** The core is minimal by design, exposing few primitives and drastically reducing the potential attack surface. Security is an architectural feature, not an add-on.

## 7. Execution Model: Deterministic State Transitions

The system evolves through a sequence of discrete, deterministic state transitions. A given state `S_n` and a triggering `Condition` `C` will **always** produce the exact same subsequent state `S_n+1`. This model is designed to eliminate sources of performance jitter common in traditional OSes, such as pre-emptive scheduling, I/O blocking, and garbage collection pauses.

## 8. The Sidekernel Concept

Non-deterministic, complex, or untrusted tasks are incompatible with the deterministic core. Such tasks are delegated to the **Sidekernel**.

- **Isolating Non-Determinism:** The Sidekernel can be a more traditional OS (like a Linux instance) or a dedicated runtime running in a lower-privilege mode.
- **Controlled Communication:** The CORODE core and the Sidekernel interact exclusively through a strictly defined, message-passing interface. This ensures that the determinism and security of the core are never compromised by the Sidekernel's operations.

## 9. Project Status

**Highly Experimental.** CORODE is in an early stage of research and development. It serves as a proof of concept for the core architectural ideas and is **not suitable for any production use**. The current implementation is tested within the QEMU emulator.

## 10. Roadmap

- **Phase 1 (Current):** Stabilize the core deterministic model, `ZuseAllocator`, and PMP enforcement within a QEMU environment.
- **Phase 2:** Define and implement the initial message-passing API for the Sidekernel interface.
- **Phase 3:** Develop a basic Sidekernel prototype and demonstrate controlled communication with the CORODE core.
- **Phase 4:** Port the CORODE core to physical RISC-V hardware (e.g., SiFive boards).
- **Phase 5:** Explore more complex `Condition` and `Role` interactions for building higher-level, verifiable abstractions.

## 11. Supported Hardware

- **Primary Target:** RISC-V (RV64GC)
- **Emulation:** Currently developed and tested using QEMU (`qemu-system-riscv64` with the `virt` machine).
- **Physical Hardware:** Support for physical boards is a key future goal.

## 12. Contributing

We welcome contributions from researchers, system developers, and security experts who are interested in exploring alternatives to traditional OS design.

- Please use the GitHub issue tracker to report bugs, ask questions, or propose new ideas.
- Follow the conventional Rust coding style (`rustfmt`).
- For significant changes, please open an issue to discuss the design first.
