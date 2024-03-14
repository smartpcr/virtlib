// -----------------------------------------------------------------------
// <copyright file="VirtualSwitchQueries.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Queries;

using System.Management;
using Models;

internal static class VSwitchQueries
{
    public static readonly string GetVSwitches = $"SELECT * FROM {VmWmiClasses.VirtualEthernetSwitch} WHERE Caption = 'Virtual Switch'";
    public static readonly string GetVSwitchByName = $"SELECT * FROM {VmWmiClasses.VirtualEthernetSwitch} WHERE Caption = 'Virtual Switch' AND ElementName = '{{0}}'";

    public static ManagementObjectCollection GetPorts(this SwitchInfo switchInfo)
    {
        return switchInfo.Inner.GetRelated(VmWmiClasses.EthernetSwitchPort, VmWmiClasses.SystemDevice);
    }

    public static ManagementObjectCollection GetPortSettings(this PortInfo portInfo)
    {
        return portInfo.Inner.GetRelated(VmWmiClasses.EthernetPortAllocationSettingData, VmWmiClasses.ElementSettingData);
    }
}