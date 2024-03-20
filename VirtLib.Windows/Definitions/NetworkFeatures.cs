// -----------------------------------------------------------------------
// <copyright file="NetworkFeatures.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Definitions;

using System;
using System.Linq;
using System.Management;
using Microsoft.Extensions.Logging;
using Queries;

public enum NetworkFeatures
{
    Bandwidth,
    Offload,
    Security,
    Vlan
}

public static class FeatureExtensions
{
    public static Guid ToFeatureGuid(this NetworkFeatures networkFeatures)
    {
        return networkFeatures switch
        {
            NetworkFeatures.Bandwidth => new Guid("24AD3CE1-69BD-4978-B2AC-DAAD389D699C"),
            NetworkFeatures.Offload => new Guid("C885BFD1-ABB7-418F-8163-9F379C9F7166"),
            NetworkFeatures.Security => new Guid("776E0BA7-94A1-41C8-8F28-951F524251B5"),
            NetworkFeatures.Vlan => new Guid("952C5004-4465-451C-8CB8-FA9AB382B773"),
            _ => throw new ArgumentOutOfRangeException(nameof(networkFeatures), networkFeatures, null)
        };
    }

    public static string ToWmiClass(this NetworkFeatures networkFeature)
    {
        return networkFeature switch
        {
            NetworkFeatures.Bandwidth => "Msvm_EthernetSwitchBandwidthSettingData",
            NetworkFeatures.Offload => "Msvm_EthernetSwitchPortOffloadSettingData",
            NetworkFeatures.Security => "Msvm_EthernetSwitchSecuritySettingData",
            NetworkFeatures.Vlan => "Msvm_EthernetSwitchVlanSettingData",
            _ => throw new ArgumentOutOfRangeException(nameof(networkFeature), networkFeature, null)
        };
    }

    public static ManagementObject CreateFeatureSettings(this NetworkFeatures networkFeatures, ManagementScope scope)
    {
        var featureGuid = networkFeatures.ToFeatureGuid();
        string? defaultFeatureSettingPath = null;
        using var featureCapabilityClass = new ManagementClass(VmWmiClasses.EthernetSwitchFeatureCapabilities);
        featureCapabilityClass.Scope = scope;
        using var featureCapabilitiesCollection = featureCapabilityClass.GetInstances();
        foreach (ManagementObject featureCapabilities in featureCapabilitiesCollection.OfType<ManagementObject>())
        {
            using (featureCapabilities)
            {
                if (string.Equals((string)featureCapabilities["FeatureId"], featureGuid.ToString(), StringComparison.OrdinalIgnoreCase))
                {
                    using ManagementObjectCollection featureSettingAssociationCollection = featureCapabilities.GetRelationships(VmWmiClasses.FeatureSettingsDefineCapabilities);
                    foreach (ManagementObject featureSettingAssociation in featureSettingAssociationCollection.OfType<ManagementObject>())
                    {
                        using (featureSettingAssociation)
                        {
                            if ((ushort)featureSettingAssociation["ValueRole"] == 0)
                            {
                                defaultFeatureSettingPath = (string)featureSettingAssociation["PartComponent"];
                                break;
                            }
                        }
                    }

                    break;
                }
            }
        }

        if (defaultFeatureSettingPath == null)
        {
            throw new ManagementException("Unable to find the Default Feature Settings.");
        }

        ManagementObject defaultFeatureSetting = new ManagementObject(defaultFeatureSettingPath)
        {
            Scope = scope
        };
        defaultFeatureSetting.Get();
        return defaultFeatureSetting;
    }

    public static void AddFeatureSettings(
        this ManagementObject vmms,
        ILogger logger,
        ManagementObject ethernetPortAllocationSettings,
        ManagementObject[] featureSettings,
        out ManagementObject[] resultingFeatureSettings)
    {
        using ManagementBaseObject inputParameters = vmms.GetMethodParameters("AddFeatureSettings");
        inputParameters["AffectedConfiguration"] = ethernetPortAllocationSettings.Path.Path;
        inputParameters["FeatureSettings"] = featureSettings.ToStringArray();
        using ManagementBaseObject outputParameters = vmms.InvokeMethod("AddFeatureSettings", inputParameters, null);
        JobOutputHelper.ValidateOutput(outputParameters, logger);
        resultingFeatureSettings = ((string[])outputParameters["ResultingFeatureSettings"]).ToObjectArray();
    }
}
