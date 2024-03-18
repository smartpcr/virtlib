// -----------------------------------------------------------------------
// <copyright file="DynamicEthernetSwitchPort.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

public class DynamicEthernetSwitchPort
{
    public string AllocationUnits { get; set; }
    public bool AutomaticAllocation { get; set; }
    public bool AutomaticDeallocation { get; set; }
    public ushort ConsumerVisibility { get; set; }
    public string Description { get; set; }
    public bool EnabledState { get; set; }
    public string[] HostResource { get; set; }
    public string InstanceId { get; set; }
    public string LastKnownSwitchName { get; set; }
    public ulong Limit { get; set; }
    public string Parent { get; set; }
    public string PoolId { get; set; }
    public string[] RequiredFeatureHints { get; set; }
    public string[] RequiredFeatures { get; set; }
    public ulong Reservation { get; set; }
    public string ResourceSubType { get; set; }
    public ushort ResourceType { get; set; }
    public string TestReplicaPoolId { get; set; }
    public string TestReplicaSwitchName { get; set; }
    public string VirtualQuantity { get; set; }
    public string VirtualQuantityUnits { get; set; }
    public uint Weight { get; set; }
}