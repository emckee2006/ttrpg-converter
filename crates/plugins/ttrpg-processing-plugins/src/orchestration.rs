//! Plugin Orchestration System
//!
//! This module provides dependency injection, pipeline management, and plugin
//! discovery using daggy, shaku, and inventory frameworks.

// TODO: Re-enable plugin imports once they're integrated properly
// use crate::{
//     shared::{AssetExecutionContext, ValidationExecutionContext},
//     validation::plugin::ValidationPlugin,
//     asset::{
//         retrieval::AssetRetrievalPlugin,
//         resolution::AssetResolutionPlugin,
//         conversion::AssetConversionPlugin,
//         scene::SceneProcessingPlugin,
//         reference::ReferenceTrackingPlugin,
//     },
// };
use ttrpg_core::{
    error::ConversionResult,
    plugin_framework::{interfaces::PluginInfo, discovery::PluginDiscovery, discovery::DiscoveryConfig},
};
use std::sync::Arc;
use std::collections::HashMap;
use daggy::{Dag, NodeIndex};
use petgraph::{visit::{IntoEdgesDirected, EdgeRef, IntoNodeIdentifiers}};
// TODO: Fix shaku import once correct API is determined
// use shaku::{Component, Interface, HasComponent, Container, ContainerBuilder};
use shaku::{Component, Interface};
use rayon::prelude::*;

/// Plugin orchestration system for managing processing pipelines
pub struct PluginOrchestrator {
    /// TODO: Add dependency injection container once shaku Container API is resolved
    // container: Container,
    /// Plugin execution DAG
    execution_dag: Dag<PluginNode, PluginEdge>,
    /// Node index mapping
    node_indices: HashMap<String, NodeIndex>,
    /// Plugin execution statistics
    stats: OrchestrationStats,
}

/// Plugin types for categorization
#[derive(Debug, Clone, PartialEq)]
pub enum PluginType {
    /// Input plugins for reading data
    Input,
    /// Processing plugins for transforming data
    Processing,
    /// Output plugins for writing data
    Output,
    /// Validation plugins for checking data
    Validation,
}

/// Plugin node in execution DAG
#[derive(Debug, Clone)]
pub struct PluginNode {
    /// Plugin identifier
    pub plugin_id: String,
    /// Plugin name
    pub plugin_name: String,
    /// Plugin type
    pub plugin_type: PluginType,
    /// Execution priority
    pub priority: i32,
    /// Execution status
    pub status: PluginStatus,
}

/// Plugin edge representing dependency
#[derive(Debug, Clone)]
pub struct PluginEdge {
    /// Dependency type
    pub dependency_type: DependencyType,
    /// Edge weight for ordering
    pub weight: f32,
}

/// Plugin execution status
#[derive(Debug, Clone, PartialEq)]
pub enum PluginStatus {
    /// Plugin is ready to execute
    Ready,
    /// Plugin is currently executing
    Running,
    /// Plugin completed successfully
    Completed,
    /// Plugin failed with error
    Failed(String),
    /// Plugin was skipped
    Skipped,
}

/// Dependency relationship types
#[derive(Debug, Clone)]
pub enum DependencyType {
    /// Hard dependency - must complete before dependent
    Required,
    /// Soft dependency - preferred order but not required
    Preferred,
    /// Data dependency - dependent needs output
    DataFlow,
}

/// Orchestration statistics
#[derive(Debug, Clone, Default)]
pub struct OrchestrationStats {
    /// Total plugins registered
    pub plugins_registered: usize,
    /// Plugins executed successfully
    pub plugins_executed: usize,
    /// Plugins failed
    pub plugins_failed: usize,
    /// Total execution time
    pub total_execution_time_ms: u64,
    /// Average plugin execution time
    pub average_execution_time_ms: f64,
}

/// Plugin processing pipeline configuration
#[derive(Debug, Clone)]
pub struct PipelineConfig {
    /// Enable parallel plugin execution where possible
    pub parallel_execution: bool,
    /// Maximum concurrent plugins
    pub max_concurrent_plugins: usize,
    /// Timeout for plugin execution in seconds
    pub plugin_timeout_seconds: u64,
    /// Enable plugin failure recovery
    pub enable_failure_recovery: bool,
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            parallel_execution: true,
            max_concurrent_plugins: 4,
            plugin_timeout_seconds: 300, // 5 minutes
            enable_failure_recovery: true,
        }
    }
}

