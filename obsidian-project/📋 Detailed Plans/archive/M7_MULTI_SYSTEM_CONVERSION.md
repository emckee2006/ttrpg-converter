# M7: Multi-System Conversion Engine - Junior Developer Implementation Guide

## 🎯 **MILESTONE 7 OVERVIEW**
**Duration**: 4 weeks | **Total Points**: 30 | **Priority**: 🔥 HIGH

Universal TTRPG system conversion using ValidationPlugin for rule validation and ReferenceTrackingPlugin for cross-system entity mapping - D&D 5e ↔ Pathfinder 1e/2e with processing plugin coordination.

### 🧪 **TESTING REQUIREMENTS**
**Every M7 task must include comprehensive testing before being marked complete:**
- ✅ **Unit Tests** - Individual conversion function testing (>95% coverage)
- ✅ **Integration Tests** - Full character/creature conversion with real data
- ✅ **Property Tests** - Bidirectional conversion validation (A→B→A consistency)
- ✅ **Accuracy Tests** - Statistical validation of conversion accuracy
- ✅ **Edge Case Tests** - Unusual character builds and corner cases

---

## **T7.1: Universal Conversion Framework**
**Duration**: 8 days | **Points**: 12 | **Priority**: 🚨 CRITICAL
**Dependencies**: M2.0 Processing Plugin Architecture Foundation (ValidationPlugin + ReferenceTrackingPlugin)

### **Implementation Steps for Junior Developer**

**Step 1: Conversion Rule Engine Integration with Processing Plugins**
Update `ttrpg-processing-plugins/Cargo.toml` for rule engine support:
```toml
[dependencies]
# Rule engine and pattern matching (eliminate reinvented wheels)
rete = "0.3"              # Professional RETE algorithm rule engine  
pest = "2.7"              # PEG parser for rule DSLs
handlebars = "4.4"        # Template engine for rule generation
serde_yaml = "0.9"        # YAML configuration for rule sets

# Functional programming for data transformations
itertools = { workspace = true }
serde_transcode = "1.1"   # Efficient JSON transformations

# Statistical analysis for conversion accuracy
statrs = "0.16"           # Statistical functions
approx = "0.5"            # Floating point comparisons
```

**Step 2: System Detection Framework**
Create `ttrpg-core/src/systems/mod.rs`:
```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Universal system detection and identification
#[derive(Debug, Clone, PartialEq)]
pub enum GameSystem {
    DnD5e,
    PathfinderFirst,
    PathfinderSecond,
    Generic,
    Unknown,
}

/// System detection engine with confidence scoring
pub struct SystemDetector {
    detection_rules: HashMap<GameSystem, SystemSignature>,
    confidence_threshold: f64,
}

#[derive(Debug, Clone)]
pub struct SystemSignature {
    /// Required fields for system identification
    required_fields: Vec<String>,
    /// Optional fields that increase confidence
    optional_fields: Vec<String>,
    /// Value patterns that indicate this system
    value_patterns: HashMap<String, SystemPattern>,
    /// Statistical characteristics
    statistical_markers: StatisticalMarkers,
}

#[derive(Debug, Clone)]
pub struct SystemPattern {
    field_name: String,
    expected_values: Vec<String>,
    pattern_type: PatternType,
}

#[derive(Debug, Clone)]
pub enum PatternType {
    ExactMatch,
    Contains,
    Regex(String),
    NumericRange { min: f64, max: f64 },
}

impl SystemDetector {
    pub fn new() -> Self {
        let mut detector = Self {
            detection_rules: HashMap::new(),
            confidence_threshold: 0.80,
        };
        
        // Initialize system signatures
        detector.initialize_system_signatures();
        detector
    }
    
    /// Detect system with confidence scoring
    pub fn detect_system(&self, campaign_data: &serde_json::Value) -> SystemDetectionResult {
        let mut detection_scores = HashMap::new();
        
        for (system, signature) in &self.detection_rules {
            let confidence = self.calculate_confidence(campaign_data, signature);
            detection_scores.insert(*system, confidence);
        }
        
        // Find highest confidence system
        let (detected_system, confidence) = detection_scores
            .iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap();
            
        SystemDetectionResult {
            system: if *confidence >= self.confidence_threshold {
                *detected_system
            } else {
                GameSystem::Unknown
            },
            confidence: *confidence,
            all_scores: detection_scores,
        }
    }
    
    /// Initialize detection signatures for each system
    fn initialize_system_signatures(&mut self) {
        // D&D 5e signature
        self.detection_rules.insert(GameSystem::DnD5e, SystemSignature {
            required_fields: vec![
                "abilities".to_string(),
                "level".to_string(),
            ],
            optional_fields: vec![
                "class".to_string(),
                "race".to_string(),
                "proficiency_bonus".to_string(),
            ],
            value_patterns: {
                let mut patterns = HashMap::new();
                patterns.insert("abilities".to_string(), SystemPattern {
                    field_name: "abilities".to_string(),
                    expected_values: vec!["strength", "dexterity", "constitution", "intelligence", "wisdom", "charisma"]
                        .into_iter().map(String::from).collect(),
                    pattern_type: PatternType::Contains,
                });
                patterns
            },
            statistical_markers: StatisticalMarkers {
                typical_stat_range: (8, 20),
                typical_level_range: (1, 20),
                skill_count_range: (0, 18),
            },
        });
        
        // Pathfinder 1e signature  
        self.detection_rules.insert(GameSystem::PathfinderFirst, SystemSignature {
            required_fields: vec![
                "abilities".to_string(),
                "level".to_string(),
            ],
            optional_fields: vec![
                "base_attack_bonus".to_string(),
                "saves".to_string(),
                "skills".to_string(),
            ],
            value_patterns: {
                let mut patterns = HashMap::new();
                patterns.insert("saves".to_string(), SystemPattern {
                    field_name: "saves".to_string(),
                    expected_values: vec!["fortitude", "reflex", "will"]
                        .into_iter().map(String::from).collect(),
                    pattern_type: PatternType::Contains,
                });
                patterns
            },
            statistical_markers: StatisticalMarkers {
                typical_stat_range: (7, 25),  // PF1e has wider stat ranges
                typical_level_range: (1, 20),
                skill_count_range: (0, 40),   // Many more skills
            },
        });
        
        // Similar initialization for PathfinderSecond...
    }
}

#[derive(Debug, Clone)]
pub struct SystemDetectionResult {
    pub system: GameSystem,
    pub confidence: f64,
    pub all_scores: HashMap<GameSystem, f64>,
}

#[derive(Debug, Clone)]
struct StatisticalMarkers {
    typical_stat_range: (i32, i32),
    typical_level_range: (i32, i32), 
    skill_count_range: (i32, i32),
}
```

