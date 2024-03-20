// -----------------------------------------------------------------------
// <copyright file="VmWmiClasses.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Queries;

public static class VmWmiClasses
{
    public const string ComputerSystem = "Msvm_ComputerSystem";
    public const string VirtualSystemSettingData = "Msvm_VirtualSystemSettingData";
    public const string VirtualSystemSettingDataComponent = "Msvm_VirtualSystemSettingDataComponent";

    public const string ResourcePool = "Msvm_ResourcePool";
    public const string ProcessorPool = "Msvm_ProcessorPool";

    public const string VirtualEthernetSwitch = "Msvm_VirtualEthernetSwitch";
    public const string EthernetSwitchPort = "Msvm_EthernetSwitchPort";
    public const string SystemDevice = "Msvm_SystemDevice";
    public const string EthernetPortAllocationSettingData = "Msvm_EthernetPortAllocationSettingData";
    public const string ElementSettingData = "Msvm_ElementSettingData";
    public const string ExternalEthernetPort = "Msvm_ExternalEthernetPort";
    public const string SyntheticEthernetPortSettingData = "Msvm_SyntheticEthernetPortSettingData";
    public const string EmulatedEthernetPortSettingData = "Msvm_EmulatedEthernetPortSettingData";
    public const string EthernetSwitchPortFeatureSettingData = "Msvm_EthernetSwitchPortFeatureSettingData";
    public const string EthernetPortSettingDataComponent = "Msvm_EthernetPortSettingDataComponent";
    public const string EthernetSwitchFeatureSettingData = "Msvm_EthernetSwitchFeatureSettingData";
    public const string VirtualEthernetSwitchSettingDataComponent = "Msvm_VirtualEthernetSwitchSettingDataComponent";
    public const string VLANEndpointSettingData = "Msvm_VLANEndpointSettingData";

    public const string AllocationCapabilities = "Msvm_AllocationCapabilities";
    public const string SettingsDefineCapabilities = "SettingsDefineCapabilities";
    public const string EthernetSwitchFeatureCapabilities = "Msvm_EthernetSwitchFeatureCapabilities";
    public const string FeatureSettingsDefineCapabilities = "Msvm_FeatureSettingsDefineCapabilities";

    public const string HgsGuardian = "MSFT_HgsGuardian";
    public const string HgsKeyProtector = "MSFT_HgsKeyProtector";
}