// -----------------------------------------------------------------------
// <copyright file="WmiUtilities.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows;

using System.Linq;
using System.Management;
using Models;
using Queries;

public static class WmiUtilities
{
    internal static SwitchInfo? FindVSwitch(string switchName, ManagementScope scope)
    {
        var query = new ObjectQuery(string.Format(VSwitchQueries.GetVSwitchByName, switchName));
        using var searcher = new ManagementObjectSearcher(scope, query);
        var instance = searcher.Get().OfType<ManagementObject>().FirstOrDefault();
        return instance == null ? null : new SwitchInfo(instance);
    }

    public static ManagementObjectCollection GetRelated(this ManagementObject managementObject, string relatedClass, string relatedRole)
    {
        return managementObject.GetRelated(relatedClass, relatedRole, null, null, null, null, false, null);
    }

    public static ManagementObject? GetFirstObject(this ManagementObjectCollection collection)
    {
        return collection.OfType<ManagementObject>().FirstOrDefault();
    }
}