# Compendium Caching Strategy

**Author:** Gemini & Alithanna
**Date:** 2024-07-26
**Status:** Finalized & Aligned with Codebase

## 1. Overview

To address a significant performance bottleneck and to create a robust foundation for parallel processing, a new caching architecture has been implemented. The previous approach re-processed all source compendia on every run. The new architecture uses a persistent, pre-built cache to provide high-speed access to compendium data during conversions.

## 2. Architectural Placement

As per the final solution architecture (`M2.3_PROJECT_ARCHITECTURE.md`), all components of the compendium system are placed in the `TTRPGConverter.Infrastructure` project to ensure a clean separation of concerns:

-   **Interfaces (`ICompendiumManager`, `ICompendiumReader`):** The core interfaces are defined in `TTRPGConverter.Infrastructure/Services/Compendium/`.
-   **Implementations:** The concrete implementations reside in the same directory:
    -   **`CompendiumCacheBuilder`**: The offline tool for building the cache.
    -   **`RavenDbCompendiumManager`**: The runtime service for querying the cache.
    -   **Reader Implementations**: All `ICompendiumReader` implementations (e.g., `NeDbCompendiumReader`, `FoundryCliBridge`) are located in the `CompendiumReaders` subdirectory.

## 3. The Caching Technology (RavenDB Embedded)

The new architecture is centered around **RavenDB Embedded**, a full-featured NoSQL document database that runs in-process with the application. This provides several key advantages:

-   **ACID Transactions:** Guarantees that the database state is always consistent.
-   **Powerful Indexing:** Allows for fast, indexed queries on any field (e.g., by `Type` and `Name`) without loading the entire dataset into memory.
-   **Native .NET Client:** Offers a clean, modern API with full LINQ support.

## 4. The Two-Phase Process

The new architecture is built on a clear separation of two distinct processes: **Building the Cache** and **Using the Cache**.

### Phase 1: Building the Cache (`update-compendium` command)

This is an offline, one-time process managed by the `CompendiumCacheBuilder` service.

**Workflow:**

1.  **Discovery:** The `CompendiumCacheBuilder` scans the user's Foundry VTT data directory to discover all available compendium packs from systems and modules.
2.  **Processing:** It uses a strategy pattern, invoking the correct `ICompendiumReader` implementation (`NeDbCompendiumReader`, `FoundryCliBridge`, etc.) for each discovered pack format.
3.  **Conflict Resolution:** All loaded items are processed to resolve name collisions and select the "best" version of each entity based on a predefined source priority.
4.  **Writing to Cache:** The final, de-duplicated list of compendium items is saved into a local RavenDB database file (`compendium.ravendb`). This process completely overwrites any existing cache.

This command should be run by the user whenever they update their Foundry VTT content to refresh the cache.

### Phase 2: Using the Cache (Runtime Conversion)

During a normal conversion process (e.g., the `convert-campaign` command), the application uses the `ICompendiumManager` interface to look up entities.

**Workflow:**

1.  **Dependency Injection:** The `ICompendiumManager` interface is injected into any service that needs compendium data.
2.  **Implementation:** The DI container provides the `RavenDbCompendiumManager` as the concrete implementation.
3.  **Querying:** The `RavenDbCompendiumManager` connects to the pre-built `compendium.ravendb` file and performs fast, indexed, read-only queries to find the required items.

This architecture ensures that the main conversion process is extremely fast, as it no longer needs to re-process the source compendia. It simply performs quick lookups against a clean, optimized, and pre-built local database.
