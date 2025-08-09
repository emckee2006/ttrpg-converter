# M8: Performance Optimization & Parallel Processing - Junior Developer Implementation Guide

## 🎯 **MILESTONE 8 OVERVIEW**
**Duration**: 3 weeks | **Total Points**: 22 | **Priority**: 🔥 HIGH

Achieve 10-50x performance improvements with enterprise-grade parallel processing - critical missing functionality from our original plan.

### 🧪 **TESTING REQUIREMENTS**
**Every M8 task must include comprehensive testing before being marked complete:**
- ✅ **Performance Tests** - Benchmarks with large campaign datasets (>10MB)
- ✅ **Concurrency Tests** - Multi-threaded safety and correctness validation
- ✅ **Memory Tests** - Memory usage profiling and leak detection
- ✅ **Scalability Tests** - Linear scaling validation up to 16 cores
- ✅ **Stress Tests** - Resource limits and error handling under load

---

## **T8.1: Multi-Threaded Architecture Foundation**
**Duration**: 8 days | **Points**: 12 | **Priority**: 🚨 CRITICAL
**Dependencies**: M2 Complete (Core conversion engine)

### **Implementation Steps for Junior Developer**

**Step 1: Professional Concurrency Dependencies**
Update `ttrpg-core/Cargo.toml`:
```toml
[dependencies]
# Professional concurrency libraries (eliminate reinvented wheels)
tokio = { workspace = true, features = ["full"] }
rayon = { workspace = true }           # Data parallelism
crossbeam = "0.8"                      # Lock-free data structures
dashmap = "5.5"                        # Concurrent HashMap
parking_lot = "0.12"                   # High-performance synchronization

# Memory management and optimization
bumpalo = "3.14"                       # Bump allocator for temporary data
typed-arena = "2.0"                    # Typed memory arenas
slotmap = "1.0"                        # Efficient entity storage

# Performance monitoring
metrics = "0.21"                       # Application metrics
prometheus = "0.13"                    # Metrics export
tokio-metrics = "0.3"                  # Tokio runtime metrics
```

