// -----------------------------------------------------------------------
// <copyright file="SwitchFeatureType.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

using System.Management;

/// <summary>
/// Advanced switch feature type.
/// </summary>
public enum SwitchFeatureType
{
    Unknown,
    Bandwidth
}

public static class SwitchFeatureTypeEx
{
    public static SwitchFeatureType ReadSwitchFeatureType(this ManagementObject managementObject)
    {
        string featureName = managementObject.Path.ClassName;
        return featureName switch
        {
            "Msvm_EthernetSwitchBandwidthSettingData" => SwitchFeatureType.Bandwidth,
            _ => SwitchFeatureType.Unknown
        };
    }
}