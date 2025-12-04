// -----------------------------------------------------------------------
// <copyright file="OSInfo.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

using System;
using System.Management;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;

public class OSInfo
{
    private readonly ILogger<OSInfo> _logger;

    public string? Name { get; set; }
    public string? Version { get; set; }

    public OSInfo(IServiceProvider serviceProvider, ManagementObject managementObject)
    {
        var loggerFactory = serviceProvider.GetRequiredService<ILoggerFactory>();
        this._logger = loggerFactory.CreateLogger<OSInfo>();
        this._logger.LogManagementObject(managementObject);
        this.Name = managementObject["Name"]?.ToString();
        this.Version = managementObject["Version"]?.ToString();
    }
}