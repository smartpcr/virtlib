// -----------------------------------------------------------------------
// <copyright file="VirtualSystemManagementService.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

using System;
using System.Management;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;

public class VirtualMachineManagementService
{
    private readonly ILogger<VirtualMachineManagementService> _logger;

    public string Caption { get; set; }
    public string CreationClassName { get; set; }
    public string Description { get; set; }
    public string ElementName { get; set; }
    public EnabledDefault EnabledDefault { get; set; }
    public EnabledState EnabledState { get; set; }
    public HealthState HealthState { get; set; }
    public DateTime InstallDate { get; set; }
    public string Name { get; set; }
    public ushort[] OperationalStatus { get; set; }
    public RequestedState RequestedState { get; set; }
    public bool Started { get; set; }
    public string Status { get; set; }
    public string[] StatusDescriptions { get; set; }
    public string SystemCreationClassName { get; set; }
    public string SystemName { get; set; }
    public DateTime TimeOfLastStateChange { get; set; }

    public VirtualMachineManagementService(IServiceProvider serviceProvider, ManagementObject vmmsObj)
    {
        var loggerFactory = serviceProvider.GetRequiredService<ILoggerFactory>();
        this._logger = loggerFactory.CreateLogger<VirtualMachineManagementService>();
        this._logger.LogManagementObject(vmmsObj);

        Caption = (string)vmmsObj["Caption"];
        CreationClassName = (string)vmmsObj["CreationClassName"];
        Description = (string)vmmsObj["Description"];
        ElementName = (string)vmmsObj["ElementName"];
        EnabledDefault = vmmsObj["EnabledDefault"].ReadEnabledDefault();
        EnabledState = vmmsObj["EnabledState"].ReadEnabledState();
        HealthState = vmmsObj["HealthState"].ReadHealthState();
        InstallDate = vmmsObj["InstallDate"].ReadDateTime() ?? DateTime.MinValue;
        Name = (string)vmmsObj["Name"];
        OperationalStatus = (ushort[])vmmsObj["OperationalStatus"];
        RequestedState = vmmsObj["RequestedState"].ReadRequestedState();
        Started = vmmsObj["Started"].ReadBool();
        Status = (string)vmmsObj["Status"];
        StatusDescriptions = (string[])vmmsObj["StatusDescriptions"];
        SystemCreationClassName = (string)vmmsObj["SystemCreationClassName"];
        SystemName = (string)vmmsObj["SystemName"];
        TimeOfLastStateChange = vmmsObj["TimeOfLastStateChange"].ReadDateTime() ?? DateTime.MinValue;
    }
}