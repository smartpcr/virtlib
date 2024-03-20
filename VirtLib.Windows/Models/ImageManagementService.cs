// -----------------------------------------------------------------------
// <copyright file="ImageManagementService.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

using System;
using System.Management;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;

public class ImageManagementService
{
    private readonly ILogger<ImageManagementService> _logger;

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

    public ImageManagementService(IServiceProvider serviceProvider, ManagementObject imageMgmtObj)
    {
        var loggerFactory = serviceProvider.GetRequiredService<ILoggerFactory>();
        this._logger = loggerFactory.CreateLogger<ImageManagementService>();
        this._logger.LogManagementObject(imageMgmtObj);

        Caption = (string)imageMgmtObj["Caption"];
        CreationClassName = (string)imageMgmtObj["CreationClassName"];
        Description = (string)imageMgmtObj["Description"];
        ElementName = (string)imageMgmtObj["ElementName"];
        EnabledDefault = imageMgmtObj["EnabledDefault"].ReadEnabledDefault();
        EnabledState = imageMgmtObj["EnabledState"].ReadEnabledState();
        HealthState = imageMgmtObj["HealthState"].ReadHealthState();
        InstallDate = imageMgmtObj["InstallDate"].ReadDateTime() ?? DateTime.MinValue;
        Name = (string)imageMgmtObj["Name"];
        OperationalStatus = (ushort[])imageMgmtObj["OperationalStatus"];
        RequestedState = imageMgmtObj["RequestedState"].ReadRequestedState();
        Started = (bool)imageMgmtObj["Started"];
        Status = (string)imageMgmtObj["Status"];
        StatusDescriptions = (string[])imageMgmtObj["StatusDescriptions"];
        SystemCreationClassName = (string)imageMgmtObj["SystemCreationClassName"];
        SystemName = (string)imageMgmtObj["SystemName"];
        TimeOfLastStateChange = imageMgmtObj["TimeOfLastStateChange"].ReadDateTime() ?? DateTime.MinValue;
    }
}