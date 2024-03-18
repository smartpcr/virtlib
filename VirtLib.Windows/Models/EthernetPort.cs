// -----------------------------------------------------------------------
// <copyright file="EthernetPort.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

public class EthernetPort
{
    public string Address { get; set; }
    public string AllocationUnits { get; set; }
    public bool AllowDirectTranslatedP2P { get; set; }
    public bool AllowPacketDirect { get; set; }
    public bool AutomaticAllocation { get; set; }
    public bool AutomaticDeallocation { get; set; }
    public bool ClusterMonitored { get; set; }
    public string Connection { get; set; }
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
}