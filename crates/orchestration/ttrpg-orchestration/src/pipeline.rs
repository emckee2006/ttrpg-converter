//! Plugin Pipeline Orchestration with Daggy
//!
//! Advanced plugin orchestration system using directed acyclic graphs (DAGs) for
//! managing complex plugin execution pipelines with dependencies, parallelization,
//! and error handling.

use daggy::petgraph::visit::IntoNodeIdentifiers;
use daggy::{Dag, NodeIndex, Walker};
use std::collections::{HashMap, HashSet};
use std::fmt;
use tracing::{debug, info, warn};

// Import types from ttrpg-core, ttrpg-traits, and ttrpg-types
use ttrpg_core::{error::{ConversionResult, ConversionError}};
use ttrpg_traits::{PluginInfo};
use ttrpg_types::Campaign;

/// Plugin execution node in the pipeline DAG
#[derive(Debug, Clone)]
pub struct PipelineNode {
    /// Unique identifier for this node
    pub id: String,
    /// Plugin information and metadata
    pub plugin_info: PluginInfo,
    /// Plugin type for execution dispatch
    pub plugin_type: PipelinePluginType,
    /// Execution priority (lower = higher priority)
    pub priority: u32,
    /// Whether this node can execute in parallel with others
    pub parallel_safe: bool,
}

/// Plugin type enumeration for pipeline dispatch
#[derive(Debug, Clone)]
pub enum PipelinePluginType {
    Input(String),      // Input format identifier
    Validation(String), // Validation rule identifier
    Asset(String),      // Asset processing identifier
    Export(String),     // Export format identifier
}

/// Pipeline execution statistics
#[derive(Debug, Clone)]
pub struct PipelineStats {
    pub total_nodes: usize,
    pub executed_nodes: usize,
    pub parallel_nodes: usize,
    pub execution_time_ms: u64,
    pub memory_usage_mb: f64,
}

/// Pipeline execution context and data
#[derive(Debug, Clone)]
pub struct PipelineContext {
    /// Campaign data being processed
    pub campaign: Campaign,
    /// Current processing stage
    pub stage: String,
    /// Plugin-specific context data
    pub context_data: HashMap<String, serde_json::Value>,
    /// Execution statistics
    pub stats: PipelineStats,
}

/// Pipeline configuration options
#[derive(Debug, Clone)]
pub struct PipelineConfig {
    /// Maximum parallel execution threads
    pub max_parallelism: usize,
    /// Enable pipeline validation before execution
    pub validate_before_execution: bool,
    /// Continue execution on non-critical errors
    pub continue_on_error: bool,
    /// Log pipeline execution details
    pub verbose_logging: bool,
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            max_parallelism: num_cpus::get(),
            validate_before_execution: true,
            continue_on_error: false,
            verbose_logging: true,
        }
    }
}

/// Advanced Plugin Pipeline Orchestrator
///
/// Coordinates plugin execution using DAG-based pipeline management with:
/// - Dependency resolution and topological sorting
/// - Parallel execution of independent plugins  
/// - Error handling and recovery strategies
/// - Pipeline validation and optimization
/// - Real-time performance monitoring
pub struct PluginPipeline {
    /// DAG representing plugin execution order and dependencies
    dag: Dag<PipelineNode, f32>,
    /// Node lookup by plugin ID
    node_indices: HashMap<String, NodeIndex>,
    /// Execution configuration
    config: PipelineConfig,
    /// Pipeline metadata
    name: String,
    version: String,
}

impl PluginPipeline {
    /// Create a new plugin pipeline
    pub fn new(name: &str, version: &str) -> Self {
        Self {
            dag: Dag::new(),
            node_indices: HashMap::new(),
            config: PipelineConfig::default(),
            name: name.to_string(),
            version: version.to_string(),
        }
    }

    /// Add a plugin node to the pipeline
    pub fn add_plugin(&mut self, node: PipelineNode) -> ConversionResult<NodeIndex> {
        if self.node_indices.contains_key(&node.id) {
            return Err(ConversionError::validation(
                "pipeline",
                &format!("Plugin node '{}' already exists in pipeline", node.id)
            ).into());
        }

        let node_index = self.dag.add_node(node.clone());
        self.node_indices.insert(node.id.clone(), node_index);

        info!(
            plugin_id = %node.id,
            plugin_type = ?node.plugin_type,
            priority = node.priority,
            "Added plugin to pipeline"
        );

        Ok(node_index)
    }

