# M3: Pathfinder 1e Conversion Pipeline

**Author:** Gemini & Alithanna
**Date:** 2024-07-26
**Status:** Future Scope
**Depends On:** `M2_ROLL20_5E_TO_FOUNDRY_PF2E.md`

## 1. Overview

This document outlines the master plan for the **Milestone 3 (M3)** conversion pipeline. The M3 pipeline is a major future feature that will enable the conversion of a complete Roll20 campaign using the Pathfinder 1e system into a playable Foundry VTT world, with an optional final step to convert the system to Pathfinder 2e.

This milestone will reuse the core architecture and services developed for the M2 pipeline, including the asset management system and the cross-system conversion engine. The primary work will be to implement the specific mapping logic for the PF1e and PF2e systems.

Like the M2 pipeline, the conversion is broken into two distinct, sequential stages:

1.  **Stage 1: Platform Conversion.** This stage focuses on the technical translation of data from the Roll20 format to the Foundry VTT format, while preserving the original game system (Pathfinder 1e).
2.  **Stage 2: System Conversion.** This optional, subsequent stage focuses on translating the game system rules and mechanics from Pathfinder 1e to Pathfinder 2e.

## 2. Stage 1: Platform Conversion (Roll20 PF1e → Foundry PF1e)

-   **Goal:** To create a high-fidelity, 1:1 copy of a Roll20 Pathfinder 1e campaign within the Foundry VTT platform.
-   **Status:** Not Started
-   **Implementation Details:** This will require creating a new set of AutoMapper profiles and resolvers specifically for the Roll20 PF1e character sheet and data structures. The work will be similar in scope to the tasks defined in `M3.1` for the 5e system.

## 3. Stage 2: System Conversion (Foundry PF1e → Foundry PF2e)

-   **Goal:** To intelligently convert the game mechanics of a Pathfinder 1e Foundry world into the Pathfinder 2e system.
-   **Status:** Not Started
-   **Implementation Details:** This will require creating a new set of system converters that plug into the `M2.5` cross-system engine. The work will involve significant logic to handle the complex mapping between the two systems (e.g., skills consolidation, action economy changes, feat translation).

## 4. Supporting Architectures

The execution of this milestone will rely on the successful completion of the M2 pipeline and its supporting architectures:

-   **`M2.3_FINAL_SOLUTION_ARCHITECTURE.md`**: The overall N-tier application structure.
-   **`M2.5_FOUNDRY_CROSS_SYSTEM_ENGINE.md`**: The interface-driven engine that will be reused to perform the PF1e-to-PF2e conversion.

This master plan provides the high-level vision for the M3 pipeline. Detailed task breakdowns for this milestone will be created in new documents when development is ready to begin.
