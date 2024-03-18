// -----------------------------------------------------------------------
// <copyright file="Processor.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

using System;

public class Processor
{
    public string AllocationUnits { get; set; }
    public bool AllowACountMCount { get; set; }
    public byte ApicMode { get; set; }
    public bool AutomaticAllocation { get; set; }
    public bool AutomaticDeallocation { get; set; }
    public ushort ConsumerVisibility { get; set; }
    public string CpuBrandString { get; set; }
    public Guid CpuGroupId { get; set; }
    public string Description { get; set; }
    public bool DisableSpeculationControls { get; set; }
    public bool EnableHostResourceProtection { get; set; }
    public bool EnableLegacyApicMode { get; set; }
    public byte EnablePageShattering { get; set; }
    public bool EnablePerfmonArchPmu { get; set; }
    public bool EnablePerfmonIpt { get; set; }
    public bool EnablePerfmonLbr { get; set; }
    public bool EnablePerfmonPebs { get; set; }
    public bool EnablePerfmonPmu { get; set; }
    public bool EnableSocketTopology { get; set; }
    public bool ExposeVirtualizationExtensions { get; set; }
    public uint ExtendedVirtualizationExtensions { get; set; }
    public bool HideHypervisorPresent { get; set; }
    public ulong HwThreadsPerCore { get; set; }
    public string InstanceId { get; set; }
    public uint L3CacheWays { get; set; }
    public byte L3ProcessorDistributionPolicy { get; set; }
    public ulong Limit { get; set; }
    public bool LimitCpuId { get; set; }
    public bool LimitProcessorFeatures { get; set; }
    public byte LimitProcessorFeaturesMode { get; set; }
    public uint MaxClusterCountPerSocket { get; set; }
    public uint MaxHwIsolatedGuests { get; set; }
    public ulong MaxNumaNodesPerSocket { get; set; }
    public uint MaxProcessorCountPerL3 { get; set; }
    public ulong MaxProcessorsPerNumaNode { get; set; }
    public uint PerfCpuFreqCapMhz { get; set; }
    public string PoolId { get; set; }
    public ulong Reservation { get; set; }
    public string ResourceSubType { get; set; }
    public ushort ResourceType { get; set; }
    public ulong VirtualQuantity { get; set; }
    public string VirtualQuantityUnits { get; set; }
    public uint Weight { get; set; }
}