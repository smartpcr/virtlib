// -----------------------------------------------------------------------
// <copyright file="HyperVHostInfo.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

using System;
using System.Management;
using System.Runtime.Versioning;

[SupportedOSPlatform("windows")]
public class HyperVHostInfo
{
    public string? ElementName { get; set; }
    public string? Name { get; set; }
    public string? Status { get; set; }
    public bool Started { get; set; }

    public HyperVHostInfo(ManagementObject managementObject)
    {
        HcsLogger.LogManagementObject(managementObject);
        ElementName = managementObject["ElementName"]?.ToString();
        Name = managementObject["Name"]?.ToString();
        Status = managementObject["Status"]?.ToString();
        Started = managementObject["Started"]?.ToString()?.Equals("TRUE", StringComparison.OrdinalIgnoreCase) ?? false;
    }
}