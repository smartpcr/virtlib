// -----------------------------------------------------------------------
// <copyright file="VirtualMachine.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

using System;
using System.Collections.Generic;
using System.Linq;
using System.Management;
using Queries;

public class VirtualMachine
{
    public string Name { get; set; }
    public Guid HyperVInstanceId { get; set; }
    public EnabledDefault EnabledDefault { get; set; }
    public EnabledState EnabledState { get; set; }
    public EnhancedSessionModeState SessionModeState { get; set; }
    public FailedOverReplicationType ReplicationType { get; set; }
    public HealthState HealthState { get; set; }
    public RequestedState RequestedState { get; set; }
    public string Description { get; set; }
    public int ThreadsPerCore { get; set; }
    public int NumaNodes { get; set; }
    public ReplicationHealth ReplicationHealth { get; set; }
    public ReplicationMode ReplicationMode { get; set; }
    public ReplicationState ReplicationState { get; set; }
    public ResetCapability ResetCapability { get; set; } = ResetCapability.Other;
    public DateTime? CreationTime { get; set; }
    public DateTime? ModificationTime { get; set; }
    public DateTime? LastStateChangeTime { get; set; }
    public List<BootDevice> BootEntries { get; set; } = new List<BootDevice>();
    public List<ResourceAllocation> Allocations { get; set; } = new List<ResourceAllocation>();

