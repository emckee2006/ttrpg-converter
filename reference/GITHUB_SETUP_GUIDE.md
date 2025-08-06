# TTRPGConverter - GitHub Repository Setup Guide

## 🚀 **CLEAN SLATE REPOSITORY CREATION**

### **Step 1: Create New GitHub Repository** 
1. Go to https://github.com/new
2. **Repository name**: `ttrpg-converter`
3. **Description**: "Pure Rust TTRPG campaign converter - Roll20, Foundry VTT, and multi-format output support"
4. **Visibility**: Public (for community engagement)
5. **Initialize**: ✅ Add README, ✅ Add .gitignore (Rust), ✅ Choose license (MIT/Apache-2.0)

### **Step 2: Initial Repository Structure**
```bash
# Clone the new repository
git clone https://github.com/[USERNAME]/ttrpg-converter.git
cd ttrpg-converter

# Create clean Rust workspace structure
mkdir -p {ttrpg-core,ttrpg-formats,ttrpg-assets,ttrpg-gui,ttrpg-cli,docs,reference}

# Initialize Cargo workspace
cat > Cargo.toml << 'EOF'
[workspace]
members = [
    "ttrpg-core",
    "ttrpg-formats", 
    "ttrpg-assets",
    "ttrpg-gui",
    "ttrpg-cli"
]
resolver = "2"

[workspace.dependencies]
# Core dependencies
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"
tracing = "0.1"
tracing-subscriber = "0.3"
anyhow = "1.0"

# CLI dependencies
clap = { version = "4.0", features = ["derive"] }
indicatif = "0.17"
colored = "2.0"
dialoguer = "0.10"

# GUI dependencies
egui = "0.24"
eframe = "0.24"

# Performance dependencies
rayon = "1.7"
parking_lot = "0.12"
EOF

# Create individual crate directories
for crate in ttrpg-core ttrpg-formats ttrpg-assets ttrpg-gui ttrpg-cli; do
    cargo new --lib $crate
done

# Initial commit
git add .
git commit -m "Initial clean slate Rust workspace structure

- Pure Rust architecture with proper workspace organization
- Core crates: core, formats, assets, gui, cli  
- Modern dependency management with workspace inheritance
- Foundation for TTRPGConverter development"
git push origin main
```

### **Step 3: GitHub Projects Setup**
1. **Navigate to**: Repository → Projects → New Project
2. **Template**: Board
3. **Project name**: "TTRPGConverter Development"
4. **Columns**:
   - 📋 **Backlog** - Planned features and tasks
   - 🚀 **Ready** - Tasks ready to start
   - 🔄 **In Progress** - Active development  
   - 👀 **Review** - Code review and testing
   - ✅ **Done** - Completed tasks

### **Step 4: Issue Templates Setup**
Create `.github/ISSUE_TEMPLATE/` with:

**feature_request.md:**
```yaml
---
name: Feature Request
about: Propose a new feature or enhancement
title: '[FEATURE] '
labels: ['enhancement', 'triage']
assignees: ''
---

## 🎯 Feature Description
Brief description of the proposed feature

## 🔧 Implementation Details  
Technical approach and requirements

## ✅ Acceptance Criteria
- [ ] Specific deliverable 1
- [ ] Specific deliverable 2

## 📊 Story Points
Estimated complexity (1-13 Fibonacci scale)
```

**bug_report.md:**
```yaml
---
name: Bug Report
about: Report a bug or issue
title: '[BUG] '
labels: ['bug', 'triage']
assignees: ''
---

## 🐛 Bug Description
Clear description of the issue

## 🔄 Steps to Reproduce
1. Step 1
2. Step 2  
3. Step 3

## ✅ Expected Behavior
What should happen

## 📊 Actual Behavior  
What actually happens

## 🔧 Environment
- OS: [Windows/Linux/macOS]
- Version: [TTRPGConverter version]
```

## 📋 **INITIAL MILESTONE CREATION**

### **Milestone 1: Pure Rust Foundation** 
- **Due date**: 3 weeks from start
- **Description**: Clean slate Rust architecture with core services
- **Issues to create**:
  - #1: Core Architecture Foundation (5 pts)
  - #2: Database Abstraction Layer (8 pts)  
  - #3: Core Services Implementation (10 pts)
  - #4: CLI Interface Implementation (8 pts)

### **Milestone 2: Native GUI Migration**
- **Due date**: 4.5 weeks from start
- **Description**: DearPyGUI → egui migration with enhanced UX
- **Issues to create**:
  - #5: DearPyGUI → egui Migration (10 pts)
  - #6: Enhanced GUI Features (8 pts)
  - #7: GUI Integration Testing (5 pts)

### **Milestone 3: Entity Processing Engine**
- **Due date**: 7 weeks from start  
- **Description**: Complete entity conversion pipeline
- **Issues to create**:
  - #8: Core Entity Framework (10 pts)
  - #9: Character/Actor Processing (8 pts)
  - #10: Scene Processing (10 pts)
  - #11: Items & Journal Processing (6 pts)
  - #12: Tables, Macros & Combat (8 pts)

## 🔗 **REFERENCE PRESERVATION**

```bash
# Copy existing work as reference (DO NOT DELETE)
mkdir reference/r20converter-legacy
cp -r ../r20converter-rust reference/r20converter-pyo3
cp -r ../obsidian-project reference/obsidian-planning

# Document the reference structure
cat > reference/README.md << 'EOF'
# Reference Materials for TTRPGConverter

## r20converter-pyo3/
Previous PyO3 integration approach - use for:
- Functionality requirements reference
- Service interface patterns  
- Entity structure insights
- Lessons learned documentation

## obsidian-planning/
Original planning documents - use for:
- Feature requirements gathering
- User story identification
- Technical architecture insights
- Timeline estimation references

## Migration Strategy
These references inform clean slate development but are NOT migrated directly.
All new implementation uses pure Rust patterns and modern architecture.
EOF

git add reference/
git commit -m "Preserve legacy implementation and planning as reference

- PyO3 version provides functionality requirements
- Obsidian planning provides user stories and technical insights
- Reference-only - not migrated to clean slate architecture"
git push origin main
```

## 📊 **SUCCESS METRICS**

**Setup Complete When:**
- ✅ GitHub repository with clean Rust workspace
- ✅ GitHub Projects with proper milestone structure  
- ✅ Initial issues created for first milestone
- ✅ Reference materials preserved and documented
- ✅ Development environment ready for T1: Core Architecture Foundation

**Total Setup Time**: ~30 minutes
**Ready to begin**: Pure Rust development with proper planning integration