    /// Add a dependency edge between two plugins
    pub fn add_dependency(
        &mut self,
        from_id: &str,
        to_id: &str,
        weight: f32,
    ) -> ConversionResult<()> {
        let from_index = self.node_indices.get(from_id).ok_or_else(|| {
            ConversionError::validation(
                "pipeline",
                &format!("Plugin '{from_id}' not found in pipeline")
            )
        })?;

        let to_index = self.node_indices.get(to_id).ok_or_else(|| {
            ConversionError::validation(
                "pipeline",
                &format!("Plugin '{to_id}' not found in pipeline")
            )
        })?;

        // Check for cycle before adding edge
        if self.would_create_cycle(*from_index, *to_index) {
            return Err(ConversionError::validation(
                "pipeline",
                &format!("Adding dependency from '{from_id}' to '{to_id}' would create a cycle")
            ).into());
        }

        self.dag
            .add_edge(*from_index, *to_index, weight)
            .map_err(|e| {
                ConversionError::validation(
                    "pipeline",
                    &format!("Failed to add dependency edge: {e:?}")
                )
            })?;

        debug!(
            from = %from_id,
            to = %to_id,
            weight = weight,
            "Added pipeline dependency"
        );

        Ok(())
    }

    /// Validate the pipeline for execution
    pub fn validate(&self) -> ConversionResult<()> {
        // Check for cycles (should be impossible due to DAG structure)
        if daggy::petgraph::algo::is_cyclic_directed(self.dag.graph()) {
            return Err(ConversionError::validation(
                "pipeline",
                "Pipeline contains cycles"
            ).into());
        }

        // Ensure we have at least one input plugin
        let has_input = self
            .dag
            .raw_nodes()
            .iter()
            .any(|node| matches!(node.weight.plugin_type, PipelinePluginType::Input(_)));

        if !has_input {
            return Err(ConversionError::validation(
                "pipeline",
                "Pipeline must contain at least one input plugin"
            ).into());
        }

        // Check for unreachable nodes
        let reachable = self.get_reachable_nodes();
        if reachable.len() != self.dag.node_count() {
            warn!(
                reachable = reachable.len(),
                total = self.dag.node_count(),
                "Pipeline contains unreachable nodes"
            );
        }

        info!(
            nodes = self.dag.node_count(),
            edges = self.dag.edge_count(),
            "Pipeline validation successful"
        );

        Ok(())
    }

    /// Execute the pipeline with the given context
    pub async fn execute(&self, context: &mut PipelineContext) -> ConversionResult<()> {
        if self.config.validate_before_execution {
            self.validate()?;
        }

        info!(
            pipeline = %self.name,
            nodes = self.dag.node_count(),
            "Starting pipeline execution"
        );

        let execution_order = self.get_execution_order()?;
        let start_time = std::time::Instant::now();

        context.stats.total_nodes = self.dag.node_count();
        context.stats.executed_nodes = 0;

        for batch in execution_order {
            self.execute_batch(&batch, context).await?;
        }

        context.stats.execution_time_ms = start_time.elapsed().as_millis() as u64;

        info!(
            pipeline = %self.name,
            execution_time_ms = context.stats.execution_time_ms,
            executed_nodes = context.stats.executed_nodes,
            "Pipeline execution completed"
        );

        Ok(())
    }

    /// Get topologically sorted execution order with parallelization
    fn get_execution_order(&self) -> ConversionResult<Vec<Vec<NodeIndex>>> {
        let mut execution_order = Vec::new();
        let mut remaining: HashSet<NodeIndex> = self.dag.node_identifiers().collect();
        let mut in_degree: HashMap<NodeIndex, usize> = HashMap::new();

        // Calculate in-degrees
        for node_index in self.dag.node_identifiers() {
            let degree = self.dag.parents(node_index).iter(&self.dag).count();
            in_degree.insert(node_index, degree);
        }

        // Process nodes in topological order with parallelization
        while !remaining.is_empty() {
            let mut current_batch = Vec::new();

            // Find all nodes with in-degree 0
            let ready_nodes: Vec<NodeIndex> = remaining
                .iter()
                .filter(|&&node| in_degree.get(&node).unwrap_or(&0) == &0)
                .copied()
                .collect();

            if ready_nodes.is_empty() {
                return Err(ConversionError::validation(
                    "pipeline",
                    "No ready nodes found - possible cycle detected"
                ).into());
            }

            // Group parallel-safe nodes together
            for &node_index in &ready_nodes {
                let node = &self.dag[node_index];
                if node.parallel_safe && current_batch.len() < self.config.max_parallelism {
                    current_batch.push(node_index);
                } else if current_batch.is_empty() {
                    // Always execute at least one node
                    current_batch.push(node_index);
                    break;
                }
            }

            // Remove processed nodes and update in-degrees
            for &node_index in &current_batch {
                remaining.remove(&node_index);

                // Update in-degrees of children
                for child in self.dag.children(node_index).iter(&self.dag) {
                    let child_index = child.1;
                    if let Some(degree) = in_degree.get_mut(&child_index) {
                        *degree = degree.saturating_sub(1);
                    }
                }
            }

            execution_order.push(current_batch);
        }

        Ok(execution_order)
    }

