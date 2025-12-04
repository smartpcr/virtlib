# VirtLib Design Patterns

## Partial Class Organization

The `HostComputeSystem` class is the central API for Hyper-V management. It's split across multiple partial class files by domain:

```
HostComputeSystem.cs           # Core: DI, WMI scopes, management services
HostComputeSystem.create.cs    # CreateVirtualMachine(), settings builders
HostComputeSystem.delete.cs    # DeleteVirtualMachine()
HostComputeSystem.get.cs       # GetVirtualMachine(), list operations
HostComputeSystem.storage.cs   # SCSI controllers, VHD, DVD drives
HostComputeSystem.network.cs   # Network adapters, virtual switches
HostComputeSystem.security.cs  # Secure boot, shielding, TPM
HostComputeSystem.validation.cs # Pre-flight checks
HostComputeSystem.private.cs   # Internal helpers

# Integration Services (each has enable/disable methods):
HostComputeSystem.heartbeat.cs
HostComputeSystem.time.cs
HostComputeSystem.shutdown.cs
HostComputeSystem.vss.cs
HostComputeSystem.dataexchange.cs
HostComputeSystem.guestservice.cs
```

## WMI Interaction Pattern

All Hyper-V operations use WMI via `System.Management`:

```csharp
// 1. Get management service singleton
using var vmms = GetVirtualMachineManagementService();

// 2. Prepare method parameters
using var inputParameters = vmms.GetMethodParameters("MethodName");
inputParameters["ParamName"] = value;

// 3. Invoke WMI method
using var output = vmms.InvokeMethod("MethodName", inputParameters, null);

// 4. Validate job output (handles async WMI jobs)
JobOutputHelper.ValidateOutput(output, _logger);
```

## Resource Creation Pattern

Resources (processors, memory, disks, NICs) follow a consistent pattern:

```csharp
// 1. Create default resource from template
var resource = ResourceQueries.CreateDefaultResource(ResourceSubtype.Processor, scope);

// 2. Configure properties
resource["ElementName"] = "VM Name Processor";
resource["VirtualQuantity"] = 4;

// 3. Serialize to CIM DTD format for WMI
var resourceText = resource.GetText(TextFormat.CimDtd20);
```

## Definition Classes (Fluent Configuration)

VM configuration uses a builder-style definition pattern in `Definitions/`:

```csharp
var vmDef = new VirtualMachineDefinition("MyVM", @"C:\VMs\MyVM")
{
    Generation = Generation.Gen2,
    Processor = new ProcessorDefinition { Quantity = 4 },
    Memory = new MemoryDefinition { Startup = 4096, Minimum = 2048, Maximum = 8192 },
    Security = new SecurityDefinition { SecureBoot = true }
};

// Collections auto-initialize with defaults
vmDef.ScsiControllers.Add(new ScsiController());
vmDef.NetworkAdapters[0].SwitchName = "Default Switch";
```

## Model Wrappers

`Models/` contains typed wrappers around WMI `ManagementObject` instances:

```csharp
public class VirtualMachine
{
    private readonly ManagementObject _instance;

    public string Name => _instance["ElementName"]?.ToString();
    public EnabledState State => (EnabledState)(ushort)_instance["EnabledState"];
}
```

## Query Helpers

`Queries/` provides WMI class names and query builders:

```csharp
// VMQueries - Virtual machine related WMI classes
VMQueries.GetVMSettingWmiClass(VMSetting.System)  // "Msvm_VirtualSystemSettingData"

// ResourceQueries - Resource allocation queries
ResourceQueries.CreateDefaultResource(ResourceSubtype.Memory, scope)

// VSwitchQueries - Virtual switch queries
VSwitchQueries.GetSwitchByName(scope, "Default Switch")
```

## Error Handling

WMI operations return job objects for async operations:

```csharp
// JobOutputHelper handles:
// - Synchronous success (ReturnValue == 0)
// - Async job completion (ReturnValue == 4096)
// - Error extraction from WMI job objects
JobOutputHelper.ValidateOutput(output, _logger);
```

## Dependency Injection

The library uses Microsoft.Extensions.DependencyInjection:

```csharp
var services = new ServiceCollection();
services.AddLogging(builder => builder.AddConsole());
var provider = services.BuildServiceProvider();

using var hcs = new HostComputeSystem(provider);
// or for remote host:
using var hcs = new HostComputeSystem(provider, "HYPERV-HOST");
```

## Generation Differences

Gen1 vs Gen2 VMs have different capabilities:

| Feature | Gen1 | Gen2 |
|---------|------|------|
| Boot | BIOS | UEFI |
| Secure Boot | No | Yes |
| Boot from SCSI | No | Yes |
| IDE Controllers | Yes | No |
| VHDX only | No | Yes |

Code handles this via `Generation` enum and conditional logic in `HostComputeSystem.create.cs`.
