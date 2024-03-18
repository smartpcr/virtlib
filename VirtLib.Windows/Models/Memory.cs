// -----------------------------------------------------------------------
// <copyright file="Memory.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

public class Memory
{
    public string AllocationUnits { get; set; }
    public bool AutomaticAllocation { get; set; }
    public bool AutomaticDeallocation { get; set; }
    public ushort ConsumerVisibility { get; set; }
    public string Description { get; set; }
    public bool DynamicMemoryEnabled { get; set; }
    public bool HugePagesEnabled { get; set; }
    public string InstanceId { get; set; }
    public bool IsVirtualized { get; set; }
    public ulong Limit { get; set; }
    public ulong MaxMemoryBlocksPerNumaNode { get; set; }
    public byte MemoryEncryptionPolicy { get; set; }
    public string PoolId { get; set; }
    public ulong Reservation { get; set; }
    public string ResourceSubType { get; set; }
    public ushort ResourceType { get; set; }
    public bool SgxEnabled { get; set; }
    public string SgxLaunchControlDefault { get; set; }
    public uint SgxLaunchControlMode { get; set; }
    public ulong SgxSize { get; set; }
    public bool SwapFilesInUse { get; set; }
    public uint TargetMemoryBuffer { get; set; }
    public ulong VirtualQuantity { get; set; }
    public string VirtualQuantityUnits { get; set; }
    public uint Weight { get; set; }
}