    /// Execute a batch of nodes in parallel
    async fn execute_batch(
        &self,
        batch: &[NodeIndex],
        context: &mut PipelineContext,
    ) -> ConversionResult<()> {
        if batch.len() == 1 {
            // Single node execution
            self.execute_node(batch[0], context).await?;
        } else {
            // Parallel execution
            context.stats.parallel_nodes += batch.len().saturating_sub(1);

            for &node_index in batch {
                // For now, execute sequentially as async parallel execution
                // would require more complex shared state management
                self.execute_node(node_index, context).await?;
            }
        }

        Ok(())
    }

    /// Execute a single plugin node
    async fn execute_node(
        &self,
        node_index: NodeIndex,
        context: &mut PipelineContext,
    ) -> ConversionResult<()> {
        let node = &self.dag[node_index];

        debug!(
            plugin_id = %node.id,
            plugin_type = ?node.plugin_type,
            "Executing plugin node"
        );

        // Plugin-specific execution logic would go here
        // For now, we'll simulate execution
        match &node.plugin_type {
            PipelinePluginType::Input(format) => {
                info!(format = %format, "Executing input plugin");
            }
            PipelinePluginType::Validation(rule) => {
                info!(rule = %rule, "Executing validation plugin");
            }
            PipelinePluginType::Asset(processor) => {
                info!(processor = %processor, "Executing asset plugin");
            }
            PipelinePluginType::Export(format) => {
                info!(format = %format, "Executing export plugin");
            }
        }

        context.stats.executed_nodes += 1;
        Ok(())
    }

    /// Check if adding an edge would create a cycle
    fn would_create_cycle(&self, from: NodeIndex, to: NodeIndex) -> bool {
        // Use a simple path exists check - if there's already a path from 'to' to 'from',
        // then adding edge from->to would create a cycle
        daggy::petgraph::algo::has_path_connecting(&self.dag.graph(), to, from, None)
    }

    /// Get all reachable nodes from input nodes
    fn get_reachable_nodes(&self) -> HashSet<NodeIndex> {
        let mut reachable = HashSet::new();

        // Find all input nodes
        let input_nodes: Vec<NodeIndex> = self
            .dag
            .raw_nodes()
            .iter()
            .enumerate()
            .filter_map(|(i, node)| match node.weight.plugin_type {
                PipelinePluginType::Input(_) => Some(NodeIndex::new(i)),
                _ => None,
            })
            .collect();

        // DFS from each input node
        for input_node in input_nodes {
            self.dfs_reachable(input_node, &mut reachable);
        }

        reachable
    }

    /// DFS helper for reachability analysis
    fn dfs_reachable(&self, node: NodeIndex, visited: &mut HashSet<NodeIndex>) {
        if visited.insert(node) {
            for child in self.dag.children(node).iter(&self.dag) {
                self.dfs_reachable(child.1, visited);
            }
        }
    }

    /// Get pipeline statistics
    pub fn get_stats(&self) -> PipelineStats {
        PipelineStats {
            total_nodes: self.dag.node_count(),
            executed_nodes: 0,
            parallel_nodes: 0,
            execution_time_ms: 0,
            memory_usage_mb: 0.0,
        }
    }

    /// Configure the pipeline
    pub fn configure(&mut self, config: PipelineConfig) {
        self.config = config;
    }
}

impl fmt::Display for PluginPipeline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PluginPipeline '{}' v{} ({} nodes, {} edges)",
            self.name,
            self.version,
            self.dag.node_count(),
            self.dag.edge_count()
        )
    }
}

