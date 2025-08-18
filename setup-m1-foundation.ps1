#!/usr/bin/env pwsh
# M1 Foundation Setup Script - TTRPGConverter C#
# Completes project structure, dependencies, and references

Write-Host "🚀 M1 Foundation Setup - TTRPGConverter C#" -ForegroundColor Green
Write-Host "===============================================" -ForegroundColor Green

# Add all projects to solution
Write-Host "📁 Adding projects to solution..." -ForegroundColor Yellow
dotnet sln TTRPGConverter.sln add src\TTRPGConverter.Processing\TTRPGConverter.Processing.csproj
dotnet sln TTRPGConverter.sln add src\TTRPGConverter.CLI\TTRPGConverter.CLI.csproj  
dotnet sln TTRPGConverter.sln add src\TTRPGConverter.GUI\TTRPGConverter.GUI.csproj
dotnet sln TTRPGConverter.sln add tests\TTRPGConverter.Core.Tests\TTRPGConverter.Core.Tests.csproj
dotnet sln TTRPGConverter.sln add tests\TTRPGConverter.Processing.Tests\TTRPGConverter.Processing.Tests.csproj

# Setup project references
Write-Host "🔗 Setting up project references..." -ForegroundColor Yellow
# Processing depends on Core
dotnet add src\TTRPGConverter.Processing\TTRPGConverter.Processing.csproj reference src\TTRPGConverter.Core\TTRPGConverter.Core.csproj
# CLI depends on Core and Processing  
dotnet add src\TTRPGConverter.CLI\TTRPGConverter.CLI.csproj reference src\TTRPGConverter.Core\TTRPGConverter.Core.csproj
dotnet add src\TTRPGConverter.CLI\TTRPGConverter.CLI.csproj reference src\TTRPGConverter.Processing\TTRPGConverter.Processing.csproj
# GUI depends on Core and Processing
dotnet add src\TTRPGConverter.GUI\TTRPGConverter.GUI.csproj reference src\TTRPGConverter.Core\TTRPGConverter.Core.csproj
dotnet add src\TTRPGConverter.GUI\TTRPGConverter.GUI.csproj reference src\TTRPGConverter.Processing\TTRPGConverter.Processing.csproj
# Test projects reference their counterparts
dotnet add tests\TTRPGConverter.Core.Tests\TTRPGConverter.Core.Tests.csproj reference src\TTRPGConverter.Core\TTRPGConverter.Core.csproj
dotnet add tests\TTRPGConverter.Processing.Tests\TTRPGConverter.Processing.Tests.csproj reference src\TTRPGConverter.Processing\TTRPGConverter.Processing.csproj

# Add essential NuGet packages to Core
Write-Host "📦 Adding NuGet packages to Core..." -ForegroundColor Yellow
dotnet add src\TTRPGConverter.Core\TTRPGConverter.Core.csproj package System.Text.Json --version 9.0.0
dotnet add src\TTRPGConverter.Core\TTRPGConverter.Core.csproj package NJsonSchema --version 11.1.0
dotnet add src\TTRPGConverter.Core\TTRPGConverter.Core.csproj package NJsonSchema.CodeGeneration.CSharp --version 11.1.0
dotnet add src\TTRPGConverter.Core\TTRPGConverter.Core.csproj package Microsoft.Extensions.DependencyInjection --version 9.0.0
dotnet add src\TTRPGConverter.Core\TTRPGConverter.Core.csproj package Microsoft.Extensions.Logging --version 9.0.0

# Add packages to Processing
Write-Host "📦 Adding NuGet packages to Processing..." -ForegroundColor Yellow
dotnet add src\TTRPGConverter.Processing\TTRPGConverter.Processing.csproj package SixLabors.ImageSharp --version 3.1.5
dotnet add src\TTRPGConverter.Processing\TTRPGConverter.Processing.csproj package System.IO.Compression --version 4.3.0
dotnet add src\TTRPGConverter.Processing\TTRPGConverter.Processing.csproj package Microsoft.Extensions.DependencyInjection --version 9.0.0

# Add packages to CLI
Write-Host "📦 Adding NuGet packages to CLI..." -ForegroundColor Yellow
dotnet add src\TTRPGConverter.CLI\TTRPGConverter.CLI.csproj package System.CommandLine --version 2.0.0-beta4.22272.1
dotnet add src\TTRPGConverter.CLI\TTRPGConverter.CLI.csproj package Microsoft.Extensions.Hosting --version 9.0.0
dotnet add src\TTRPGConverter.CLI\TTRPGConverter.CLI.csproj package Microsoft.Extensions.DependencyInjection --version 9.0.0

# Add testing packages
Write-Host "📦 Adding testing packages..." -ForegroundColor Yellow
dotnet add tests\TTRPGConverter.Core.Tests\TTRPGConverter.Core.Tests.csproj package FluentAssertions --version 6.12.2
dotnet add tests\TTRPGConverter.Processing.Tests\TTRPGConverter.Processing.Tests.csproj package FluentAssertions --version 6.12.2

# Build solution to verify everything works
Write-Host "🔨 Building solution..." -ForegroundColor Yellow
dotnet build TTRPGConverter.sln

# Run tests to verify setup
Write-Host "🧪 Running initial tests..." -ForegroundColor Yellow
dotnet test TTRPGConverter.sln

Write-Host "✅ M1 Foundation setup complete!" -ForegroundColor Green
Write-Host "📋 Next steps:" -ForegroundColor Cyan
Write-Host "  • Generate type-safe models from JSON schemas" -ForegroundColor White
Write-Host "  • Setup dependency injection containers" -ForegroundColor White
Write-Host "  • Create base entity classes and interfaces" -ForegroundColor White
