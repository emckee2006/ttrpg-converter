# M4: Foundry Enhancement Pipeline

**Author:** Gemini & Alithanna
**Date:** 2024-07-26
**Status:** Future Scope
**Depends On:** `M1.7_ADVANCED_ASSET_PIPELINE.md`, `M3.1_REMAINING_MAPPING_TASKS.md`

## 1. Overview

This document outlines the architecture for a **Milestone 4 (M4)** enhancement pipeline. This is a distinct workflow that operates on **existing Foundry VTT worlds**. Its purpose is not to convert from another platform, but to take a Foundry world and improve it by applying the advanced processing capabilities developed for this project.

This provides significant value to users by allowing them to optimize and upgrade worlds they may already be using.

## 2. Core Features

1.  **Asset Optimization:** The pipeline will reuse the services defined in **`M1.7`** to scan an entire Foundry world's assets and apply our advanced optimization (WEBP conversion, downscaling, etc.) to all images, audio, and video files.
2.  **Compendium Matching:** The pipeline will introduce a new, advanced feature to scan all "homebrew" items, spells, and monsters within a world. It will use fuzzy matching and heuristics to find equivalent entries in the user's official compendia and offer to replace the homebrew versions with the high-quality, official ones.

## 3. Implementation Plan

This feature will be implemented as a new, selectable pipeline in the GUI, as envisioned in the `M6_GUI_APPLICATION.md` v2.0 plan.

### Phase 1: Create the Enhancement Service
-   **Task:** Create a new `EnhancementService` in the `TTRPGConverter.Processing` project.
-   **Logic:** This service will be responsible for loading a Foundry world, iterating through its assets and database files, and passing them to the appropriate sub-services.

### Phase 2: Integrate Asset Optimization
-   **Task:** Inject the `IAssetMapper` and its underlying services into the `EnhancementService`.
-   **Logic:** The service will scan the world for all asset paths and use the asset processing pipeline to create optimized versions.

### Phase 3: Implement Compendium Matching
-   **Task:** Create a new `CompendiumMatcher` service.
-   **Logic:** This service will implement the fuzzy matching and confidence scoring algorithms required to compare homebrew entities against the official compendium cache.

This plan provides a clear roadmap for a powerful, value-add feature that leverages the other architectural components we have designed.