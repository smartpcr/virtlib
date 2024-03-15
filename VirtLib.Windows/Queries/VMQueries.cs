// -----------------------------------------------------------------------
// <copyright file="VMQueries.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Queries;

internal static class VMQueries
{
    public static class RelatedSettings
    {
        public const string System = "Msvm_VirtualSystemSettingData";
        public const string Resource = "Msvm_ResourceAllocationSettingData";
        public const string Memory = "Msvm_MemorySettingData";
        public const string Processor = "Msvm_ProcessorSettingData";
        public const string Storage = "Msvm_StorageAllocationSettingData";
        public const string Security = "Msvm_SecuritySettingData";
        public const string VirtualHardDisk = "Msvm_VirtualHardDiskSettingData";
        public const string NetworkAdapter = "Msvm_SyntheticEthernetPortSettingData";
        public const string SwitchPort = "Msvm_EthernetPortAllocationSettingData";
        public const string SwitchPortOffload = "Msvm_EthernetSwitchPortOffloadSettingData";
        public const string Shutdown = "Msvm_ShutdownComponentSettingData";
        public const string TimeSynchronization = "Msvm_TimeSyncComponentSettingData";
        public const string DataExchange = "Msvm_KvpExchangeComponentSettingData";
        public const string Heartbeat = "Msvm_HeartbeatComponentSettingData";
        public const string VolumeShadowCopy = "Msvm_VssComponentSettingData";
        public const string GuestServices = "Msvm_GuestServiceInterfaceComponentSettingData";
    }

    public static readonly string GetVMs = $"SELECT * FROM {VmWmiClasses.ComputerSystem} WHERE Caption = 'Virtual Machine'";
    public static readonly string GetVMByName = $"SELECT * FROM {VmWmiClasses.ComputerSystem} WHERE ElementName = '{{0}}'";
    public static readonly string GetVMsByResourcePool = $"SELECT * FROM {VmWmiClasses.ComputerSystem} WHERE ResourcePool = '{{0}}'";
    public static readonly string GetVMsByResourcePoolAndStatus = $"SELECT * FROM {VmWmiClasses.ComputerSystem} WHERE ResourcePool = '{{0}}' AND EnabledState = {{1}}";
}