// Trait definitions for dependency injection
trait AssetProcessor: Interface {
    fn process_assets(&self, assets: Vec<ttrpg_core::plugin_framework::types::AssetInfo>) -> ConversionResult<Vec<u8>>;
}

trait ValidationProcessor: Interface {
    fn validate_data(&self, data: &serde_json::Value) -> ConversionResult<bool>;
}

// Shaku component implementations
#[derive(Component)]
#[shaku(interface = AssetProcessor)]
struct AssetProcessorImpl {
    #[shaku(inject)]
    retrieval: Arc<dyn AssetRetriever>,
    #[shaku(inject)]
    resolution: Arc<dyn AssetResolver>,
    #[shaku(inject)]
    conversion: Arc<dyn AssetConverter>,
}

trait AssetRetriever: Interface {}
trait AssetResolver: Interface {}
trait AssetConverter: Interface {}

#[derive(Component)]
#[shaku(interface = AssetRetriever)]
struct AssetRetrieverImpl;

#[derive(Component)]
#[shaku(interface = AssetResolver)]
struct AssetResolverImpl;

#[derive(Component)]
#[shaku(interface = AssetConverter)]
struct AssetConverterImpl;

impl AssetRetriever for AssetRetrieverImpl {}
impl AssetResolver for AssetResolverImpl {}
impl AssetConverter for AssetConverterImpl {}

impl AssetProcessor for AssetProcessorImpl {
    fn process_assets(&self, _assets: Vec<ttrpg_core::plugin_framework::types::AssetInfo>) -> ConversionResult<Vec<u8>> {
        // Implementation would coordinate between retrieval, resolution, and conversion
        Ok(vec![])
    }
}

impl PluginOrchestrator {
    /// Create new plugin orchestrator
    pub fn new() -> ConversionResult<Self> {
        // TODO: Re-enable container once shaku Container API is resolved
        // let container = Self::build_container()?;
        let execution_dag = Dag::new();
        let node_indices = HashMap::new();
        let stats = OrchestrationStats::default();
        
        Ok(Self {
            // container,
            execution_dag,
            node_indices,
            stats,
        })
    }
    
    /// TODO: Build dependency injection container once shaku Container API is resolved
    // fn build_container() -> ConversionResult<Container> {
    fn _build_container_placeholder() -> ConversionResult<()> {
        // TODO: Implement container building once shaku API is resolved
        // let mut builder = ContainerBuilder::new();
        // 
        // // Register components
        // builder.register_type::<AssetRetrieverImpl>().as_type::<dyn AssetRetriever>();
        // builder.register_type::<AssetResolverImpl>().as_type::<dyn AssetResolver>();
        // builder.register_type::<AssetConverterImpl>().as_type::<dyn AssetConverter>();
        // builder.register_type::<AssetProcessorImpl>().as_type::<dyn AssetProcessor>();
        // 
        // Ok(builder.build())
        Ok(())
    }
    
    /// Discover and register plugins using inventory
    pub fn discover_plugins(&mut self) -> ConversionResult<()> {
        tracing::info!("Discovering plugins using inventory");
        
        // Use proper plugin discovery system from ttrpg-core
        let discovery = PluginDiscovery::new(DiscoveryConfig::default());
        discovery.discover_all()?;
        let plugin_infos = discovery.list_all_plugins()?;
        let plugins: Vec<PluginInfo> = plugin_infos.into_iter().map(|p| p.info).collect();
        
        tracing::info!("Found {} plugins", plugins.len());
        
        // Register each plugin in the DAG
        for plugin_info in &plugins {
            self.register_plugin(plugin_info)?;
        }
        
        // Build dependency edges
        self.build_dependency_graph()?;
        
        self.stats.plugins_registered = self.node_indices.len();
        
        Ok(())
    }
    
