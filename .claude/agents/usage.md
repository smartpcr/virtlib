# VirtLib Usage Patterns

## Basic Setup

```csharp
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;
using VirtLib.Windows;
using VirtLib.Windows.Definitions;

// Setup DI container
var services = new ServiceCollection();
services.AddLogging(builder => builder.AddConsole());
var provider = services.BuildServiceProvider();

// Connect to local Hyper-V host (requires admin)
using var hcs = new HostComputeSystem(provider);

// Or connect to remote host
using var hcs = new HostComputeSystem(provider, "HYPERV-SERVER");
```

## Create a Virtual Machine

### Gen2 VM with Dynamic Memory

```csharp
var vmDef = new VirtualMachineDefinition("MyVM", @"C:\Hyper-V\MyVM")
{
    Generation = Generation.Gen2,
    Notes = "Created via VirtLib",

    Processor = new ProcessorDefinition
    {
        Quantity = 4,
        Reservation = 0,
        Limit = 100,
        Weight = 100
    },

    Memory = new MemoryDefinition
    {
        Startup = 4096,    // MB
        Minimum = 2048,    // Enables dynamic memory
        Maximum = 8192,
        Weight = MemoryWeight.Medium
    },

    Security = new SecurityDefinition
    {
        SecureBoot = true,
        SecureBootTemplate = SecureBootTemplates.MicrosoftWindows
    }
};

// Add SCSI controller with VHD
vmDef.ScsiControllers[0].Drives.Add(new VirtualHardDrive
{
    Path = @"C:\Hyper-V\MyVM\disk.vhdx",
    SizeInBytes = 100L * 1024 * 1024 * 1024  // 100 GB
});

// Configure network
vmDef.NetworkAdapters[0].SwitchName = "Default Switch";

var vm = hcs.CreateVirtualMachine(vmDef);
```

### Gen1 VM with Static Memory

```csharp
var vmDef = new VirtualMachineDefinition("LegacyVM", @"C:\Hyper-V\LegacyVM")
{
    Generation = Generation.Gen1,

    Processor = new ProcessorDefinition { Quantity = 2 },

    // Static memory (no min/max)
    Memory = new MemoryDefinition { Startup = 2048 }
};

var vm = hcs.CreateVirtualMachine(vmDef);
```

## Query Virtual Machines

```csharp
// Get single VM by name
var vm = hcs.GetVirtualMachine("MyVM");
if (vm != null)
{
    Console.WriteLine($"State: {vm.State}");
    Console.WriteLine($"Generation: {vm.Generation}");
}

// Check if VM exists
bool exists = hcs.IsVirtualMachineExist("MyVM");
```

## Configure Storage

### Add SCSI Controller and Disk

```csharp
// In VM definition
vmDef.ScsiControllers.Add(new ScsiController());
vmDef.ScsiControllers[1].Drives.Add(new VirtualHardDrive
{
    Path = @"C:\Hyper-V\MyVM\data.vhdx",
    SizeInBytes = 500L * 1024 * 1024 * 1024
});
```

### Add DVD Drive with ISO

```csharp
vmDef.ScsiControllers[0].Drives.Add(new VirtualDvdDrive
{
    Path = @"C:\ISO\ubuntu.iso"
});
```

## Configure Networking

### Multiple NICs

```csharp
// First NIC (default)
vmDef.NetworkAdapters[0].SwitchName = "External Switch";
vmDef.NetworkAdapters[0].MacAddress = MacAddress.Dynamic;

// Add second NIC
vmDef.NetworkAdapters.Add(new NetworkAdapter
{
    SwitchName = "Internal Switch",
    MacAddress = new MacAddress("00-15-5D-01-02-03")
});
```

## Configure Checkpoints

```csharp
vmDef.Checkpoints = new CheckpointsDefinition
{
    Type = CheckpointType.Production,
    Fallback = true,  // Fall back to standard if production fails
    Path = @"C:\Hyper-V\MyVM\Checkpoints"
};
```

## Configure Auto-Start/Stop

```csharp
vmDef.AutomaticStart = new AutomaticStartDefinition
{
    Action = AutomaticStartAction.StartIfRunning,
    Delay = 30  // seconds
};

vmDef.AutomaticStop = new AutomaticStopDefinition
{
    Action = AutomaticStopAction.Save
};
```

## Delete Virtual Machine

```csharp
hcs.DeleteVirtualMachine("MyVM");
```

## Integration Services

```csharp
// Enable/disable via IntegrationServicesDefinition
vmDef.IntegrationServices = new IntegrationServicesDefinition
{
    Heartbeat = true,
    TimeSync = true,
    Shutdown = true,
    VSS = true,
    DataExchange = true,
    GuestServices = false
};
```

## Virtual Switch Operations

```csharp
// Query switches
var switches = hcs.GetVirtualSwitches();
foreach (var sw in switches)
{
    Console.WriteLine($"{sw.Name}: {sw.SwitchType}");
}

// Get specific switch
var defaultSwitch = hcs.GetVirtualSwitch("Default Switch");
```

## Error Handling

```csharp
try
{
    var vm = hcs.CreateVirtualMachine(vmDef);
}
catch (UnauthorizedAccessException ex)
{
    // Not running as administrator
    Console.WriteLine("Admin rights required");
}
catch (InvalidOperationException ex)
{
    // VM already exists, switch not found, etc.
    Console.WriteLine(ex.Message);
}
```

## SpecFlow Test Examples

Feature files in `VirtLib.Windows.Tests/Features/` demonstrate usage:

```gherkin
Scenario: Create gen-2 virtual machine
    Given hyper-v host running on localhost
    And ubuntu image "ubuntu2404.iso" is at "X:\iso\ubuntu2404.iso"
    When I create gen 2 vm with name "ubuntu3"
      | Name             | Value          |
      | CpuCount         | 2              |
      | MemoryInMB       | 2048           |
      | HardDiskSizeInGB | 20             |
      | SwitchName       | Default Switch |
    Then vm should exist with name "ubuntu3"
    And delete vm with name "ubuntu3"
```
