# M2: Roll20 Conversion Pipeline

**Author:** Gemini & Alithanna
**Date:** 2024-07-26
**Status:** In Progress
**Depends On:** `M2.3_FINAL_SOLUTION_ARCHITECTURE.md`

## 1. Overview

This document outlines the master plan for the **Milestone 2 (M2)** conversion pipeline, which is the primary goal of the TTRPG Converter. The M2 pipeline is designed to convert a complete Roll20 campaign into a playable Foundry VTT world. 

To manage complexity, the conversion is broken into two distinct, sequential stages:

1.  **Stage 1: Platform Conversion.** This stage focuses on the technical translation of data from the Roll20 format to the Foundry VTT format, while preserving the original game system (D&D 5e).
2.  **Stage 2: System Conversion.** This optional, subsequent stage focuses on translating the game system rules and mechanics from D&D 5e to Pathfinder 2e.

This two-stage approach allows for a clean separation of concerns and provides maximum flexibility, allowing users to stop after Stage 1 if they wish to continue playing in the D&D 5e system.

## 2. Stage 1: Platform Conversion (Roll20 5e → Foundry 5e)

-   **Goal:** To create a high-fidelity, 1:1 copy of a Roll20 D&D 5e campaign within the Foundry VTT platform.
-   **Status:** In Progress
-   **Detailed Plan:** The specific implementation tasks for this stage (e.g., creating AutoMapper profiles, building the `ConversionService`, etc.) are defined in **`M3.1_REMAINING_MAPPING_TASKS.md`**.

## 3. Stage 2: System Conversion (Foundry 5e → Foundry PF2e)

-   **Goal:** To intelligently convert the game mechanics of a D&D 5e Foundry world into the Pathfinder 2e system.
-   **Status:** Future Scope (Blocked by Stage 1)
-   **Detailed Plan:** The specific implementation tasks for this stage (e.g., creating the cross-system adapter engine, mapping class features, etc.) are defined in **`M2.5_FOUNDRY_CROSS_SYSTEM_ENGINE.md`**.

## 4. Supporting Architectures

The execution of this milestone relies on several foundational architectural plans:

-   **`M1.5_DATABASE_DRIVEN_ARCHITECTURE.md`**: Defines the EF Core + SQLite database used for caching.
-   **`M1.6_ASSET_MANAGEMENT_ARCHITECTURE.md`**: Defines the plan for resiliently caching and managing all campaign assets.
-   **`M2.1_AUTOMAPPER_INTEGRATION_STRATEGY.md`**: Defines the core strategy of using AutoMapper for all data transformation.

This master plan provides the high-level vision for the M2 pipeline. For specific implementation details, please refer to the detailed plans linked above.