    public VirtualMachine(ManagementObject vmObj)
    {
        HcsLogger.LogManagementObject(vmObj);
        Name = vmObj["ElementName"]?.ToString() ?? string.Empty;
        HyperVInstanceId = Guid.Parse(vmObj["Name"]?.ToString() ?? Guid.Empty.ToString());
        EnabledDefault = vmObj["EnabledDefault"].ReadEnabledDefault();
        EnabledState = vmObj["EnabledState"].ReadEnabledState();
        SessionModeState = vmObj["EnhancedSessionModeState"].ReadEnhancedSessionModeState();
        ReplicationType = vmObj["ReplicationType"].ReadFailedOverReplicationType();
        HealthState = vmObj["HealthState"].ReadHealthState();
        RequestedState = vmObj["RequestedState"].ReadRequestedState();
        Description = vmObj["Description"]?.ToString() ?? string.Empty;
        ThreadsPerCore = vmObj["HwThreadsPerCoreRealized"] == null ? 0 : Convert.ToInt32(vmObj["HwThreadsPerCoreRealized"]);
        NumaNodes = vmObj["NumberOfNumaNodes"] == null ? 0 : Convert.ToInt32(vmObj["NumberOfNumaNodes"]);
        ReplicationHealth = vmObj["ReplicationHealth"].ReadReplicationHealth();
        ReplicationMode = vmObj["ReplicationMode"].ReadReplicationMode();
        ReplicationState = vmObj["ReplicationState"].ReadReplicationState();
        CreationTime = vmObj["InstallDate"].ReadDateTime();
        ModificationTime = vmObj["TimeOfLastConfigurationChange"].ReadDateTime();
        LastStateChangeTime = vmObj["TimeOfLastStateChange"].ReadDateTime();

        using var systemCollection = vmObj.GetRelated(VMQueries.RelatedSettings.System);
        using var systemSettings = systemCollection.OfType<ManagementObject>().FirstOrDefault();
        if (systemSettings != null)
        {
            HcsLogger.LogManagementObject(systemSettings);

            using var processorCollection = systemSettings.GetRelated(VMQueries.RelatedSettings.Processor);
            using var processorSettings = processorCollection.OfType<ManagementObject>().FirstOrDefault();
            if (processorSettings != null)
            {
                HcsLogger.LogManagementObject(processorSettings);
            }

            using var memoryCollection = systemSettings.GetRelated(VMQueries.RelatedSettings.Memory);
            using var memorySettings = memoryCollection.OfType<ManagementObject>().FirstOrDefault();
            if (memorySettings != null)
            {
                HcsLogger.LogManagementObject(memorySettings);
            }

            if (systemSettings["BootSourceOrder"] is string[] bootEntries)
            {
                foreach (var bootEntry in bootEntries)
                {
                    var bootDevice = new BootDevice(bootEntry);
                    BootEntries.Add(bootDevice);
                }
            }

            using var storageCollection = systemSettings.GetRelated(VMQueries.RelatedSettings.Storage);
            using var storageSettings = storageCollection.OfType<ManagementObject>().FirstOrDefault();
            if (storageSettings != null)
            {
                HcsLogger.LogManagementObject(storageSettings);
            }

            using var virtualHardDiskCollection = systemSettings.GetRelated(VMQueries.RelatedSettings.VirtualHardDisk);
            foreach (var virtualHardDisk in virtualHardDiskCollection)
            {
                using (virtualHardDisk)
                {
                    if (virtualHardDisk is ManagementObject virtualHardDiskSettings)
                    {
                        HcsLogger.LogManagementObject(virtualHardDiskSettings);
                    }
                }
            }

            using var syntheticPortCollection = systemSettings.GetRelated(VMQueries.RelatedSettings.SyntheticEthernetPort);
            foreach (var syntheticPort in syntheticPortCollection)
            {
                using (syntheticPort)
                {
                    if (syntheticPort is ManagementObject syntheticPortObj)
                    {
                        HcsLogger.LogManagementObject(syntheticPortObj);
                    }
                }
            }

            using var emulatedPortCollection = systemSettings.GetRelated(VMQueries.RelatedSettings.EmulatedEthernetPort);
            foreach (var emulatedPort in emulatedPortCollection)
            {
                using (emulatedPort)
                {
                    if (emulatedPort is ManagementObject emulatedPortObj)
                    {
                        HcsLogger.LogManagementObject(emulatedPortObj);
                    }
                }
            }

            using var resourceAllocCollection = systemSettings.GetRelated(VMQueries.RelatedSettings.Resource);
            foreach (var resourceAllocation in resourceAllocCollection)
            {
                using (resourceAllocation)
                {
                    if (resourceAllocation is ManagementObject resourceAllocationObj)
                    {
                        HcsLogger.LogManagementObject(resourceAllocationObj);
                    }
                }
            }

            using var ethernetPortAllocationCollection = systemSettings.GetRelated(VMQueries.RelatedSettings.EthernetPortAllocation);
            using var switchPortSettings = ethernetPortAllocationCollection.OfType<ManagementObject>().FirstOrDefault();
            if (switchPortSettings != null)
            {
                HcsLogger.LogManagementObject(switchPortSettings);

                using var portOffloadCollection = switchPortSettings.GetRelated(VMQueries.RelatedSettings.EthernetPortOffload);
                using var switchPortOffloadSettings = portOffloadCollection.OfType<ManagementObject>().FirstOrDefault();
                if (switchPortOffloadSettings != null)
                {
                    HcsLogger.LogManagementObject(switchPortOffloadSettings);
                }
            }

            using var shutdownCollection = systemSettings.GetRelated(VMQueries.RelatedSettings.Shutdown);
            using var shutdownSettings = shutdownCollection.OfType<ManagementObject>().FirstOrDefault();
            if (shutdownSettings != null)
            {
                HcsLogger.LogManagementObject(shutdownSettings);
            }

            using var heartbeatCollection = systemSettings.GetRelated(VMQueries.RelatedSettings.Heartbeat);
            using var heartbeatSettings = heartbeatCollection.OfType<ManagementObject>().FirstOrDefault();
            if (heartbeatSettings != null)
            {
                HcsLogger.LogManagementObject(heartbeatSettings);
            }

            using var vssCollection = systemSettings.GetRelated(VMQueries.RelatedSettings.VolumeShadowCopy);
            using var vssSettings = vssCollection.OfType<ManagementObject>().FirstOrDefault();
            if (vssSettings != null)
            {
                HcsLogger.LogManagementObject(vssSettings);
            }

            using var guestServiceCollection = systemSettings.GetRelated(VMQueries.RelatedSettings.GuestServiceInterface);
            using var guestServicesSettings = guestServiceCollection.OfType<ManagementObject>().FirstOrDefault();
            if (guestServicesSettings != null)
            {
                HcsLogger.LogManagementObject(guestServicesSettings);
            }

            using var securityCollection = systemSettings.GetRelated(VMQueries.RelatedSettings.Security);
            using var securitySettings = securityCollection.OfType<ManagementObject>().FirstOrDefault();
            if (securitySettings != null)
            {
                HcsLogger.LogManagementObject(securitySettings);
            }

            using var snapshotCollection = systemSettings.GetRelated(VMQueries.RelatedSettings.Sanpshot);
            foreach (var snapshot in snapshotCollection)
            {
                using (snapshot)
                {
                    if (snapshot is ManagementObject snapshotObj)
                    {
                        HcsLogger.LogManagementObject(snapshotObj);
                    }
                }
            }
        }
    }
}