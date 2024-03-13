// -----------------------------------------------------------------------
// <copyright file="HyperVHost.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace virt.net;

using System;
using System.Management;
using System.Runtime.Versioning;

[SupportedOSPlatform("windows")]
public class HyperVHost
{
    public string? ElementName { get; set; }
    public string? Name { get; set; }
    public string? Status { get; set; }
    public bool Started { get; set; }

    public static HyperVHost? FromManagementObject(ManagementObject? m)
    {
        if (m == null)
        {
            return null;
        }

        foreach(var p in m.Properties)
        {
            Console.WriteLine($"{p.Name} = {p.Value}");
        }

        return new HyperVHost
        {
            ElementName = m["ElementName"]?.ToString(),
            Name = m["Name"]?.ToString(),
            Status = m["Status"]?.ToString(),
            Started = m["Started"]?.ToString()?.Equals("TRUE", StringComparison.OrdinalIgnoreCase) ?? false
        };
    }
}