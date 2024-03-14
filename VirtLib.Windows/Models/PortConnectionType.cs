// -----------------------------------------------------------------------
// <copyright file="PortConnectionType.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

using System;
using System.Management;
using Queries;

/// <summary>
/// The type of connection for a single port.
/// </summary>
public enum PortConnectionType
{
    Nothing,
    Internal,
    External,
    VirtualMachine
}

public static class PortConnectionTypeEx
{
    public static PortConnectionType ReadPortConnectionType(this ManagementObject? portSettingObj)
    {
        if (portSettingObj == null)
        {
            return PortConnectionType.Nothing;
        }

        string[]? hostResource = portSettingObj["HostResource"] as string[];
        if (hostResource == null || hostResource.Length == 0)
        {
            return PortConnectionType.Nothing;
        }

        var hostResourcePath = new ManagementPath(hostResource[0]);
        if (hostResourcePath.ClassName.Equals(VmWmiClasses.ComputerSystem, StringComparison.OrdinalIgnoreCase))
        {
            return PortConnectionType.Internal;
        }

        if (hostResourcePath.ClassName.Equals(VmWmiClasses.ExternalEthernetPort, StringComparison.OrdinalIgnoreCase))
        {
            return PortConnectionType.External;
        }

        var parent = portSettingObj["Parent"] as string;
        if (!string.IsNullOrEmpty(parent))
        {
            var parentPath = new ManagementPath(parent);
            if (parentPath.ClassName.Equals(VmWmiClasses.SyntheticEthernetPortSettingData, StringComparison.OrdinalIgnoreCase))
            {
                return PortConnectionType.VirtualMachine;
            }

            if (parentPath.ClassName.Equals(VmWmiClasses.EmulatedEthernetPortSettingData, StringComparison.OrdinalIgnoreCase))
            {
                return PortConnectionType.VirtualMachine;
            }
        }

        return PortConnectionType.Nothing;
    }
}