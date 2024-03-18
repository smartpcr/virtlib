// -----------------------------------------------------------------------
// <copyright file="HardDiskImage.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

using System;

public class HardDiskImage
{
    public string AllocationUnits { get; set; }
    public bool AutomaticAllocation { get; set; }
    public bool AutomaticDeallocation { get; set; }
    public ushort CachingMode { get; set; }
    public ushort ConsumerVisibility { get; set; }
    public string Description { get; set; }
    public string[] HostResource { get; set; }
    public bool IgnoreFlushes { get; set; }
    public string InstanceId { get; set; }
    public string IopsAllocationUnits { get; set; }
    public ulong IopsLimit { get; set; }
    public ulong IopsReservation { get; set; }
    public ulong Limit { get; set; }
    public string Parent { get; set; }
    public bool PersistentReservationsSupported { get; set; }
    public string PoolId { get; set; }
    public ulong Reservation { get; set; }
    public string ResourceSubType { get; set; }
    public ushort ResourceType { get; set; }
    public Guid SnapshotId { get; set; }
    public Guid StorageQoSPolicyId { get; set; }
    public ulong VirtualQuantity { get; set; }
    public string VirtualQuantityUnits { get; set; }
    public uint Weight { get; set; }
    public ushort WriteHardeningMethod { get; set; }
}