**Step 3: Abstract Conversion Pipeline**
Create `ttrpg-core/src/conversion/pipeline.rs`:
```rust
/// Universal conversion pipeline with plugin architecture
pub struct ConversionPipeline {
    source_system: GameSystem,
    target_system: GameSystem,
    conversion_rules: ConversionRuleSet,
    rule_engine: rete::Engine,
}

impl ConversionPipeline {
    /// Create conversion pipeline for specific system pair
    pub fn new(source: GameSystem, target: GameSystem) -> Result<Self, ConversionError> {
        let conversion_rules = ConversionRuleSet::load_for_systems(source, target)?;
        let rule_engine = rete::Engine::new();
        
        // Load conversion rules into RETE engine
        let mut pipeline = Self {
            source_system: source,
            target_system: target,
            conversion_rules,
            rule_engine,
        };
        
        pipeline.initialize_rule_engine()?;
        Ok(pipeline)
    }
    
    /// Convert entity with full validation and error handling
    pub async fn convert_entity(&self, entity: &EntityData) -> ConversionResult<EntityData> {
        // Phase 1: Validate source entity
        let validation_result = self.validate_source_entity(entity)?;
        if !validation_result.is_valid {
            return Err(ConversionError::InvalidSourceEntity {
                issues: validation_result.issues,
            });
        }
        
        // Phase 2: Apply conversion rules
        let converted_data = self.apply_conversion_rules(entity).await?;
        
        // Phase 3: Validate converted entity
        let target_validation = self.validate_target_entity(&converted_data)?;
        if !target_validation.is_valid {
            return Err(ConversionError::ConversionValidationFailed {
                issues: target_validation.issues,
            });
        }
        
        // Phase 4: Calculate conversion quality score
        let quality_score = self.calculate_conversion_quality(entity, &converted_data)?;
        
        Ok(ConvertedEntity {
            data: converted_data,
            quality_score,
            conversion_notes: self.generate_conversion_notes(entity, &converted_data)?,
            warnings: target_validation.warnings,
        })
    }
    
    /// Apply RETE rule engine for complex conversions
    async fn apply_conversion_rules(&self, entity: &EntityData) -> ConversionResult<EntityData> {
        let mut working_memory = rete::WorkingMemory::new();
        
        // Insert entity facts into working memory
        for (field, value) in entity.fields() {
            working_memory.insert(rete::Fact::new(field.clone(), value.clone()));
        }
        
        // Execute rule engine
        let inference_result = self.rule_engine.infer(&mut working_memory)?;
        
        // Extract converted entity from inference results
        let converted_entity = self.extract_converted_entity(&inference_result)?;
        
        Ok(converted_entity)
    }
}

/// Conversion rule set with YAML configuration
#[derive(Debug, Deserialize, Serialize)]
pub struct ConversionRuleSet {
    pub name: String,
    pub source_system: String,
    pub target_system: String,
    pub stat_mappings: HashMap<String, StatMapping>,
    pub skill_mappings: HashMap<String, SkillMapping>,
    pub class_mappings: HashMap<String, ClassMapping>,
    pub spell_mappings: HashMap<String, SpellMapping>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StatMapping {
    pub source_stat: String,
    pub target_stat: String,
    pub conversion_formula: String,  // Mathematical expression
    pub min_value: Option<i32>,
    pub max_value: Option<i32>,
}
```

