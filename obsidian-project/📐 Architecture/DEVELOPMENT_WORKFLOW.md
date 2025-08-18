# TTRPGConverter Development Workflow

## 🔄 Issue-Based Development

### Creating New Issues
1. Use appropriate issue templates
2. Assign to relevant milestone  
3. Add appropriate labels
4. Link to related issues/PRs

### Working on Issues
1. Create feature branch: git checkout -b feature/issue-123-description
2. Make commits with issue references: git commit -m "feat: implement X (#123)"
3. Create PR linking to issue: Closes #123

### Commit Message Format
`
type(scope): description (#issue)

feat(core): add Roll20 parser (#123)
fix(gui): resolve crash on file load (#124)  
docs(readme): update installation guide (#125)
test(parser): add property tests for entities (#126)
`

## 📊 Project Management

### Milestones
- **M1: Foundation** - Clean Rust architecture setup
- **M2: Core** - Parsing and conversion engine
- **M3: GUI** - Native interface development  
- **M4: Performance** - Optimization and advanced features
- **M5: Multi-Format** - PDF and web export

### Labels  
- **Type**: nhancement, ug, documentation
- **Milestone**: M1: Foundation, M2: Core, etc.
- **Component**: core, gui, parser, ssets
- **Priority**: critical, high, medium, low

## 🚀 Development Process
1. **Plan** - Update issues and milestones
2. **Develop** - Feature branches with issue links
3. **Test** - Comprehensive testing before PR
4. **Review** - Code review with architecture focus
5. **Deploy** - Merge and update project status
