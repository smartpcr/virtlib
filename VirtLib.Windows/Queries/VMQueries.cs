// -----------------------------------------------------------------------
// <copyright file="VMQueries.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Queries;

internal static class VMQueries
{
    public static readonly string GetVMs = $"SELECT * FROM {VmWmiClasses.ComputerSystem} WHERE Caption = 'Virtual Machine'";
    public static readonly string GetVMByName = $"SELECT * FROM {VmWmiClasses.ComputerSystem} WHERE ElementName = '{{0}}'";
    public static readonly string GetVMsByResourcePool = $"SELECT * FROM {VmWmiClasses.ComputerSystem} WHERE ResourcePool = '{{0}}'";
    public static readonly string GetVMsByResourcePoolAndStatus = $"SELECT * FROM {VmWmiClasses.ComputerSystem} WHERE ResourcePool = '{{0}}' AND EnabledState = {{1}}";
}