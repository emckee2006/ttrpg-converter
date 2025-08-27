# M1.6: Asset Management Architecture

**Author:** Gemini & Alithanna
**Date:** 2024-07-26
**Status:** Ready for Development

## 1. Current Architecture

This document outlines the architecture for the TTRPG Converter's asset management system. The system is responsible for handling all external assets (images, tokens, maps) referenced in a campaign.

The current implementation uses a database-backed cache to prevent redundant downloads. The key components are:

-   **Database:** EF Core with a SQLite provider.
-   **`AssetMapping` Entity:** A simple lookup table in the database that maps a remote `SourceUrl` to a `LocalPath` once downloaded.
-   **`IAssetMapper` Service:** The public interface for requesting a local asset path for a given source URL.
-   **`ParallelAssetProcessor`:** The implementation that uses a producer-consumer pattern (`System.Threading.Channels`) to download assets in parallel for high performance.

## 2. Identified Flaws & Required Enhancements

While functional for the "happy path," the current implementation has three critical flaws that must be addressed to make the system robust and reliable.

1.  **Filename Collisions:** The current system uses the original filename from the URL (`Path.GetFileName`). If two different assets from two different URLs share the same filename (e.g., `token.png`), the second download will overwrite the first, leading to data corruption.
2.  **Lack of Resilience:** The system has no mechanism to handle transient network errors. If an `HttpClient` request fails for any reason, the asset is marked as failed with no attempt to retry.
3.  **Poor Network Citizenship:** The high-speed parallel downloader makes many requests in a very short period. This can overwhelm a single host server, leading to dropped requests, temporary IP bans, and a negative impact on the services we rely on.

## 3. The Implementation Plan

To address these flaws, we will implement the following changes.

### 3.1. The Asset Processing Workflow

The `ParallelAssetProcessor` will be updated to follow a more robust, multi-step workflow for each asset request:

1.  **Check Local ZIP First (New Feature):** Before any network activity, the worker will first check if a local campaign ZIP file was provided. If it was, the worker will attempt to find the asset within the archive. If the asset is found, it is extracted and used as the source, and the network download is skipped entirely. This is a major performance optimization.

2.  **Network Fetch (Fallback):** If the asset is not found in the ZIP, the worker will proceed to download it from the `sourceUrl`.

3.  **Generate Unique Filename:** To guarantee that every unique asset URL results in a unique file on disk, the service will generate a new filename by creating a **SHA256 hash of the source URL** and appending the original file extension. 
    -   *Example: `https://example.com/path/to/image.png` -> `a1b2c3d4e5f6a7b8c9d0...png`*

4.  **Save to Disk:** The downloaded or extracted asset data is saved to the new, unique filename in the appropriate output directory.

### 3.2. Implement Resilient & Responsible Downloading

To solve both the resilience and network citizenship issues, we will add the **Polly** NuGet package to the `TTRPGConverter.Infrastructure` project and create a sophisticated, combined policy.

-   **Action:** Modify the `ParallelAssetProcessor` service.
-   **Implementation:** We will create a single, combined Polly policy that will wrap every `HttpClient` request. This policy will be configured to:
    1.  **Rate Limit:** Use `Polly.RateLimit` to ensure we only make a specific number of requests per second to any single host (e.g., 10 requests/sec). This prevents us from overwhelming servers.
    2.  **Retry Policy:** Use `Polly.Retry` to automatically re-attempt any failed HTTP requests (e.g., 3 retries with a short, exponential backoff delay). This handles transient network glitches.
    3.  **Circuit Breaker:** Use `Polly.CircuitBreaker` as a safety valve. If requests to a specific host fail repeatedly, the circuit will "trip," and the application will stop trying to contact that host for a short period (e.g., 30 seconds), giving it time to recover and preventing us from wasting resources.

This unified plan will result in a robust, reliable, and responsible asset management system, providing a solid foundation for the entire conversion process.
