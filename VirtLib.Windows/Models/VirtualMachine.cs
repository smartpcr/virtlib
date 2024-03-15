// -----------------------------------------------------------------------
// <copyright file="VirtualMachine.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

using System;
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

        var systemSettings = vmObj.GetRelated(VMQueries.RelatedSettings.System).OfType<ManagementObject>().FirstOrDefault();

    }
}