---

## **T7.2: D&D 5e ↔ Pathfinder Conversion Rules**
**Duration**: 10 days | **Points**: 15 | **Priority**: 🔥 HIGH
**Dependencies**: T7.1 Complete

### **Implementation Steps for Junior Developer**

**Step 1: Create Conversion Rule Configurations**
Create `ttrpg-core/config/conversions/dnd5e_to_pf1e.yaml`:
```yaml
name: "D&D 5e to Pathfinder 1e Conversion"
source_system: "DnD5e"
target_system: "PathfinderFirst"

# Ability score conversion (6 stats → expanded)
stat_mappings:
  strength:
    source_stat: "strength"
    target_stat: "strength" 
    conversion_formula: "value"  # Direct mapping
    min_value: 7
    max_value: 25
    
  dexterity:
    source_stat: "dexterity"
    target_stat: "dexterity"
    conversion_formula: "value"
    
  constitution:
    source_stat: "constitution" 
    target_stat: "constitution"
    conversion_formula: "value"
    
  intelligence:
    source_stat: "intelligence"
    target_stat: "intelligence"
    conversion_formula: "value"
    
  wisdom:
    source_stat: "wisdom"
    target_stat: "wisdom"  
    conversion_formula: "value"
    
  charisma:
    source_stat: "charisma"
    target_stat: "charisma"
    conversion_formula: "value"

# Skill conversion (18 → 35+ skills)
skill_mappings:
  acrobatics:
    source_skill: "acrobatics"
    target_skill: "acrobatics"
    ability_base: "dexterity"
    
  athletics:
    source_skill: "athletics" 
    target_skill: "climb"  # Split into multiple PF skills
    ability_base: "strength"
    additional_skills: ["swim"]
    
  deception:
    source_skill: "deception"
    target_skill: "bluff"
    ability_base: "charisma"
    
  history:
    source_skill: "history"
    target_skill: "knowledge_history"
    ability_base: "intelligence"
    
  insight:
    source_skill: "insight"
    target_skill: "sense_motive" 
    ability_base: "wisdom"
    
  investigation:
    source_skill: "investigation"
    target_skill: "perception"  # Closest equivalent
    ability_base: "intelligence"
    conversion_notes: "Investigation mapped to Perception with INT base"

# Class level equivalencies  
class_mappings:
  fighter:
    source_class: "fighter"
    target_class: "fighter"
    level_conversion: "direct"  # 1:1 mapping
    
  wizard:
    source_class: "wizard"
    target_class: "wizard" 
    level_conversion: "direct"
    spell_progression: "keep_known_spells"
    
  rogue:
    source_class: "rogue"
    target_class: "rogue"
    level_conversion: "direct"
    
  barbarian:
    source_class: "barbarian"
    target_class: "barbarian"
    level_conversion: "direct"

# Spell system conversion
spell_mappings:
  cantrips:
    source_level: 0
    target_level: 0
    conversion_type: "direct"
    
  first_level:
    source_level: 1
    target_level: 1
    conversion_type: "direct"
    
  # Higher level spells...
  
# Feat conversions
feat_mappings:
  great_weapon_master:
    source_feat: "Great Weapon Master"
    target_feat: "Power Attack"
    conversion_notes: "Similar mechanics, different implementation"
```

