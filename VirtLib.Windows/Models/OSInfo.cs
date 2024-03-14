// -----------------------------------------------------------------------
// <copyright file="OSInfo.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

using System.Management;

public class OSInfo
{
    public string? Name { get; set; }
    public string? Version { get; set; }

    public OSInfo(ManagementObject managementObject)
    {
        HcsLogger.LogManagementObject(managementObject);
        this.Name = managementObject["Name"]?.ToString();
        this.Version = managementObject["Version"]?.ToString();
    }
}