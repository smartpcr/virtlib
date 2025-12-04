# VirtLib Context Memory

## Project Identity

- **Name**: VirtLib (Microsoft.AzureStack.Virtlib)
- **Purpose**: .NET library for Hyper-V virtual machine management via WMI
- **Platform**: Windows-only (`net8.0-windows`)
- **Namespace Root**: `VirtLib.Windows`

## Key Files to Reference

| Purpose | File(s) |
|---------|---------|
| VM creation entry point | `src/VirtLib.Windows/HostComputeSystem.create.cs` |
| VM definition model | `src/VirtLib.Windows/Definitions/VirtualMachineDefinition.cs` |
| WMI query helpers | `src/VirtLib.Windows/Queries/VMQueries.cs`, `ResourceQueries.cs` |
| Job output handling | `src/VirtLib.Windows/JobOutputHelper.cs` |
| Test scenarios | `src/VirtLib.Windows.Tests/Features/*.feature` |
| Build config | `Directory.Build.props`, `Packages.props` |

## WMI Classes Used

| WMI Class | Purpose |
|-----------|---------|
| `Msvm_VirtualSystemManagementService` | VM lifecycle operations |
| `Msvm_SecurityService` | Security/shielding operations |
| `Msvm_ImageManagementService` | VHD operations |
| `Msvm_VirtualSystemSettingData` | VM settings |
| `Msvm_ProcessorSettingData` | CPU configuration |
| `Msvm_MemorySettingData` | Memory configuration |
| `Msvm_SyntheticEthernetPortSettingData` | NIC configuration |
| `Msvm_VirtualEthernetSwitch` | Virtual switches |

## Common WMI Return Values

- `0` - Success (synchronous)
- `4096` - Job started (async, check job object)
- Other values indicate errors

## Resource Subtypes (for ResourceQueries)

```
Processor, Memory, SCSIController, IDEController,
VirtualHardDisk, VirtualDVD, EthernetAdapter
```

## Generation-Specific Behavior

**Gen1**:
- Uses IDE controllers for boot
- BIOS boot
- VHD or VHDX support
- No Secure Boot

**Gen2**:
- SCSI-only (no IDE)
- UEFI boot with Secure Boot option
- VHDX only
- TPM support for shielded VMs

## Test Categories

- `unit_test` - CI-safe, no Hyper-V required
- `integration_test` - Requires Hyper-V host
- `@vm` - Virtual machine tests
- `@vswitch` - Virtual switch tests

## Dependencies to Note

- `System.Management` - WMI access
- `Newtonsoft.Json` - Serialization
- `Microsoft.Extensions.DependencyInjection` - DI
- `Microsoft.Extensions.Logging` - Logging
- `SpecFlow.xUnit` - BDD testing

## Common Modification Points

When adding new VM features:
1. Add definition class in `Definitions/`
2. Add property to `VirtualMachineDefinition`
3. Add partial file `HostComputeSystem.{feature}.cs`
4. Add WMI query helpers if needed in `Queries/`
5. Add model wrapper in `Models/` if returning data

When adding new integration service:
1. Create `HostComputeSystem.{service}.cs`
2. Add enable/disable methods
3. Add property to `IntegrationServicesDefinition`
4. Wire up in `HostComputeSystem.create.cs`

## Gotchas

- Must run as Administrator
- WMI paths use backslashes and are case-sensitive
- Job objects must be waited on for async operations
- ManagementObject instances must be disposed
- Remote host requires WMI permissions configured