    /// Register individual plugin
    fn register_plugin(&mut self, plugin_info: &PluginInfo) -> ConversionResult<()> {
        let plugin_node = PluginNode {
            plugin_id: plugin_info.name.clone(),
            plugin_name: plugin_info.name.to_string(),
            plugin_type: PluginType::Processing, // TODO: Determine plugin type from supported_features
            priority: self.calculate_plugin_priority(plugin_info),
            status: PluginStatus::Ready,
        };
        
        let node_index = self.execution_dag.add_node(plugin_node);
        self.node_indices.insert(plugin_info.name.clone(), node_index);
        
        tracing::debug!("Registered plugin: {} ({})", plugin_info.name, plugin_info.version);
        
        Ok(())
    }
    
    /// Calculate plugin execution priority
    fn calculate_plugin_priority(&self, plugin_info: &PluginInfo) -> i32 {
        // TODO: Determine priority from plugin name or supported_features
        if plugin_info.name.contains("Validation") {
            75  // High priority for validation
        } else if plugin_info.name.contains("Input") {
            100 // Highest priority for input
        } else if plugin_info.name.contains("Output") {
            10  // Lower priority for output
        } else {
            50  // Medium priority for processing
        }
    }
    
    /// Build dependency graph based on plugin dependencies
    fn build_dependency_graph(&mut self) -> ConversionResult<()> {
        // Use proper plugin discovery system from ttrpg-core
        let discovery = PluginDiscovery::new(DiscoveryConfig::default());
        discovery.discover_all()?;
        let plugin_infos = discovery.list_all_plugins()?;
        let plugins: Vec<PluginInfo> = plugin_infos.into_iter().map(|p| p.info).collect();
        
        for plugin_info in &plugins {
            if let Some(&dependent_idx) = self.node_indices.get(&plugin_info.name) {
                // Add edges for each dependency
                for dependency_id in &plugin_info.dependencies {
                    if let Some(&dependency_idx) = self.node_indices.get(dependency_id) {
                        let edge = PluginEdge {
                            dependency_type: DependencyType::Required,
                            weight: 1.0,
                        };
                        
                        self.execution_dag.add_edge(dependency_idx, dependent_idx, edge)
                            .map_err(|e| ttrpg_core::error::ConversionError::PluginError {
                                message: format!("Failed to add dependency edge: {:?}", e),
                                plugin_name: None,
                                source: None,
                            })?;
                        
                        tracing::debug!("Added dependency: {} -> {}", dependency_id, plugin_info.name);
                    } else {
                        tracing::warn!("Dependency '{}' not found for plugin '{}'", dependency_id, plugin_info.name);
                    }
                }
            }
        }
        
        // Validate DAG (check for cycles)
        if self.has_cycles() {
            return Err(Box::new(ttrpg_core::error::ConversionError::PluginError {
                message: "Circular dependency detected in plugin graph".to_string(),
                plugin_name: None,
                source: None,
            }));
        }
        
        Ok(())
    }
    
    /// Check for cycles in the dependency graph
    fn has_cycles(&self) -> bool {
        // Simple cycle detection using DFS
        let mut visited = std::collections::HashSet::new();
        let mut rec_stack = std::collections::HashSet::new();
        
        for node_idx in self.execution_dag.node_identifiers() {
            if !visited.contains(&node_idx) {
                if self.dfs_has_cycle(node_idx, &mut visited, &mut rec_stack) {
                    return true;
                }
            }
        }
        
        false
    }
    
    /// DFS helper for cycle detection
    fn dfs_has_cycle(
        &self,
        node: NodeIndex,
        visited: &mut std::collections::HashSet<NodeIndex>,
        rec_stack: &mut std::collections::HashSet<NodeIndex>,
    ) -> bool {
        visited.insert(node);
        rec_stack.insert(node);
        
        // Check all neighbors
        // TODO: Fix daggy Direction import
            // for edge_ref in self.execution_dag.edges_directed(node, daggy::Direction::Outgoing) {
            for edge_ref in self.execution_dag.edges_directed(node, petgraph::Direction::Outgoing) {
            let neighbor = edge_ref.target();
            
            if !visited.contains(&neighbor) {
                if self.dfs_has_cycle(neighbor, visited, rec_stack) {
                    return true;
                }
            } else if rec_stack.contains(&neighbor) {
                return true;
            }
        }
        
        rec_stack.remove(&node);
        false
    }
    
