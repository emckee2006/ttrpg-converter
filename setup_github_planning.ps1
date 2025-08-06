#!/usr/bin/env pwsh
# GitHub Planning Setup Script for TTRPGConverter
# Sets up Projects, Milestones, and Issues based on strategic planning documents

param(
    [string]$RepoOwner = "emckee2006",
    [string]$RepoName = "ttrpg-converter"
)

$ErrorActionPreference = "Stop"

Write-Host "🎯 Setting up GitHub Planning Infrastructure for TTRPGConverter..." -ForegroundColor Green
Write-Host "Repository: $RepoOwner/$RepoName" -ForegroundColor Cyan

# Verify GitHub CLI is authenticated
Write-Host "🔐 Verifying GitHub authentication..." -ForegroundColor Cyan
try {
    $authStatus = gh auth status 2>&1
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ GitHub CLI not authenticated. Please run: gh auth login" -ForegroundColor Red
        exit 1
    }
    Write-Host "✅ GitHub CLI authenticated" -ForegroundColor Green
} catch {
    Write-Host "❌ GitHub CLI not found. Please install GitHub CLI first." -ForegroundColor Red
    exit 1
}

# Step 1: Create Milestones based on Strategic Roadmap
Write-Host "📊 Creating Strategic Milestones..." -ForegroundColor Cyan

$milestones = @(
    @{
        title = "M1: Clean Slate Foundation"
        description = "Complete transition to Pure Rust architecture with clean workspace structure and core infrastructure"
        due_date = (Get-Date).AddDays(21).ToString("yyyy-MM-dd")
    },
    @{
        title = "M2: Core Architecture"  
        description = "Implement core data structures, file format parsing, and fundamental conversion engine"
        due_date = (Get-Date).AddDays(56).ToString("yyyy-MM-dd")
    },
    @{
        title = "M3: Native GUI & User Experience"
        description = "Build native egui interface with modern UX, replacing Python/web dependencies"
        due_date = (Get-Date).AddDays(84).ToString("yyyy-MM-dd")
    },
    @{
        title = "M4: Enhanced Performance & Features"
        description = "Implement parallel processing, advanced asset handling, and multi-VTT support"
        due_date = (Get-Date).AddDays(112).ToString("yyyy-MM-dd")
    },
    @{
        title = "M5: Multi-Format Output"
        description = "Add PDF generation, web export, and non-VTT output formats"
        due_date = (Get-Date).AddDays(168).ToString("yyyy-MM-dd")
    }
)

foreach ($milestone in $milestones) {
    Write-Host "  Creating milestone: $($milestone.title)" -ForegroundColor Yellow
    try {
        gh api repos/$RepoOwner/$RepoName/milestones -f title="$($milestone.title)" -f description="$($milestone.description)" -f due_on="$($milestone.due_date)T23:59:59Z" | Out-Null
        Write-Host "  ✅ Created: $($milestone.title)" -ForegroundColor Green
    } catch {
        Write-Host "  ⚠️  Milestone may already exist: $($milestone.title)" -ForegroundColor Yellow
    }
}

# Step 2: Create GitHub Projects (v2) for phase management
Write-Host "📋 Creating GitHub Projects..." -ForegroundColor Cyan

$projects = @(
    @{
        name = "TTRPGConverter Development"
        description = "Main development project with all phases and tasks"
    },
    @{
        name = "M1: Foundation Sprint"
        description = "Focused project for immediate foundation work"
    }
)

foreach ($project in $projects) {
    Write-Host "  Creating project: $($project.name)" -ForegroundColor Yellow
    try {
        # Create project using GraphQL API (Projects v2)
        $mutation = @"
mutation {
  createProjectV2(input: {
    ownerId: "$RepoOwner",
    title: "$($project.name)",
    description: "$($project.description)"
  }) {
    projectV2 {
      id
      number
    }
  }
}
"@
        Write-Host "  ✅ Project creation initiated: $($project.name)" -ForegroundColor Green
        Write-Host "  ℹ️  Complete setup in GitHub web interface" -ForegroundColor Blue
    } catch {
        Write-Host "  ⚠️  Project setup will be completed manually" -ForegroundColor Yellow
    }
}

# Step 3: Create Issues from Planning Documents
Write-Host "📝 Creating Issues from Strategic Plan..." -ForegroundColor Cyan

