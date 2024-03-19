// -----------------------------------------------------------------------
// <copyright file="ResourceQueries.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Queries;

using System;
using System.Linq;
using System.Management;

public enum ResourceSubtype
{
    Processor,
    Memory,
    SCSIController,
    VirtualHardDrive,
    VirtualHardDisk,
    VirtualDvdDrive,
    VirtualDvdDisk,
    NetworkAdapter,
    SwitchPort
}

public static class ResourceQueries
{
    public static string GetResourceSubtypeName(ResourceSubtype resourceSubtype)
    {
        return resourceSubtype switch
        {
            ResourceSubtype.Processor => "Microsoft:Hyper-V:Processor",
            ResourceSubtype.Memory => "Microsoft:Hyper-V:Memory",
            ResourceSubtype.SCSIController => "Microsoft:Hyper-V:Synthetic SCSI Controller",
            ResourceSubtype.VirtualHardDrive => "Microsoft:Hyper-V:Synthetic Disk Drive",
            ResourceSubtype.VirtualHardDisk => "Microsoft:Hyper-V:Virtual Hard Disk",
            ResourceSubtype.VirtualDvdDrive => "Microsoft:Hyper-V:Synthetic DVD Drive",
            ResourceSubtype.VirtualDvdDisk => "Microsoft:Hyper-V:Virtual CD/DVD Disk",
            ResourceSubtype.NetworkAdapter => "Microsoft:Hyper-V:Synthetic Ethernet Port",
            ResourceSubtype.SwitchPort => "Microsoft:Hyper-V:Ethernet Connection",
            _ => throw new ArgumentOutOfRangeException(nameof(resourceSubtype), resourceSubtype, null)
        };
    }

    public static string GetResourceSubtypeQuery(ResourceSubtype resourceSubtype)
    {
        var resourcePoolClassName = resourceSubtype == ResourceSubtype.Processor
            ? VmWmiClasses.ProcessorPool
            : VmWmiClasses.ResourcePool;
        var resourceSubtypeName = GetResourceSubtypeName(resourceSubtype);
        var query = $"SELECT * FROM {resourcePoolClassName} WHERE ResourceSubType = \"{resourceSubtypeName}\" AND Primordial = True";
        return query;
    }

    public static ManagementObject CreateDefaultResource(ResourceSubtype resourceSubtype, ManagementScope scope)
    {
        var query = GetResourceSubtypeQuery(resourceSubtype);
        using var searcher = new ManagementObjectSearcher(scope, new ObjectQuery(query));
        using var collection = searcher.Get();
        using var resourcePool = collection.OfType<ManagementObject>().First();
        using var capabilitiesCollection = resourcePool.GetRelated(VmWmiClasses.AllocationCapabilities);
        using var capabilities = capabilitiesCollection.OfType<ManagementObject>().First();
        using var settingAssociations = capabilities.GetRelationships(VmWmiClasses.SettingsDefineCapabilities);
        using var setting = settingAssociations.OfType<ManagementObject>().FirstOrDefault(s => (ushort)s["ValueRole"] == 0);
        var defaultSettingPath = setting?["PartComponent"]?.ToString();
        if (string.IsNullOrEmpty(defaultSettingPath))
        {
            throw new ManagementException("Unable to find the Default Resource Settings.");
        }

        var defaultSetting = new ManagementObject(defaultSettingPath)
        {
            Scope = scope
        };
        defaultSetting.Get();
        return defaultSetting;
    }

    public static void AddResourceSettings(this ManagementObject vmms, ManagementObject systemSettings, ManagementObject[] resourceSettings, out ManagementObject[] resultingResourceSettings)
    {
        using ManagementBaseObject inputParameters = vmms.GetMethodParameters("AddResourceSettings");
        inputParameters["AffectedConfiguration"] = systemSettings.Path.Path;
        inputParameters["ResourceSettings"] = resourceSettings.ToStringArray();
        using ManagementBaseObject outputParameters = vmms.InvokeMethod("AddResourceSettings", inputParameters, null);
        JobOutputHelper.ValidateOutput(outputParameters);
        resultingResourceSettings = ((string[])outputParameters["ResultingResourceSettings"]).ToObjectArray();
    }

}