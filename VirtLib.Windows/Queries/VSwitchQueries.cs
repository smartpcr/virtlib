// -----------------------------------------------------------------------
// <copyright file="VirtualSwitchQueries.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Queries;

internal static class VSwitchQueries
{
    public static readonly string GetVSwitches = $"SELECT * FROM {VmWmiClasses.VirtualEthernetSwitch} WHERE Caption = 'Virtual Switch'";
    public static readonly string GetVSwitchByName = $"SELECT * FROM {VmWmiClasses.VirtualEthernetSwitch} WHERE Caption = 'Virtual Switch' AND ElementName = '{{0}}'";

}