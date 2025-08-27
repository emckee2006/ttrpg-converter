# M3.1: Stage 1 - Platform Conversion Tasks

**Author:** Gemini & Alithanna
**Date:** 2024-07-26
**Status:** Ready for Development
**Depends On:** `M2_ROLL20_5E_TO_FOUNDRY_PF2E.md`, `M2.1_AUTOMAPPER_INTEGRATION_STRATEGY.md`

## 1. Overview

This document outlines the remaining implementation tasks required to complete **Stage 1 (Platform Conversion)** of the M2 Milestone. The goal of this stage is to perform a high-fidelity, 1:1 conversion of a Roll20 D&D 5e campaign into the Foundry VTT format.

The work is divided into two main categories: completing the detailed mapping for complex entities and building the high-level services to orchestrate the conversion process.

## 2. Detailed Mapping Tasks (Completing the Profiles)

These tasks involve enhancing our existing AutoMapper profiles with more granular logic, based on a careful review of the `scenes.py` reference and our mapping guides.

### 2.1. Scene Mapping (`PageToSceneProfile`)

-   **Task:** Map Roll20 `paths` to Foundry `walls` and `drawings`.
-   **Implementation:**
    -   Create a new `PagePathsResolver`.
    -   This resolver will iterate through the `paths` on a Roll20 `Page`.
    -   It will use heuristics (e.g., checking the `stroke` color, path complexity) to differentiate between simple lines (walls) and freehand shapes (drawings).
    -   It will map wall lines to `Wall` documents, correctly identifying **doors** based on their stroke color.
    -   It will map freehand shapes and polygons to `Drawing` documents.

-   **Task:** Map Roll20 `text` objects to Foundry `drawings`.
-   **Implementation:**
    -   Create a new `PageTextResolver`.
    -   This resolver will convert Roll20 text objects into Foundry `Drawing` documents with the `type` set to `text`.
    -   It must preserve the text content, position, font, size, and color.

### 2.2. Item Mapping (`Roll20ItemProfile`)

-   **Task:** Infer more specific item types.
-   **Implementation:**
    -   Enhance the `AfterMap` logic in the `Roll20ItemProfile`.
    -   Add heuristics to identify `consumable`, `tool`, and `feat` types from the Roll20 item data (e.g., by checking for keywords in the item name or description) and set the `Dnd5eitemtype` enum accordingly.

### 2.3. Card Mapping (`Roll20CardToFoundryCardProfile`)

-   **Task:** Map multiple card faces.
-   **Implementation:**
    -   Create a `CardFacesResolver`.
    -   This resolver will handle the logic for cards that have different front and back images, creating multiple `Face` objects in the Foundry `Card`'s `faces` array.

## 3. Integration and Orchestration Tasks

This is the final phase of the core logic development, where we tie all the individual mappings together.

### 3.1. `ConversionService` (in `TTRPGConverter.Processing`)

-   **Task:** Create the main `ConversionService.cs`.
-   **Implementation:**
    -   This service will inject the `IMapper`, `Roll20CampaignService`, and `FoundryWorldBuilder`.
    -   It will contain the main `ConvertCampaignAsync` method.
    -   This method will load the source campaign, iterate through each entity type (characters, handouts, pages, etc.), and use `_mapper.Map<TDestination>(source)` to convert them into lists of Foundry documents.

### 3.2. `FoundryWorldBuilder` (in `TTRPGConverter.Infrastructure`)

-   **Task:** Implement the `FoundryWorldBuilder.cs`.
-   **Implementation:**
    -   This service will receive the lists of converted Foundry documents from the `ConversionService`.
    -   It will be responsible for creating the world directory structure (`data/`, `assets/`, etc.).
    -   It will serialize each document list into the appropriate NeDB database file (e.g., `actors.db`, `items.db`, `scenes.db`).

### 3.3. Actor-Item Linking

-   **Task:** Connect converted Items to their owner Actor.
-   **Implementation:**
    -   This logic belongs in the `ConversionService` or `FoundryWorldBuilder`.
    -   After all `Dnd5eItem` documents have been created and mapped, the service will iterate through the `Dnd5eActor` documents.
    -   For each actor, it will find all the items that belong to it and add their `_id` to the actor's `items` array before saving.

This comprehensive plan provides a clear roadmap to completing the converter. The next logical step is to begin the detailed mapping tasks, starting with the `Page` to `Scene` walls and drawings.
