# M1.7: Advanced Asset Pipeline Architecture

**Author:** Gemini & Alithanna
**Date:** 2024-07-26
**Status:** Adopted
**Builds Upon:** `M1.6_ASSET_MANAGEMENT_ARCHITECTURE.md`

## 1. Overview

This document outlines the architecture for a high-performance, parallelized, and configurable asset processing pipeline. This system extends the foundational asset management features defined in M1.6 to provide on-the-fly conversion and optimization of assets. The primary goals are to significantly reduce the final world size and improve in-game rendering performance by converting assets to modern, efficient formats and combining scene tiles into a single background image.

This plan is based on the proven implementation of professional-grade tools like the `media-optimizer` Foundry VTT module.

## 2. Core Features

-   **Asset Conversion:** Convert all standard image, audio, and video formats to WEBP, OGG, and WEBM respectively.
-   **Image Optimization:** Downscale large images to a configurable maximum resolution and apply quality-based compression.
-   **Tile Combination:** For each scene, combine the background image and all static tiles into a single, flattened background image to dramatically improve rendering performance.
-   **Configurability:** Allow the user to enable, disable, and fine-tune all aspects of the pipeline via the GUI.

## 3. The Multi-Stage Asset Processing Pipeline

The architecture will extend the existing `ParallelAssetProcessor` to support a multi-stage pipeline. After an asset is downloaded, it will be passed through a series of optional processing stages before being saved to its final destination.

### 3.1. The `AssetMapping` Document

The existing `AssetMapping` entity in our EF Core/SQLite database is sufficient for this pipeline. The `LocalPath` will now point to the final, *processed* asset (e.g., the `.webp` file), not the original downloaded file.

### 3.2. The `AssetProcessingOptions` Model

A new `AssetProcessingOptions` class will be created in `TTRPGConverter.Core` to provide fine-grained control over the pipeline. This will be populated from the GUI.

```csharp
public class AssetProcessingOptions
{
    public bool OptimizeImages { get; set; } = true;
    public bool ConvertImagesToWebp { get; set; } = true;
    public int WebpQuality { get; set; } = 75;
    public int MaxImageWidth { get; set; } = 8192;
    public int MaxImageHeight { get; set; } = 8192;

    public bool OptimizeAudio { get; set; } = true;
    public bool ConvertAudioToOgg { get; set; } = true;
    public int OggQuality { get; set; } = 5; // VBR quality scale

    public bool OptimizeVideo { get; set; } = true;
    public bool ConvertVideoToWebm { get; set; } = true;

    public bool CombineTiles { get; set; } = true;
}
```

## 4. Phased Implementation Plan

### Phase 1: Configuration & UI

1.  **Create `AssetProcessingOptions`:** Implement the class as defined above in the `Core` project.
2.  **Update GUI:**
    -   Add a new "Settings" or "Options" section to the `MainWindow.axaml` view.
    -   Add checkboxes and sliders bound to a new `AssetProcessingOptions` property in the `MainWindowViewModel`.
    -   This allows the user to configure the pipeline before starting a conversion.

### Phase 2: Image Processing Pipeline

1.  **Add Dependency:** Add the `SixLabors.ImageSharp` NuGet package to the `TTRPGConverter.Infrastructure` project.
2.  **Create `ImageProcessor` Service:** Create a new `ImageProcessor` class that takes `AssetProcessingOptions` in its constructor.
3.  **Implement Processing Logic:** The `ImageProcessor` will have a method `ProcessImageAsync(byte[] imageData)` that:
    a.  Decodes the image data into an `ImageSharp` `Image` object.
    b.  Checks the image dimensions against `MaxImageWidth` and `MaxImageHeight`. If larger, it resizes the image using a high-quality resampling algorithm (e.g., `Lanczos3`).
    c.  If `ConvertImagesToWebp` is true, it encodes the final image as WEBP using the configured `WebpQuality`. Otherwise, it encodes it in its original format.
    d.  Returns the final image data as a `byte[]`.
4.  **Integrate into `ParallelAssetProcessor`:**
    -   Inject the new `ImageProcessor` into the `ParallelAssetProcessor`.
    -   After an image asset is successfully downloaded, the consumer thread will pass the image data to the `ImageProcessor` before saving the result to disk.

### Phase 3: Audio/Video Processing Pipeline

1.  **Add FFmpeg Dependency:** This feature requires an external dependency on **FFmpeg**. The application must be designed to handle its absence gracefully.
2.  **Create `FFmpegService`:** Create a service to detect if `ffmpeg.exe` is available on the user's system (e.g., by checking common paths or the system's PATH environment variable).
3.  **Create `AudioVideoProcessor` Service:** This service will take the `AssetProcessingOptions` and `FFmpegService` in its constructor.
4.  **Implement Processing Logic:** It will have a method `ProcessAsync(string tempInputPath)` that:
    a.  Checks if FFmpeg is available. If not, it returns the original file path.
    b.  Executes an `ffmpeg.exe` command-line process with the appropriate arguments to convert the file to OGG or WEBM based on the options.
    c.  Returns the path to the new, converted file.
5.  **Integrate into `ParallelAssetProcessor`:**
    -   Inject the new `AudioVideoProcessor`.
    -   After an audio or video asset is downloaded, the consumer will save it to a temporary file and pass the path to the `AudioVideoProcessor`.

### Phase 4: Scene Tile Combination

This is the most complex feature and should be implemented as a distinct, final step in the world conversion process.

1.  **Create `ISceneProcessor` Interface:** Define a new interface in the `Core` project for post-processing scenes.
2.  **Create `TileCombinerService`:** This service will implement `ISceneProcessor` and use `ImageSharp`.
3.  **Implement Workflow:** The service's `ProcessSceneAsync(FoundryScene scene)` method will:
    a.  Use the `IAssetMapper` to get the final, local file paths for the scene's background image and all of its tile images.
    b.  Load the background image as the base canvas.
    c.  Iterate through the scene's `tiles` array, sorted by `sort` and `elevation` to ensure correct Z-ordering.
    d.  For each tile, load its image and apply all transformations (`rotation`, `alpha`, `tint`, `x`, `y`, `width`, `height`).
    e.  Draw the transformed tile onto the main canvas.
4.  **Save New Asset & Update Scene:**
    a.  The final, combined canvas is saved as a new, single image file (e.g., `scene-123-combined.webp`).
    b.  The `FoundryScene` document is modified: the `background.src` property is updated to point to the new combined image, and the `tiles` array is cleared.
5.  **Integrate into `FoundryWorldBuilder`:**
    -   Inject the `TileCombinerService`.
    -   After all standard scene conversion is complete, iterate through the generated scenes and pass each one to the `TileCombinerService` if the `CombineTiles` option is enabled.

This comprehensive plan provides a clear path to implementing a professional-grade asset optimization and processing pipeline, resulting in smaller, faster, and more modern Foundry VTT worlds.