    /// Execute plugin pipeline
    pub async fn execute_pipeline(&mut self, config: &PipelineConfig) -> ConversionResult<PipelineExecutionResult> {
        tracing::info!("Executing plugin pipeline with {} plugins", self.node_indices.len());
        
        let start_time = std::time::Instant::now();
        let execution_order = self.calculate_execution_order()?;
        let mut executed_plugins = Vec::new();
        let mut failed_plugins = Vec::new();
        
        // Execute plugins in topological order
        for plugin_batch in execution_order {
            if config.parallel_execution && plugin_batch.len() > 1 {
                // Execute batch in parallel
                let results = self.execute_plugin_batch_parallel(plugin_batch, config).await?;
                for result in results {
                    match result {
                        Ok(plugin_id) => executed_plugins.push(plugin_id),
                        Err(plugin_id) => failed_plugins.push(plugin_id),
                    }
                }
            } else {
                // Execute batch sequentially
                for plugin_id in plugin_batch {
                    match self.execute_single_plugin(&plugin_id, config).await {
                        Ok(_) => executed_plugins.push(plugin_id),
                        Err(_) => failed_plugins.push(plugin_id),
                    }
                }
            }
        }
        
        let execution_time = start_time.elapsed();
        
        // Update statistics
        self.stats.plugins_executed = executed_plugins.len();
        self.stats.plugins_failed = failed_plugins.len();
        self.stats.total_execution_time_ms = execution_time.as_millis() as u64;
        self.stats.average_execution_time_ms = if self.stats.plugins_executed > 0 {
            self.stats.total_execution_time_ms as f64 / self.stats.plugins_executed as f64
        } else {
            0.0
        };
        
        tracing::info!("Pipeline execution completed in {}ms: {} succeeded, {} failed", 
                      self.stats.total_execution_time_ms,
                      self.stats.plugins_executed,
                      self.stats.plugins_failed);
        
        Ok(PipelineExecutionResult {
            executed_plugins,
            failed_plugins,
            execution_time_ms: self.stats.total_execution_time_ms,
            stats: self.stats.clone(),
        })
    }
    
    /// Calculate execution order using topological sort
    fn calculate_execution_order(&self) -> ConversionResult<Vec<Vec<String>>> {
        let mut execution_order = Vec::new();
        let mut in_degree: HashMap<NodeIndex, usize> = HashMap::new();
        
        // Calculate in-degrees
        for node_idx in self.execution_dag.node_identifiers() {
            let incoming_edges = self.execution_dag.edges_directed(node_idx, petgraph::Direction::Incoming).count();
            in_degree.insert(node_idx, incoming_edges);
        }
        
        // Topological sort with batching
        while !in_degree.is_empty() {
            // Find all nodes with in-degree 0
            let ready_nodes: Vec<NodeIndex> = in_degree.iter()
                .filter(|(_, &degree)| degree == 0)
                .map(|(&node, _)| node)
                .collect();
            
            if ready_nodes.is_empty() {
                return Err(Box::new(ttrpg_core::error::ConversionError::PluginError {
                    message: "Circular dependency or invalid DAG state".to_string(),
                    plugin_name: None,
                    source: None,
                }));
            }
            
            // Convert to plugin IDs
            let ready_plugin_ids: Vec<String> = ready_nodes.iter()
                .filter_map(|&node_idx| {
                    self.execution_dag.node_weight(node_idx)
                        .map(|node| node.plugin_id.clone())
                })
                .collect();
            
            execution_order.push(ready_plugin_ids);
            
            // Remove ready nodes and update in-degrees
            for &node_idx in &ready_nodes {
                in_degree.remove(&node_idx);
                
                // Decrease in-degree of neighbors
                for edge_ref in self.execution_dag.edges_directed(node_idx, petgraph::Direction::Outgoing) {
                    let target = edge_ref.target();
                    if let Some(degree) = in_degree.get_mut(&target) {
                        *degree -= 1;
                    }
                }
            }
        }
        
        Ok(execution_order)
    }
    