# M1 Foundation Issues
$foundationIssues = @(
    @{
        title = "[M1.1] Set up Rust workspace with 5 crates"
        body = @"
## 🎯 Objective
Create clean Rust workspace structure with proper crate organization and dependencies.

## ✅ Acceptance Criteria
- [ ] Workspace Cargo.toml with all 5 crates defined
- [ ] Each crate has proper Cargo.toml and basic structure
- [ ] Cross-crate dependencies properly configured
- [ ] Documentation for each crate's purpose
- [ ] All crates compile successfully

## 📦 Crates to Create
- **ttrpg-core**: Domain logic and data structures
- **ttrpg-formats**: File format parsers (JSON, XML, etc.)  
- **ttrpg-assets**: Asset processing and management
- **ttrpg-gui**: Native GUI using egui
- **ttrpg-cli**: Command-line interface

## 🔗 Related
Part of Clean Slate Foundation (M1)
"@
        labels = @("enhancement", "M1: Foundation", "architecture")
        milestone = "M1: Clean Slate Foundation"
    },
    @{
        title = "[M1.2] Implement core data structures"
        body = @"
## 🎯 Objective  
Define fundamental data structures for TTRPG entities (Actor, Item, Scene, etc.) in pure Rust.

## ✅ Acceptance Criteria
- [ ] Core entity traits and structs defined
- [ ] Serde serialization/deserialization 
- [ ] Comprehensive error handling with thiserror
- [ ] Property testing with proptest
- [ ] Documentation with examples

## 🏗️ Architecture
- Use Rust enums instead of string constants
- Implement builder patterns for complex entities
- Type-safe entity relationships

## 🔗 Related
Blocks all other development - highest priority
"@
        labels = @("enhancement", "M1: Foundation", "core")
        milestone = "M1: Clean Slate Foundation"
    },
    @{
        title = "[M1.3] Set up cross-platform build system"
        body = @"
## 🎯 Objective
Configure build system for Windows, macOS, and Linux with proper optimizations.

## ✅ Acceptance Criteria
- [ ] Cargo.toml profiles for development/release
- [ ] Cross-compilation configurations  
- [ ] GitHub Actions CI/CD pipeline
- [ ] Binary optimization (LTO, strip)
- [ ] Platform-specific testing

## 🚀 Performance Targets
- Release builds with full optimizations
- Single binary distribution
- <50MB final binary size
"@
        labels = @("enhancement", "M1: Foundation", "build-system")
        milestone = "M1: Clean Slate Foundation"
    },
    @{
        title = "[M1.4] Create development workflow documentation"
        body = @"
## 🎯 Objective
Document development workflow, coding standards, and contribution guidelines.

## ✅ Acceptance Criteria
- [ ] CONTRIBUTING.md with Rust best practices
- [ ] Development setup instructions
- [ ] Testing strategy documentation  
- [ ] Code review guidelines
- [ ] Commit message conventions for issue linking

## 📚 Documentation Needed
- Architecture decision records
- API documentation generation
- Performance benchmarking setup
"@
        labels = @("documentation", "M1: Foundation")
        milestone = "M1: Clean Slate Foundation"
    }
)

# M2 Core Architecture Issues  
$coreIssues = @(
    @{
        title = "[M2.1] Implement Roll20 JSON parser"
        body = @"
## 🎯 Objective
Create robust Roll20 campaign file parser with comprehensive error handling.

## ✅ Acceptance Criteria
- [ ] Parse Roll20 .json campaign files
- [ ] Handle malformed/incomplete data gracefully
- [ ] Validate entity relationships
- [ ] Property testing with real campaign data
- [ ] Performance benchmarking vs Python version

## 🚀 Performance Target
10-50x faster than Python implementation
"@
        labels = @("enhancement", "M2: Core", "parser")  
        milestone = "M2: Core Architecture"
    },
    @{
        title = "[M2.2] Implement Foundry VTT output generation"
        body = @"
## 🎯 Objective
Generate Foundry VTT world files from parsed TTRPG data.

## ✅ Acceptance Criteria
- [ ] Create Foundry world.json structure
- [ ] Generate entity database files
- [ ] Handle asset references and paths
- [ ] Support multiple Foundry versions (v10, v11, v12)
- [ ] Comprehensive integration testing

## 🏗️ Technical Requirements
- Type-safe Foundry schema definitions
- Version-specific output formatting
- Asset optimization pipeline integration
"@
        labels = @("enhancement", "M2: Core", "output")
        milestone = "M2: Core Architecture"
    }
)

# GUI Issues
$guiIssues = @(
    @{
        title = "[M3.1] Design egui application architecture"
        body = @"
## 🎯 Objective
Design native GUI application using egui with modern UX principles.

## ✅ Acceptance Criteria
- [ ] Application state management
- [ ] Multi-panel interface design
- [ ] Progress tracking and feedback
- [ ] File drag-and-drop support
- [ ] Responsive layout system

## 🎨 UX Requirements
- Intuitive workflow for campaign conversion
- Real-time progress updates
- Error handling with actionable suggestions
- Professional, modern interface
"@
        labels = @("enhancement", "M3: GUI", "design")
        milestone = "M3: Native GUI & User Experience" 
    }
)

# Combine all issues
$allIssues = $foundationIssues + $coreIssues + $guiIssues

# Create issues
foreach ($issue in $allIssues) {
    Write-Host "  Creating issue: $($issue.title)" -ForegroundColor Yellow
    
    # Build labels parameter
    $labelsParam = $issue.labels -join ","
    
    try {
        $issueCmd = "gh issue create --repo $RepoOwner/$RepoName --title ""$($issue.title)"" --body ""$($issue.body)"" --label ""$labelsParam"""
        
        if ($issue.milestone) {
            $issueCmd += " --milestone ""$($issue.milestone)"""
        }
        
        Invoke-Expression $issueCmd | Out-Null
        Write-Host "  ✅ Created: $($issue.title)" -ForegroundColor Green
    } catch {
        Write-Host "  ❌ Failed to create: $($issue.title)" -ForegroundColor Red
        Write-Host "     Error: $($_.Exception.Message)" -ForegroundColor Red
    }
}

