// -----------------------------------------------------------------------
// <copyright file="HyperVHostInfo.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

using System;
using System.Management;
using System.Runtime.Versioning;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;

[SupportedOSPlatform("windows")]
public class HyperVHostInfo
{
    private readonly ILogger<HyperVHostInfo> _logger;

    public string? ElementName { get; set; }
    public string? Name { get; set; }
    public string? Status { get; set; }
    public bool Started { get; set; }

    public HyperVHostInfo(IServiceProvider serviceProvider, ManagementObject managementObject)
    {
        var loggerFactory = serviceProvider.GetRequiredService<ILoggerFactory>();
        this._logger = loggerFactory.CreateLogger<HyperVHostInfo>();
        this._logger.LogManagementObject(managementObject);

        ElementName = managementObject["ElementName"]?.ToString();
        Name = managementObject["Name"]?.ToString();
        Status = managementObject["Status"]?.ToString();
        Started = managementObject["Started"]?.ToString()?.Equals("TRUE", StringComparison.OrdinalIgnoreCase) ?? false;
    }
}