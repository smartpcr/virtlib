// -----------------------------------------------------------------------
// <copyright file="HostComputeSystem.create.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows;

using System;
using System.Linq;
using System.Management;
using Definitions;
using Models;
using Queries;

public partial class HostComputeSystem
{
    public VirtualMachine CreateVirtualMachine(VirtualMachineDefinition vm)
    {
        if (IsVirtualMachineExist(vm.Name))
        {
            throw new InvalidOperationException($"Virtual machine {vm.Name} already exists.");
        }

        ValidateHardDisksNotExist(vm);
        ValidateVirtualSwitchExists(vm);

        using var inputParameters = this.Vmms.GetMethodParameters("DefineSystem");
        using var systemSettings = CreateSystemSettings(vm);
        inputParameters["SystemSettings"] = systemSettings.GetText(TextFormat.CimDtd20);
        using var processorSettings = CreateProcessorSettings(vm);
        using var memorySettings = CreateMemorySettings(vm);
        inputParameters["ResourceSettings"] = new[]
        {
            processorSettings.GetText(TextFormat.CimDtd20),
            memorySettings.GetText(TextFormat.CimDtd20)
        };
        using var output = this.Vmms.InvokeMethod("DefineSystem", inputParameters, null);

        JobOutputHelper.ValidateOutput(output, this._logger);
        using var virtualMachine = new ManagementObject(output["ResultingSystem"].ToString());
        using var newSystemSettings = virtualMachine.GetRelated(VMQueries.GetVMSettingWmiClass(VMSetting.System)).OfType<ManagementObject>().First();

        UpdateScsiControllers(vm, newSystemSettings);
        UpdateNetworkAdapters(vm, newSystemSettings);
        UpdateSecuritySettings(vm, newSystemSettings);
        UpdateShutdownSetting(vm, newSystemSettings);
        UpdateTimeSyncSetting(vm, newSystemSettings);
        UpdateDataExchangeSetting(vm, newSystemSettings);
        UpdateHeartbeatSetting(vm, newSystemSettings);
        UpdateVolumeShadownCopySetting(vm, newSystemSettings);
        UpdateGuestServiceSetting(vm, newSystemSettings);

        var vmResponse = new VirtualMachine(this._serviceProvider, virtualMachine);
        return vmResponse;
    }

    private static bool IsDynamicMemoryEnabled(VirtualMachineDefinition vm)
    {
        var enableDynamicMemory =
            vm.Memory.Minimum > 0 &&
            vm.Memory.Maximum > 0 &&
            vm.Memory.Minimum <= vm.Memory.Startup &&
            vm.Memory.Maximum >= vm.Memory.Startup;
        return enableDynamicMemory;
    }

    private ManagementObject CreateSystemSettings(VirtualMachineDefinition vm)
    {
        using var managementClass = new ManagementClass(VMQueries.GetVMSettingWmiClass(VMSetting.System));
        managementClass.Scope = this.VirtualizationScope;
        var systemSettings = managementClass.CreateInstance();
        systemSettings["ElementName"] = vm.Name;
        systemSettings["ConfigurationDataRoot"] = vm.Path;
        systemSettings["VirtualSystemSubtype"] = vm.Generation == Generation.Gen1
            ? "Microsoft:Hyper-V:SubType:1"
            : "Microsoft:Hyper-V:SubType:2";
        systemSettings["VirtualNumaEnabled"] = !IsDynamicMemoryEnabled(vm);
        systemSettings["Notes"] = new[] { vm.Notes };

        // secure boot
        systemSettings["SecureBootEnabled"] = vm.Security.SecureBoot;
        if (vm.Security.SecureBoot)
        {
            systemSettings["SecureBootTemplateId"] = vm.Security.SecureBootTemplate;
        }

        // snapshots
        systemSettings["SnapshotDataRoot"] = vm.Checkpoints.Path;
        switch (vm.Checkpoints.Type)
        {
            case CheckpointType.None:
                systemSettings["UserSnapshotType"] = (ushort)2; // Disabled
                break;
            case CheckpointType.Production:
                if (vm.Checkpoints.Fallback)
                {
                    systemSettings["UserSnapshotType"] = (ushort)3; // ProductionFallbackToTest
                }
                else
                {
                    systemSettings["UserSnapshotType"] = (ushort)4; // ProductionNoFallback
                }

                break;
            case CheckpointType.Standard:
                systemSettings["UserSnapshotType"] = (ushort)5; // Test
                break;
        }

        // auto-start/stop
        systemSettings["AutomaticStartupAction"] = vm.AutomaticStart.Action;
        systemSettings["AutomaticShutdownAction"] = vm.AutomaticStop.Action;
        if (vm.AutomaticStart.Action != AutomaticStartAction.Nothing)
        {
            // Startup Delay
            systemSettings["AutomaticStartupActionDelay"] = ManagementDateTimeConverter.ToDmtfTimeInterval(TimeSpan.FromSeconds(vm.AutomaticStart.Delay));
        }

        // swap
        systemSettings["SwapFileDataRoot"] = vm.SmartPaging.Path;

        var propValues = systemSettings.GetPropertyValues();
        this._logger.LogSystemSettingDefinition(propValues);
        return systemSettings;
    }

    private ManagementObject CreateProcessorSettings(VirtualMachineDefinition vm)
    {
        var processor = ResourceQueries.CreateDefaultResource(ResourceSubtype.Processor, this.VirtualizationScope);
        processor["ElementName"] = vm.Name + " Processor";
        processor["VirtualQuantity"] = vm.Processor.Quantity;
        processor["Reservation"] = vm.Processor.Reservation;
        processor["Limit"] = vm.Processor.Limit;
        processor["Weight"] = vm.Processor.Weight;
        processor["LimitProcessorFeatures"] = vm.Processor.LimitFeatures;
        processor["MaxProcessorsPerNumaNode"] = vm.Processor.MaxProcessorsPerNumaNode;
        processor["MaxNumaNodesPerSocket"] = vm.Processor.MaxNumaNodesPerSocket;
        processor["HwThreadsPerCore"] = vm.Processor.HardwareThreadsPerCore;

        var propValues = processor.GetPropertyValues();
        this._logger.LogProcessorDefinition(propValues);
        return processor;
    }

    private ManagementObject CreateMemorySettings(VirtualMachineDefinition vm)
    {
        var memory = ResourceQueries.CreateDefaultResource(ResourceSubtype.Memory, this.VirtualizationScope);
        memory["ElementName"] = vm.Name + " Memory";
        memory["VirtualQuantity"] = vm.Memory.Startup;
        memory["Weight"] = vm.Memory.Weight;
        memory["MaxMemoryBlocksPerNumaNode"] = vm.Memory.NumaMemoryPerNode;
        bool enableDynamicMemory = IsDynamicMemoryEnabled(vm);
        memory["DynamicMemoryEnabled"] = enableDynamicMemory;
        if (enableDynamicMemory)
        {
            memory["Reservation"] = vm.Memory.Minimum;
            memory["Limit"] = vm.Memory.Maximum;
        }

        var propValues = memory.GetPropertyValues();
        this._logger.LogMemoryDefinition(propValues);
        return memory;
    }
}