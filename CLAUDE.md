# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

```bash
# Restore dependencies
dotnet restore

# Build the solution
dotnet build

# Build in Release mode
dotnet build --configuration Release

# Run all unit tests
dotnet test --filter Category=unit_test

# Run a specific test by name
dotnet test --filter "FullyQualifiedName~TestMethodName"

# Run integration tests (requires Hyper-V host)
dotnet test --filter Category=integration_test
```

## Project Overview

VirtLib is a .NET 8.0 library for managing Hyper-V virtual machines on Windows using WMI (Windows Management Instrumentation). It provides a managed API for VM lifecycle operations including creation, configuration, and deletion.

## Architecture

### Core Components

- **VirtLib.Windows**: Main library targeting `net8.0-windows` with Windows-specific APIs
- **VirtLib.Windows.Tests**: SpecFlow-based integration tests using xUnit
- **Tests/Generation2VM**: Test project for Gen2 VM scenarios

### Key Classes

**HostComputeSystem** (`VirtLib.Windows/HostComputeSystem*.cs`): The central class for Hyper-V management, split across partial class files by functionality:
- `.cs` - Core initialization, WMI scope setup, management services
- `.create.cs` - VM creation logic
- `.delete.cs` - VM deletion
- `.get.cs` - VM queries
- `.storage.cs` - SCSI controllers, hard disks, DVD drives
- `.network.cs` - Network adapter configuration
- `.security.cs` - Security settings (Secure Boot, shielding)
- `.heartbeat.cs`, `.time.cs`, `.shutdown.cs`, `.vss.cs`, `.dataexchange.cs`, `.guestservice.cs` - Integration services

**VirtualMachineDefinition** (`VirtLib.Windows/Definitions/`): Configuration model for new VMs including:
- Processor, Memory, SCSI Controllers, Network Adapters
- Security (Secure Boot), Checkpoints, Auto-start/stop
- Supports Gen1 and Gen2 VMs

**Models** (`VirtLib.Windows/Models/`): WMI wrapper types for Hyper-V objects (VirtualMachine, SwitchInfo, EthernetPort, etc.)

**Queries** (`VirtLib.Windows/Queries/`): WMI query constants and helpers (VMQueries, VSwitchQueries, ResourceQueries)

### WMI Namespaces

The library interacts with:
- `root\virtualization\v2` - Primary Hyper-V WMI namespace
- `root\Microsoft\Windows\Hgs` - Host Guardian Service for shielded VMs

## Testing

Tests use SpecFlow BDD framework with feature files in `VirtLib.Windows.Tests/Features/`:
- `VirtualMachine.feature` - VM creation and management scenarios
- `VSwitch.feature` - Virtual switch operations
- `ResourceCapability.feature` - Resource validation
- `SanityCheck.feature` - Basic connectivity tests

Test categories:
- `unit_test` - Run in CI without Hyper-V
- `integration_test` - Require Hyper-V host access

## Configuration

- **Directory.Build.props**: Shared MSBuild properties, C# 10, nullable enabled, .NET 8.0
- **Packages.props**: Centralized package version management
- **stylecop.ruleset**: Code analysis rules

## Requirements

- Windows with Hyper-V role enabled
- Administrator privileges required (checked at runtime)
- .NET SDK 8.0
