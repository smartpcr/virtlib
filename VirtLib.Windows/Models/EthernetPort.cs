// -----------------------------------------------------------------------
// <copyright file="EthernetPort.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

using System.Management;

public class EthernetPort
{
    public string Address { get; set; }
    public string AllocationUnits { get; set; }
    public bool AllowDirectTranslatedP2P { get; set; }
    public bool AllowPacketDirect { get; set; }
    public bool AutomaticAllocation { get; set; }
    public bool AutomaticDeallocation { get; set; }
    public bool ClusterMonitored { get; set; }
    public string[] Connection { get; set; }
    public ushort ConsumerVisibility { get; set; }
    public string Description { get; set; }
    public bool DeviceNamingEnabled { get; set; }
    public string InstanceId { get; set; }
    public bool InterruptModeration { get; set; }
    public ulong Limit { get; set; }
    public uint MediaType { get; set; }
    public bool NumaAwarePlacement { get; set; }
    public string PoolId { get; set; }
    public ulong Reservation { get; set; }
    public string ResourceSubType { get; set; }
    public ushort ResourceType { get; set; }
    public bool StaticMacAddress { get; set; }
    public ulong VirtualQuantity { get; set; }
    public string VirtualQuantityUnits { get; set; }
    public string[] VirtualSystemIdentifiers { get; set; }
    public uint Weight { get; set; }

    public EthernetPort(ManagementObject portObj)
    {
        HcsLogger.LogManagementObject(portObj);

        Address = (string)portObj["Address"];
        AllocationUnits = (string)portObj["AllocationUnits"];
        AllowDirectTranslatedP2P = (bool)portObj["AllowDirectTranslatedP2P"];
        AllowPacketDirect = (bool)portObj["AllowPacketDirect"];
        AutomaticAllocation = (bool)portObj["AutomaticAllocation"];
        AutomaticDeallocation = (bool)portObj["AutomaticDeallocation"];
        ClusterMonitored = (bool)portObj["ClusterMonitored"];
        Connection = (string[])portObj["Connection"];
        ConsumerVisibility = (ushort)portObj["ConsumerVisibility"];
        Description = (string)portObj["Description"];
        DeviceNamingEnabled = (bool)portObj["DeviceNamingEnabled"];
        InstanceId = (string)portObj["InstanceID"];
        InterruptModeration = (bool)portObj["InterruptModeration"];
        Limit = (ulong)portObj["Limit"];
        MediaType = (uint)portObj["MediaType"];
        NumaAwarePlacement = (bool)portObj["NumaAwarePlacement"];
        PoolId = (string)portObj["PoolID"];
        Reservation = (ulong)portObj["Reservation"];
        ResourceSubType = (string)portObj["ResourceSubType"];
        ResourceType = (ushort)portObj["ResourceType"];
        StaticMacAddress = (bool)portObj["StaticMacAddress"];
        VirtualQuantity = (ulong)portObj["VirtualQuantity"];
        VirtualQuantityUnits = (string)portObj["VirtualQuantityUnits"];
        VirtualSystemIdentifiers = (string[])portObj["VirtualSystemIdentifiers"];
        Weight = (uint)portObj["Weight"];
    }
}