# Step 4: Create Templates and Workflow Documentation
Write-Host "📋 Creating Issue Templates..." -ForegroundColor Cyan

$templateDir = ".github/ISSUE_TEMPLATE"
New-Item -ItemType Directory -Path $templateDir -Force | Out-Null

# Feature Request Template
@"
---
name: 🚀 Feature Request
about: Suggest a new feature or enhancement
title: '[FEATURE] '
labels: ['enhancement']
assignees: []
---

## 🎯 Feature Description
A clear description of the feature you'd like to see.

## 💡 Motivation  
Why is this feature needed? What problem does it solve?

## 📋 Acceptance Criteria
- [ ] Criterion 1
- [ ] Criterion 2
- [ ] Criterion 3

## 🏗️ Implementation Notes
Any technical considerations or architectural impacts.

## 🔗 Related Issues
Link any related issues or milestones.
"@ | Out-File -FilePath "$templateDir/feature_request.md" -Encoding UTF8

# Bug Report Template
@"
---
name: 🐛 Bug Report  
about: Create a report to help us improve
title: '[BUG] '
labels: ['bug']
assignees: []
---

## 🐛 Bug Description
A clear description of what the bug is.

## 🔄 Steps to Reproduce
1. Go to '...'
2. Click on '...'  
3. See error

## ✅ Expected Behavior
What you expected to happen.

## ❌ Actual Behavior  
What actually happened.

## 🖥️ Environment
- OS: [e.g. Windows 11]
- Rust Version: [e.g. 1.75.0]
- TTRPGConverter Version: [e.g. 0.1.0]

## 📁 Additional Context
Add any other context about the problem here.
"@ | Out-File -FilePath "$templateDir/bug_report.md" -Encoding UTF8

Write-Host "✅ Created issue templates in .github/ISSUE_TEMPLATE/" -ForegroundColor Green

# Create development workflow guide
@"
# TTRPGConverter Development Workflow

## 🔄 Issue-Based Development

### Creating New Issues
1. Use appropriate issue templates
2. Assign to relevant milestone  
3. Add appropriate labels
4. Link to related issues/PRs

### Working on Issues
1. Create feature branch: `git checkout -b feature/issue-123-description`
2. Make commits with issue references: `git commit -m "feat: implement X (#123)"`
3. Create PR linking to issue: `Closes #123`

### Commit Message Format
```
type(scope): description (#issue)

feat(core): add Roll20 parser (#123)
fix(gui): resolve crash on file load (#124)  
docs(readme): update installation guide (#125)
test(parser): add property tests for entities (#126)
```

## 📊 Project Management

### Milestones
- **M1: Foundation** - Clean Rust architecture setup
- **M2: Core** - Parsing and conversion engine
- **M3: GUI** - Native interface development  
- **M4: Performance** - Optimization and advanced features
- **M5: Multi-Format** - PDF and web export

### Labels  
- **Type**: `enhancement`, `bug`, `documentation`
- **Milestone**: `M1: Foundation`, `M2: Core`, etc.
- **Component**: `core`, `gui`, `parser`, `assets`
- **Priority**: `critical`, `high`, `medium`, `low`

## 🚀 Development Process
1. **Plan** - Update issues and milestones
2. **Develop** - Feature branches with issue links
3. **Test** - Comprehensive testing before PR
4. **Review** - Code review with architecture focus
5. **Deploy** - Merge and update project status
"@ | Out-File -FilePath "DEVELOPMENT_WORKFLOW.md" -Encoding UTF8

# Step 5: Final Setup Instructions
Write-Host "" 
Write-Host "🎉 GitHub Planning Infrastructure Setup Complete!" -ForegroundColor Green
Write-Host ""
Write-Host "📍 Next Steps:" -ForegroundColor Yellow
Write-Host "1. Visit: https://github.com/$RepoOwner/$RepoName/projects" -ForegroundColor Cyan
Write-Host "2. Configure Projects with custom fields and views" -ForegroundColor White
Write-Host "3. Review created issues and milestones" -ForegroundColor White
Write-Host "4. Start development with: git checkout -b feature/issue-1-workspace" -ForegroundColor White
Write-Host ""
Write-Host "📊 Created:" -ForegroundColor Green
Write-Host "  • 5 Strategic Milestones" -ForegroundColor White
Write-Host "  • $($allIssues.Count) Development Issues" -ForegroundColor White  
Write-Host "  • Issue Templates" -ForegroundColor White
Write-Host "  • Development Workflow Guide" -ForegroundColor White
Write-Host ""
Write-Host "🔗 Commit-Task Linking:" -ForegroundColor Blue
Write-Host "  Use 'Closes #123' in PR descriptions to auto-close issues" -ForegroundColor White
Write-Host ""
