// -----------------------------------------------------------------------
// <copyright file="PortFeatureType.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

using System.Management;

/// <summary>
/// Advanced port feature type.
/// </summary>
public enum PortFeatureType
{
    Unknown,
    Acl,
    Bandwidth,
    Offload,
    Profile,
    Security,
    Vlan
}

public static class PortFeatureTypeEx
{
    public static PortFeatureType ReadPortFeatureType(this ManagementObject managementObject)
    {
        string featureName = managementObject.Path.ClassName;
        return featureName switch
        {
            "Msvm_EthernetSwitchPortAclSettingData" => PortFeatureType.Acl,
            "Msvm_EthernetSwitchPortBandwidthSettingData" => PortFeatureType.Bandwidth,
            "Msvm_EthernetSwitchPortOffloadSettingData" => PortFeatureType.Offload,
            "Msvm_EthernetSwitchPortProfileSettingData" => PortFeatureType.Profile,
            "Msvm_EthernetSwitchPortSecuritySettingData" => PortFeatureType.Security,
            "Msvm_EthernetSwitchPortVlanSettingData" => PortFeatureType.Vlan,
            _ => PortFeatureType.Unknown
        };
    }
}