    /// Execute plugin batch in parallel
    async fn execute_plugin_batch_parallel(
        &mut self,
        plugin_batch: Vec<String>,
        _config: &PipelineConfig,
    ) -> ConversionResult<Vec<Result<String, String>>> {
        // Use rayon for CPU-bound parallel execution
        let results = tokio::task::spawn_blocking(move || {
            plugin_batch.par_iter()
                .map(|plugin_id| {
                    // Simulate plugin execution
                    // TODO: Replace with actual plugin execution
                    Ok(format!("Executed plugin: {}", plugin_id))
                })
                .collect::<Vec<Result<String, String>>>()
        }).await.map_err(|e| ttrpg_core::error::ConversionError::PluginError {
            message: format!("Join error in plugin batch execution: {:?}", e),
            plugin_name: None,
            source: None,
        })?;
        
        Ok(results)
    }
    
    /// Execute single plugin
    async fn execute_single_plugin(&mut self, plugin_id: &str, config: &PipelineConfig) -> ConversionResult<()> {
        tracing::debug!("Executing plugin: {}", plugin_id);
        
        // Update plugin status
        if let Some(&node_idx) = self.node_indices.get(plugin_id) {
            if let Some(node) = self.execution_dag.node_weight_mut(node_idx) {
                node.status = PluginStatus::Running;
            }
        }
        
        // Simulate plugin execution with timeout
        let execution_future = tokio::time::timeout(
            std::time::Duration::from_secs(config.plugin_timeout_seconds),
            async {
                // Actual plugin execution would happen here
                tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                Ok(())
            }
        );
        
        match execution_future.await {
            Ok(Ok(())) => {
                // Mark as completed
                if let Some(&node_idx) = self.node_indices.get(plugin_id) {
                    if let Some(node) = self.execution_dag.node_weight_mut(node_idx) {
                        node.status = PluginStatus::Completed;
                    }
                }
                tracing::debug!("Plugin {} completed successfully", plugin_id);
                Ok(())
            },
            Ok(Err(e)) => {
                // Mark as failed
                if let Some(&node_idx) = self.node_indices.get(plugin_id) {
                    if let Some(node) = self.execution_dag.node_weight_mut(node_idx) {
                        node.status = PluginStatus::Failed(format!("{:?}", e));
                    }
                }
                Err(e)
            },
            Err(_) => {
                // Timeout
                let error = ttrpg_core::error::ConversionError::PluginError {
                    message: format!("Plugin {} timed out", plugin_id),
                    plugin_name: Some(plugin_id.to_string()),
                    source: None,
                };
                if let Some(&node_idx) = self.node_indices.get(plugin_id) {
                    if let Some(node) = self.execution_dag.node_weight_mut(node_idx) {
                        node.status = PluginStatus::Failed("Timeout".to_string());
                    }
                }
                Err(Box::new(error))
            }
        }
    }
    
    /// Get orchestration statistics
    pub fn get_statistics(&self) -> &OrchestrationStats {
        &self.stats
    }
    
    /// Reset orchestrator state
    pub fn reset(&mut self) {
        self.execution_dag = Dag::new();
        self.node_indices.clear();
        self.stats = OrchestrationStats::default();
    }
}

/// Result of pipeline execution
#[derive(Debug)]
pub struct PipelineExecutionResult {
    /// Successfully executed plugins
    pub executed_plugins: Vec<String>,
    /// Failed plugins
    pub failed_plugins: Vec<String>,
    /// Total execution time in milliseconds
    pub execution_time_ms: u64,
    /// Execution statistics
    pub stats: OrchestrationStats,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_plugin_discovery() {
        let mut orchestrator = PluginOrchestrator::new().unwrap();
        orchestrator.discover_plugins().unwrap();
        
        assert!(orchestrator.stats.plugins_registered > 0);
    }
    
    #[tokio::test]
    async fn test_pipeline_execution() {
        let mut orchestrator = PluginOrchestrator::new().unwrap();
        orchestrator.discover_plugins().unwrap();
        
        let config = PipelineConfig::default();
        let result = orchestrator.execute_pipeline(&config).await.unwrap();
        
        assert!(result.executed_plugins.len() + result.failed_plugins.len() > 0);
    }
}
