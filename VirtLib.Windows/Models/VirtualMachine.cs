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
    public HealthState HealthState { get; set; }
    public RequestedState RequestedState { get; set; }
    public string Description { get; set; }
    public int ThreadsPerCore { get; set; }
    public int NumaNodes { get; set; }
    public DateTime? CreationTime { get; set; }
    public DateTime? ModificationTime { get; set; }
    public DateTime? LastStateChangeTime { get; set; }
    public List<BootDevice> BootEntries { get; set; } = new List<BootDevice>();

    public VirtualMachine(ManagementObject vmObj)
    {
        HcsLogger.LogManagementObject(vmObj);
        Name = vmObj["ElementName"]?.ToString() ?? string.Empty;
        HyperVInstanceId = Guid.Parse(vmObj["Name"]?.ToString() ?? Guid.Empty.ToString());
        EnabledDefault = vmObj["EnabledDefault"].ReadEnabledDefault();
        EnabledState = vmObj["EnabledState"].ReadEnabledState();
        HealthState = vmObj["HealthState"].ReadHealthState();
        RequestedState = vmObj["RequestedState"].ReadRequestedState();
        Description = vmObj["Description"]?.ToString() ?? string.Empty;
        ThreadsPerCore = vmObj["HwThreadsPerCoreRealized"] == null ? 0 : Convert.ToInt32(vmObj["HwThreadsPerCoreRealized"]);
        NumaNodes = vmObj["NumberOfNumaNodes"] == null ? 0 : Convert.ToInt32(vmObj["NumberOfNumaNodes"]);
        CreationTime = vmObj["InstallDate"] == null ? null : ManagementDateTimeConverter.ToDateTime(vmObj["InstallDate"].ToString());
        ModificationTime = vmObj["TimeOfLastConfigurationChange"] == null
            ? null
            : ManagementDateTimeConverter.ToDateTime(vmObj["TimeOfLastConfigurationChange"].ToString());
        LastStateChangeTime = vmObj["TimeOfLastStateChange"] == null
            ? null
            : ManagementDateTimeConverter.ToDateTime(vmObj["TimeOfLastStateChange"].ToString());

        using var systemSettings = vmObj.GetRelated(VMQueries.RelatedSettings.System).OfType<ManagementObject>().FirstOrDefault();
        if (systemSettings != null)
        {
            HcsLogger.LogManagementObject(systemSettings);

            using var processorSettings = systemSettings.GetRelated(VMQueries.RelatedSettings.Processor).OfType<ManagementObject>().FirstOrDefault();
            if (processorSettings != null)
            {
                HcsLogger.LogManagementObject(processorSettings);
            }

            using var memorySettings = systemSettings.GetRelated(VMQueries.RelatedSettings.Memory).OfType<ManagementObject>().FirstOrDefault();
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
        }

        var storageSettings = vmObj.GetRelated(VMQueries.RelatedSettings.Storage).OfType<ManagementObject>().FirstOrDefault();
        if (storageSettings != null)
        {
            HcsLogger.LogManagementObject(storageSettings);
        }

        var networkAdapterSettings = vmObj.GetRelated(VMQueries.RelatedSettings.NetworkAdapter).OfType<ManagementObject>().FirstOrDefault();
        if (networkAdapterSettings != null)
        {
            HcsLogger.LogManagementObject(networkAdapterSettings);
        }

        var switchPortSettings = vmObj.GetRelated(VMQueries.RelatedSettings.SwitchPort).OfType<ManagementObject>().FirstOrDefault();
        if (switchPortSettings != null)
        {
            HcsLogger.LogManagementObject(switchPortSettings);
        }

        var switchPortOffloadSettings = vmObj.GetRelated(VMQueries.RelatedSettings.SwitchPortOffload).OfType<ManagementObject>().FirstOrDefault();
        if (switchPortOffloadSettings != null)
        {
            HcsLogger.LogManagementObject(switchPortOffloadSettings);
        }

        var shutdownSettings = vmObj.GetRelated(VMQueries.RelatedSettings.Shutdown).OfType<ManagementObject>().FirstOrDefault();
        if (shutdownSettings != null)
        {
            HcsLogger.LogManagementObject(shutdownSettings);
        }

        var guestServicesSettings = vmObj.GetRelated(VMQueries.RelatedSettings.GuestServices).OfType<ManagementObject>().FirstOrDefault();
        if (guestServicesSettings != null)
        {
            HcsLogger.LogManagementObject(guestServicesSettings);
        }
    }
}