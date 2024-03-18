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

    public List<VirtualMachineSettings> Settings { get; set; } = new List<VirtualMachineSettings>();
    public List<BootDevice> BootEntries { get; set; } = new List<BootDevice>();


    public VirtualMachine(ManagementObject vmObj)
    {
        HcsLogger.LogManagementObject(vmObj);
        Name = vmObj["ElementName"]?.ToString() ?? string.Empty;
        HyperVInstanceId = Guid.Parse(vmObj["Name"]?.ToString() ?? Guid.Empty.ToString());
        EnabledDefault = vmObj["EnabledDefault"].ReadEnabledDefault();
        EnabledState = vmObj["EnabledState"].ReadEnabledState();
        SessionModeState = vmObj["EnhancedSessionModeState"].ReadEnhancedSessionModeState();
        ReplicationType = vmObj["FailedOverReplicationType"].ReadFailedOverReplicationType();
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
        foreach (var systemSettings in systemCollection)
        {
            using (systemSettings)
            {
                if (systemSettings is ManagementObject systemSettingsObj)
                {
                    this.Settings.Add(new VirtualMachineSettings(systemSettingsObj));

                    if (systemSettingsObj["BootSourceOrder"] is string[] bootEntries)
                    {
                        foreach (var bootEntry in bootEntries)
                        {
                            var bootDevice = new BootDevice(bootEntry);
                            BootEntries.Add(bootDevice);
                        }
                    }

                    using var storageCollection = systemSettingsObj.GetRelated(VMQueries.RelatedSettings.Storage);
                    using var storageSettings = storageCollection.OfType<ManagementObject>().FirstOrDefault();
                    if (storageSettings != null)
                    {
                        HcsLogger.LogManagementObject(storageSettings);
                    }

                    using var snapshotCollection = systemSettingsObj.GetRelated(VMQueries.RelatedSettings.Sanpshot);
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

    }
}