/// Pipeline builder for fluent construction
pub struct PipelineBuilder {
    pipeline: PluginPipeline,
}

impl PipelineBuilder {
    /// Create a new pipeline builder
    pub fn new(name: &str, version: &str) -> Self {
        Self { pipeline: PluginPipeline::new(name, version) }
    }

    /// Add a plugin with fluent API
    pub fn with_plugin(mut self, node: PipelineNode) -> ConversionResult<Self> {
        self.pipeline.add_plugin(node)?;
        Ok(self)
    }

    /// Add a dependency with fluent API
    pub fn with_dependency(
        mut self,
        from_id: &str,
        to_id: &str,
        weight: f32,
    ) -> ConversionResult<Self> {
        self.pipeline.add_dependency(from_id, to_id, weight)?;
        Ok(self)
    }

    /// Configure the pipeline
    pub fn with_config(mut self, config: PipelineConfig) -> Self {
        self.pipeline.configure(config);
        self
    }

    /// Build the final pipeline
    pub fn build(self) -> ConversionResult<PluginPipeline> {
        self.pipeline.validate()?;
        Ok(self.pipeline)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ttrpg_traits::PluginInfo;

    fn create_test_plugin_info() -> PluginInfo {
        PluginInfo {
            name: "Test Plugin".to_string(),
            version: "1.0.0".to_string(),
            description: "Test plugin for pipeline testing".to_string(),
            author: "Test Author".to_string(),
            supported_features: vec!["test".to_string()],
            dependencies: vec![],
        }
    }

    #[test]
    fn test_pipeline_node_creation() {
        let plugin_info = create_test_plugin_info();
        let node = PipelineNode {
            id: "test-node".to_string(),
            plugin_info: plugin_info.clone(),
            plugin_type: PipelinePluginType::Input("test-input".to_string()),
            priority: 1,
            parallel_safe: true,
        };

        assert_eq!(node.id, "test-node");
        assert_eq!(node.plugin_info.name, "Test Plugin");
        assert_eq!(node.priority, 1);
        assert!(node.parallel_safe);
    }

    #[test]
    fn test_pipeline_stats_creation() {
        let stats = PipelineStats {
            total_nodes: 5,
            executed_nodes: 3,
            parallel_nodes: 2,
            execution_time_ms: 1000,
            memory_usage_mb: 128.5,
        };

        assert_eq!(stats.total_nodes, 5);
        assert_eq!(stats.executed_nodes, 3);
        assert_eq!(stats.parallel_nodes, 2);
        assert_eq!(stats.execution_time_ms, 1000);
        assert_eq!(stats.memory_usage_mb, 128.5);
    }

    #[test]
    fn test_pipeline_plugin_type_variants() {
        let input_type = PipelinePluginType::Input("roll20".to_string());
        let validation_type = PipelinePluginType::Validation("schema".to_string());
        let asset_type = PipelinePluginType::Asset("image".to_string());
        let export_type = PipelinePluginType::Export("foundry".to_string());

        // Test that variants can be created and compared
        match input_type {
            PipelinePluginType::Input(format) => assert_eq!(format, "roll20"),
            _ => panic!("Expected Input variant"),
        }

        match validation_type {
            PipelinePluginType::Validation(rule) => assert_eq!(rule, "schema"),
            _ => panic!("Expected Validation variant"),
        }

        match asset_type {
            PipelinePluginType::Asset(processor) => assert_eq!(processor, "image"),
            _ => panic!("Expected Asset variant"),
        }

        match export_type {
            PipelinePluginType::Export(format) => assert_eq!(format, "foundry"),
            _ => panic!("Expected Export variant"),
        }
    }

    #[test]
    fn test_pipeline_config_creation() {
        let config = PipelineConfig {
            max_parallelism: 4,
            validate_before_execution: true,
            continue_on_error: false,
            verbose_logging: true,
        };

        assert_eq!(config.max_parallelism, 4);
        assert!(config.validate_before_execution);
        assert!(!config.continue_on_error);
        assert!(config.verbose_logging);
    }

    #[test]
    fn test_pipeline_config_default() {
        let config = PipelineConfig::default();
        
        assert!(config.max_parallelism > 0); // Should be num_cpus::get()
        assert!(config.validate_before_execution);
        assert!(!config.continue_on_error);
        assert!(config.verbose_logging);
    }
}