**Step 2: Thread Pool Management**
Create `ttrpg-core/src/performance/thread_pool.rs`:
```rust
use std::sync::Arc;
use tokio::sync::{mpsc, Semaphore};
use rayon::{ThreadPool, ThreadPoolBuilder};
use crossbeam::channel::{Receiver, Sender};
use parking_lot::RwLock;

/// Professional thread pool management with work stealing
pub struct TTRPGThreadPool {
    /// Async runtime pool for I/O operations
    async_runtime: tokio::runtime::Runtime,
    /// CPU-bound computation pool
    compute_pool: ThreadPool,
    /// Work queue for batch processing
    work_queue: Arc<crossbeam::queue::SegQueue<WorkItem>>,
    /// Resource limits and monitoring
    resource_monitor: Arc<ResourceMonitor>,
    /// Configuration
    config: ThreadPoolConfig,
}

#[derive(Debug, Clone)]
pub struct ThreadPoolConfig {
    /// Number of async worker threads
    pub async_workers: usize,
    /// Number of compute worker threads  
    pub compute_workers: usize,
    /// Maximum concurrent operations
    pub max_concurrent_ops: usize,
    /// Memory limit per operation (bytes)
    pub memory_limit_per_op: usize,
    /// Queue capacity before backpressure
    pub queue_capacity: usize,
}

impl TTRPGThreadPool {
    pub fn new(config: ThreadPoolConfig) -> Result<Self, PerformanceError> {
        // Create async runtime with configured workers
        let async_runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(config.async_workers)
            .thread_name("ttrpg-async")
            .enable_all()
            .build()?;
            
        // Create compute thread pool
        let compute_pool = ThreadPoolBuilder::new()
            .num_threads(config.compute_workers)
            .thread_name(|i| format!("ttrpg-compute-{}", i))
            .build()?;
            
        let work_queue = Arc::new(crossbeam::queue::SegQueue::new());
        let resource_monitor = Arc::new(ResourceMonitor::new(&config));
        
        Ok(Self {
            async_runtime,
            compute_pool,
            work_queue,
            resource_monitor,
            config,
        })
    }
    
    /// Process campaign with parallel entity processing
    pub async fn process_campaign_parallel(&self, campaign: Campaign) -> Result<Campaign, PerformanceError> {
        let start_time = std::time::Instant::now();
        
        // Phase 1: Analyze dependencies between entities
        let dependency_graph = self.analyze_entity_dependencies(&campaign).await?;
        
        // Phase 2: Create processing batches based on dependencies
        let processing_batches = self.create_processing_batches(&dependency_graph)?;
        
        // Phase 3: Process batches in parallel with progress tracking
        let processed_entities = self.process_batches_parallel(processing_batches).await?;
        
        // Phase 4: Reassemble campaign with processed entities
        let processed_campaign = self.reassemble_campaign(campaign, processed_entities).await?;
        
        // Record performance metrics
        let duration = start_time.elapsed();
        self.record_performance_metrics("campaign_processing", duration, &processed_campaign);
        
        Ok(processed_campaign)
    }
    
    /// Parallel entity processing with resource management
    async fn process_batches_parallel(&self, batches: Vec<EntityBatch>) -> Result<Vec<ProcessedEntity>, PerformanceError> {
        let semaphore = Arc::new(Semaphore::new(self.config.max_concurrent_ops));
        let results = Arc<RwLock<Vec<ProcessedEntity>>>::new(RwLock::new(Vec::new()));
        
        let batch_futures: Vec<_> = batches.into_iter().map(|batch| {
            let semaphore = Arc::clone(&semaphore);
            let results = Arc::clone(&results);
            let resource_monitor = Arc::clone(&self.resource_monitor);
            
            async move {
                // Acquire resource permit
                let _permit = semaphore.acquire().await?;
                
                // Check resource limits before processing
                resource_monitor.check_resource_limits()?;
                
                // Process batch on compute thread pool
                let processed_batch = self.process_entity_batch(batch).await?;
                
                // Store results thread-safely
                {
                    let mut results = results.write();
                    results.extend(processed_batch);
                }
                
                Ok::<(), PerformanceError>(())
            }
        }).collect();
        
        // Wait for all batches to complete
        futures::future::try_join_all(batch_futures).await?;
        
        // Return collected results
        let results = Arc::try_unwrap(results).unwrap().into_inner();
        Ok(results)
    }
    
    /// Memory-efficient entity processing using arenas
    async fn process_entity_batch(&self, batch: EntityBatch) -> Result<Vec<ProcessedEntity>, PerformanceError> {
        // Use typed arena for temporary allocations
        let arena = typed_arena::Arena::new();
        
        // Process entities in compute pool
        let processed = self.compute_pool.install(|| {
            batch.entities
                .par_iter()  // Rayon parallel iterator
                .map(|entity| {
                    // Allocate temporary data in arena
                    let temp_data = arena.alloc(EntityTempData::new(entity));
                    
                    // Process entity (CPU-bound work)
                    self.process_single_entity(entity, temp_data)
                })
                .collect::<Result<Vec<_>, _>>()
        })?;
        
        // Arena automatically deallocates when dropped
        Ok(processed)
    }
}

/// Resource monitoring and limits enforcement
pub struct ResourceMonitor {
    max_memory_usage: usize,
    max_cpu_usage: f64,
    metrics_collector: metrics::Counter,
}

impl ResourceMonitor {
    pub fn new(config: &ThreadPoolConfig) -> Self {
        Self {
            max_memory_usage: config.memory_limit_per_op * config.max_concurrent_ops,
            max_cpu_usage: 0.90,  // 90% CPU utilization limit
            metrics_collector: metrics::counter!("resource_checks_total"),
        }
    }
    
    /// Check system resource limits before processing
    pub fn check_resource_limits(&self) -> Result<(), PerformanceError> {
        self.metrics_collector.increment(1);
        
        // Check memory usage
        let current_memory = self.get_current_memory_usage()?;
        if current_memory > self.max_memory_usage {
            return Err(PerformanceError::MemoryLimitExceeded {
                current: current_memory,
                limit: self.max_memory_usage,
            });
        }
        
        // Check CPU usage
        let cpu_usage = self.get_current_cpu_usage()?;
        if cpu_usage > self.max_cpu_usage {
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    
    #[tokio::test]
    async fn test_parallel_campaign_processing() {
        let config = ThreadPoolConfig {
            async_workers: 4,
            compute_workers: 8,
            max_concurrent_ops: 16,
            memory_limit_per_op: 50 * 1024 * 1024, // 50MB per operation
            queue_capacity: 1000,
        };
        
        let thread_pool = TTRPGThreadPool::new(config).unwrap();
        
        // Create large test campaign
        let campaign = create_large_test_campaign(1000); // 1000 entities
        
        let start_time = std::time::Instant::now();
        let result = thread_pool.process_campaign_parallel(campaign).await.unwrap();
        let duration = start_time.elapsed();
        
        // Verify processing completed
        assert_eq!(result.entities.len(), 1000);
        
        // Performance target: should process 1000 entities in <5 seconds
        assert!(duration < Duration::from_secs(5));
        
        println!("Processed 1000 entities in {:?}", duration);
    }
    
    #[tokio::test]
    async fn test_resource_limits() {
        let config = ThreadPoolConfig {
            async_workers: 2,
            compute_workers: 2, 
            max_concurrent_ops: 1,  // Very restrictive
            memory_limit_per_op: 10 * 1024 * 1024, // 10MB limit
            queue_capacity: 10,
        };
        
        let thread_pool = TTRPGThreadPool::new(config).unwrap();
        let monitor = thread_pool.resource_monitor;
        
        // Should enforce resource limits
        let result = monitor.check_resource_limits().await;
        assert!(result.is_ok());
    }
}
```

