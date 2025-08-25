# Compendium Caching Strategy

**Author:** Gemini
**Date:** 2024-07-25
**Status:** Design Finalized

## 1. Overview

To address a significant performance bottleneck and to create a robust foundation for parallel processing, a new caching and state management strategy has been implemented. The previous approach re-processed all source compendia on every run and lacked a mechanism to safely coordinate parallel tasks.

The new architecture uses a single, powerful embedded database to manage both the persistent compendium cache and the transient state of a conversion run, ensuring high performance, data integrity, and safe concurrency.

## 2. The Caching and State Management Architecture

The new architecture is centered around **RavenDB Embedded**, a full-featured NoSQL document database that runs in-process with the application.

### 2.1. The Database (RavenDB Embedded)

The choice of RavenDB Embedded was made specifically to handle the demands of a highly concurrent conversion process:

-   **ACID Transactions:** Guarantees that the database state is always consistent, even if the application crashes mid-operation. This is critical for both cache generation and managing the state of a conversion.
-   **Session-Based Unit of Work:** Provides excellent concurrency management. Each parallel task operates in its own session, allowing for safe, isolated reads and writes without manual locking. Optimistic concurrency prevents race conditions when multiple threads try to create the same custom entity or asset.
-   **Powerful Indexing:** Allows for fast, indexed queries on any field, including nested document properties. This is used for high-speed lookups of compendium items and assets without loading the entire dataset into memory.
-   **Native .NET Client:** Offers a clean, modern API with full LINQ support, making database interactions intuitive and maintainable.

### 2.2. The `update-compendium` Command

A new CLI command is introduced to manage the persistent cache.

**Workflow:**

1.  **Discovery & Processing:** It uses the `CompendiumManager` to load all source compendia and resolve all entity conflicts.
2.  **Writing to Cache:**
    -   It connects to the `compendium.db` RavenDB file.
    -   It completely clears any old data in the `CompendiumItems` collection.
    -   It stores all the unified compendium items in the collection.
    -   It ensures that appropriate indexes (e.g., on `Type` and `Name`) are created.

This command should be run by the user whenever they update their FoundryVTT content to refresh the cache.

## 3. The Conversion Process Workflow

1.  **Initialization:** The `ConversionStateManager` connects to the `compendium.db` file.
2.  **State Management:** For each conversion run, it creates **temporary collections** (e.g., `CustomItems_Run_XYZ`, `AssetMappings_Run_XYZ`). This isolates the state of each run.
3.  **Parallel Processing:**
    -   Dozens or hundreds of `IAssetProcessor` and entity processing tasks can run in parallel.
    -   Each task uses its own RavenDB session to interact with the database.
    -   **Compendium Lookups:** Tasks perform fast, indexed, concurrent reads against the permanent `CompendiumItems` collection.
    -   **Custom Item/Asset Creation:** When a new item must be created, the task attempts to write it to the temporary collection. RavenDB's concurrency control ensures that only one thread can create a given item, preventing duplicates and race conditions.
4.  **Finalization:** Once all parallel tasks are complete, the finalization process reads the temporary collections to generate the output JSON files.
5.  **Cleanup:** After the output is successfully written, the temporary collections for the completed run are deleted from the database, ensuring the cache file does not grow with transient data.

This architecture provides a scalable, reliable, and high-performance foundation for the entire conversion process.