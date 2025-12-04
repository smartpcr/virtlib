// -----------------------------------------------------------------------
// <copyright file="VMQueries.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Queries;

using System;
using System.Management;

public enum VMSetting
{
    System,
    Component,
    Resource,
    Memory,
    Processor,
    Storage,
    Security,
    VirtualHardDisk,
    SyntheticEthernetPort,
    EmulatedEthernetPort,
    EthernetPortAllocation,
    EthernetPortOffload,
    Shutdown,
    TimeSynchronization,
    DataExchange,
    Heartbeat,
    VolumeShadowCopy,
    GuestServiceInterface,
    Sanpshot
}

internal static class VMQueries
{
    private static class RelatedSettings
    {
        public const string System = "Msvm_VirtualSystemSettingData";
        public const string Component = "Msvm_VirtualSystemSettingDataComponent";
        public const string Resource = "Msvm_ResourceAllocationSettingData";
        public const string Memory = "Msvm_MemorySettingData";
        public const string Processor = "Msvm_ProcessorSettingData";
        public const string Storage = "Msvm_StorageAllocationSettingData";
        public const string Security = "Msvm_SecuritySettingData";
        public const string VirtualHardDisk = "Msvm_VirtualHardDiskSettingData";
        public const string SyntheticEthernetPort = "Msvm_SyntheticEthernetPortSettingData";
        public const string EmulatedEthernetPort = "Msvm_EmulatedEthernetPortSettingData";
        public const string EthernetPortAllocation = "Msvm_EthernetPortAllocationSettingData";
        public const string EthernetPortOffload = "Msvm_EthernetSwitchPortOffloadSettingData";
        public const string Shutdown = "Msvm_ShutdownComponentSettingData";
        public const string TimeSynchronization = "Msvm_TimeSyncComponentSettingData";
        public const string DataExchange = "Msvm_KvpExchangeComponentSettingData";
        public const string Heartbeat = "Msvm_HeartbeatComponentSettingData";
        public const string VolumeShadowCopy = "Msvm_VssComponentSettingData";
        public const string GuestServiceInterface = "Msvm_GuestServiceInterfaceComponentSettingData";
        public const string Sanpshot = "Msvm_SnapshotOfVirtualSystem";
    }

    public static string GetVMSettingWmiClass(VMSetting vmSetting)
    {
        return vmSetting switch
        {
            VMSetting.System => RelatedSettings.System,
            VMSetting.Component => RelatedSettings.Component,
            VMSetting.Resource => RelatedSettings.Resource,
            VMSetting.Memory => RelatedSettings.Memory,
            VMSetting.Processor => RelatedSettings.Processor,
            VMSetting.Storage => RelatedSettings.Storage,
            VMSetting.Security => RelatedSettings.Security,
            VMSetting.VirtualHardDisk => RelatedSettings.VirtualHardDisk,
            VMSetting.SyntheticEthernetPort => RelatedSettings.SyntheticEthernetPort,
            VMSetting.EmulatedEthernetPort => RelatedSettings.EmulatedEthernetPort,
            VMSetting.EthernetPortAllocation => RelatedSettings.EthernetPortAllocation,
            VMSetting.EthernetPortOffload => RelatedSettings.EthernetPortOffload,
            VMSetting.Shutdown => RelatedSettings.Shutdown,
            VMSetting.TimeSynchronization => RelatedSettings.TimeSynchronization,
            VMSetting.DataExchange => RelatedSettings.DataExchange,
            VMSetting.Heartbeat => RelatedSettings.Heartbeat,
            VMSetting.VolumeShadowCopy => RelatedSettings.VolumeShadowCopy,
            VMSetting.GuestServiceInterface => RelatedSettings.GuestServiceInterface,
            VMSetting.Sanpshot => RelatedSettings.Sanpshot,
            _ => throw new ArgumentOutOfRangeException(nameof(vmSetting), vmSetting, null)
        };
    }

    public static ManagementObject CreateVMSettingObject(VMSetting vmSetting, ManagementScope scope)
    {
        using ManagementClass managementClass = new ManagementClass(GetVMSettingWmiClass(vmSetting));
        managementClass.Scope = scope;
        return managementClass.CreateInstance();
    }

    public static readonly string GetVMs = $"SELECT * FROM {VmWmiClasses.ComputerSystem} WHERE Caption = 'Virtual Machine'";
    public static readonly string GetVMByName = $"SELECT * FROM {VmWmiClasses.ComputerSystem} WHERE ElementName = '{{0}}'";
    public static readonly string GetVMsByResourcePool = $"SELECT * FROM {VmWmiClasses.ComputerSystem} WHERE ResourcePool = '{{0}}'";
    public static readonly string GetVMsByResourcePoolAndStatus = $"SELECT * FROM {VmWmiClasses.ComputerSystem} WHERE ResourcePool = '{{0}}' AND EnabledState = {{1}}";
}