**Step 2: Bidirectional Conversion Implementation**
Create `ttrpg-core/src/conversion/bidirectional.rs`:
```rust
/// Bidirectional conversion engine with round-trip validation
pub struct BidirectionalConverter {
    forward_pipeline: ConversionPipeline,
    reverse_pipeline: ConversionPipeline,
}

impl BidirectionalConverter {
    pub fn new(system_a: GameSystem, system_b: GameSystem) -> Result<Self, ConversionError> {
        let forward = ConversionPipeline::new(system_a, system_b)?;
        let reverse = ConversionPipeline::new(system_b, system_a)?;
        
        Ok(Self {
            forward_pipeline: forward,
            reverse_pipeline: reverse,
        })
    }
    
    /// Test round-trip conversion accuracy
    pub async fn test_round_trip_accuracy(&self, entity: &EntityData) -> RoundTripResult {
        // Convert A → B
        let forward_result = self.forward_pipeline.convert_entity(entity).await?;
        
        // Convert B → A  
        let reverse_result = self.reverse_pipeline.convert_entity(&forward_result.data).await?;
        
        // Compare original vs round-trip result
        let accuracy = self.calculate_round_trip_accuracy(entity, &reverse_result.data)?;
        
        RoundTripResult {
            original: entity.clone(),
            forward_converted: forward_result.data,
            reverse_converted: reverse_result.data,
            accuracy_score: accuracy,
            data_preservation: self.analyze_data_preservation(entity, &reverse_result.data)?,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_dnd5e_to_pf1e_conversion() {
        let converter = BidirectionalConverter::new(
            GameSystem::DnD5e, 
            GameSystem::PathfinderFirst
        ).unwrap();
        
        // Create test D&D 5e fighter
        let dnd5e_fighter = create_test_dnd5e_fighter();
        
        let result = converter.forward_pipeline.convert_entity(&dnd5e_fighter).await.unwrap();
        
        // Verify conversion accuracy
        assert!(result.quality_score >= 0.90);
        assert_eq!(result.data.get_field("class"), Some("fighter"));
        assert!(result.data.has_field("base_attack_bonus"));  // PF1e specific
    }
    
    #[tokio::test]
    async fn test_round_trip_accuracy() {
        let converter = BidirectionalConverter::new(
            GameSystem::DnD5e,
            GameSystem::PathfinderFirst
        ).unwrap();
        
        let original = create_test_dnd5e_character();
        let round_trip = converter.test_round_trip_accuracy(&original).await.unwrap();
        
        // Should preserve core character identity
        assert!(round_trip.accuracy_score >= 0.85);
        assert_eq!(
            original.get_field("name"),
            round_trip.reverse_converted.get_field("name")
        );
    }
}
```

---

## **T7.3: Advanced Conversion Features & Quality Control**
**Duration**: 6 days | **Points**: 8 | **Priority**: 🟡 MEDIUM
**Dependencies**: T7.2 Complete

**Step 1: Conversion Quality Analytics**
```rust
/// Statistical analysis of conversion accuracy and quality
pub struct ConversionAnalytics {
    accuracy_metrics: HashMap<String, f64>,
    conversion_statistics: ConversionStatistics,
}

impl ConversionAnalytics {
    /// Analyze conversion accuracy across large datasets
    pub async fn analyze_conversion_batch(&mut self, conversions: Vec<ConversionResult>) -> AnalysisReport {
        let mut accuracy_scores = Vec::new();
        let mut quality_metrics = HashMap::new();
        
        for conversion in conversions {
            // Statistical analysis of conversion quality
            let accuracy = self.calculate_statistical_accuracy(&conversion)?;
            accuracy_scores.push(accuracy);
            
            // Track specific metric accuracy
            for (metric, score) in conversion.metric_scores {
                quality_metrics.entry(metric).or_insert_with(Vec::new).push(score);
            }
        }
        
        // Calculate overall statistics
        let mean_accuracy = statrs::statistics::Data::new(accuracy_scores).mean().unwrap();
        let std_deviation = statrs::statistics::Data::new(accuracy_scores).std_dev().unwrap();
        
        AnalysisReport {
            overall_accuracy: mean_accuracy,
            accuracy_std_dev: std_deviation,
            per_metric_accuracy: quality_metrics,
            recommendations: self.generate_improvement_recommendations(&quality_metrics),
        }
    }
}
```

**Acceptance Criteria for M7:**
- [ ] ✅ 95%+ accurate stat conversion between systems
- [ ] ✅ Automatic system detection with 90%+ accuracy
- [ ] ✅ Support for D&D 5e ↔ Pathfinder 1e/2e ↔ Generic
- [ ] ✅ User-customizable conversion rules via YAML
- [ ] ✅ Round-trip conversion accuracy ≥85%
- [ ] ✅ Statistical validation with large character datasets
- [ ] ✅ Comprehensive edge case handling
