# M7: Pathbuilder 2e Integration

**Author:** Gemini & Alithanna
**Date:** 2024-07-26
**Status:** Future Scope
**Depends On:** `M6_GUI_APPLICATION.md`

## 1. Overview

This document outlines the implementation plan for a **Milestone 7 (M7)** feature: the integration of a new conversion source, **Pathbuilder 2e**. This pipeline will allow users to import characters directly from the Pathbuilder 2e web application and convert them into playable Foundry VTT actors.

This represents a significant expansion of the converter's capabilities, demonstrating that our core architecture is flexible enough to support new, distinct conversion workflows beyond the initial Roll20 focus.

## 2. Core Features

-   **API Client:** A new service to interact with the Pathbuilder 2e JSON export API.
-   **Platform-Only Conversion:** Since Pathbuilder exports PF2e data and the target is Foundry PF2e, no complex *system* conversion is required. The work is focused on *platform* format translation.
-   **GUI Integration:** A new tab or section in the v2.0 GUI (as planned in `M6`) to manage the Pathbuilder import workflow.
-   **Batch Processing:** Allow users to import an entire party of characters from a list of Pathbuilder URLs.

## 3. Implementation Plan

### Phase 1: API and Data Services
-   **Task:** Create a `PathbuilderService` in the `Infrastructure` project to handle HTTP requests to the Pathbuilder API, including rate-limiting to act as a good network citizen.
-   **Task:** Create AutoMapper profiles in the `Core` project to map the Pathbuilder JSON model to our `FoundryPf2eActor` model.

### Phase 2: Processing Service
-   **Task:** Create a new `PathbuilderImporterService` in the `Processing` project.
-   **Logic:** This service will orchestrate the import process, taking a list of URLs, calling the `PathbuilderService` to fetch the data, and using `IMapper` to convert the characters.

### Phase 3: GUI Integration
-   **Task:** Implement the "Pathbuilder Import" tab as envisioned in the `M6` GUI v2.0 plan.
-   **Logic:** The new UI will allow users to enter URLs, manage a list of characters, and start the import process by calling the `PathbuilderImporterService`.

This plan provides a clear roadmap for a powerful new feature that will significantly expand the converter's audience and utility.