**Step 3: Producer-Consumer Processing Patterns**
Create `ttrpg-core/src/performance/streaming.rs`:
```rust
/// High-throughput streaming processor for large campaigns
pub struct StreamingProcessor<T> {
    producer: Arc<Producer<T>>,
    consumers: Vec<Arc<Consumer<T>>>,
    buffer_size: usize,
    backpressure_threshold: f64,
}

impl<T> StreamingProcessor<T> 
where
    T: Send + Sync + 'static,
{
    /// Process data stream with automatic backpressure handling
    pub async fn process_stream<F, R>(&self, input: impl Stream<Item = T>, processor: F) -> Result<impl Stream<Item = R>, PerformanceError>
    where
        F: Fn(T) -> R + Send + Sync + 'static,
        R: Send + 'static,
    {
        let (sender, receiver) = mpsc::channel(self.buffer_size);
        
        // Spawn producer task
        let producer_handle = tokio::spawn(async move {
            let mut input = Box::pin(input);
            while let Some(item) = input.next().await {
                // Backpressure: wait if buffer is full
                if sender.send(item).await.is_err() {
                    break; // Consumer dropped
                }
            }
        });
        
        // Create processing stream with backpressure
        let output_stream = ReceiverStream::new(receiver)
            .map(move |item| processor(item))
            .buffer_unordered(self.buffer_size);
            
        Ok(output_stream)
    }
}
```

---

## **T8.2: Memory Optimization & Large Dataset Handling**
**Duration**: 6 days | **Points**: 8 | **Priority**: 🔥 HIGH  
**Dependencies**: T8.1 Complete

**Step 1: Memory-Mapped File Processing**
```rust
use memmap2::MmapOptions;
use std::fs::File;

/// Memory-mapped file processing for campaigns larger than RAM
pub struct MemoryMappedProcessor {
    mmap: memmap2::Mmap,
    chunk_size: usize,
    processing_window: usize,
}

impl MemoryMappedProcessor {
    /// Process enormous campaign files with minimal memory usage
    pub async fn process_large_campaign(&self, file_path: &Path) -> Result<ProcessingResult, PerformanceError> {
        let file = File::open(file_path)?;
        let mmap = unsafe { MmapOptions::new().map(&file)? };
        
        // Process in chunks to stay within memory limits
        let mut results = Vec::new();
        let chunk_size = 16 * 1024 * 1024; // 16MB chunks
        
        for chunk_start in (0..mmap.len()).step_by(chunk_size) {
            let chunk_end = std::cmp::min(chunk_start + chunk_size, mmap.len());
            let chunk_data = &mmap[chunk_start..chunk_end];
            
            // Process chunk and immediately release memory
            let chunk_result = self.process_chunk(chunk_data).await?;
            results.push(chunk_result);
            
            // Force garbage collection to free memory
            drop(chunk_data);
        }
        
        Ok(ProcessingResult::from_chunks(results))
    }
}
```

**Step 2: Performance Benchmarking Suite**
```rust
#[cfg(test)]
mod benchmarks {
    use super::*;
    use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
    
    fn benchmark_parallel_processing(c: &mut Criterion) {
        let mut group = c.benchmark_group("parallel_processing");
        group.throughput(Throughput::Elements(1000));
        
        // Benchmark single-threaded vs multi-threaded processing
        group.bench_function("single_threaded", |b| {
            b.iter(|| {
                let campaign = create_test_campaign(1000);
                process_campaign_single_threaded(black_box(campaign))
            })
        });
        
        group.bench_function("multi_threaded", |b| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            b.to_async(&rt).iter(|| async {
                let campaign = create_test_campaign(1000);
                let thread_pool = TTRPGThreadPool::new(default_config()).unwrap();
                thread_pool.process_campaign_parallel(black_box(campaign)).await.unwrap()
            })
        });
        
        group.finish();
    }
    
    fn benchmark_memory_efficiency(c: &mut Criterion) {
        let mut group = c.benchmark_group("memory_efficiency");
        
        // Test memory usage with different entity counts
        for entity_count in [100, 1000, 10000] {
            group.bench_with_input(
                format!("entities_{}", entity_count),
                &entity_count,
                |b, &count| {
                    b.iter(|| {
                        let campaign = create_test_campaign(count);
                        measure_memory_usage(|| {
                            process_campaign_memory_optimized(black_box(campaign))
                        })
                    })
                }
            );
        }
    }
}
```

**Acceptance Criteria for M8:**
- [ ] ✅ 10-50x performance improvement over single-threaded baseline
- [ ] ✅ CPU utilization >80% on multi-core systems (up to 16 cores)
- [ ] ✅ Linear scaling with available cores
- [ ] ✅ Memory usage under 2GB for campaigns with 10,000+ entities
- [ ] ✅ Zero deadlocks or race conditions in stress tests
- [ ] ✅ Automatic backpressure handling for resource limits
- [ ] ✅ Real-time progress tracking with cancellation support
