// -----------------------------------------------------------------------
// <copyright file="Processor.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

using System;
using System.Management;

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

    public Processor(ManagementObject processorObj)
    {
        HcsLogger.LogManagementObject(processorObj);

        AllocationUnits = (string)processorObj["AllocationUnits"];
        AllowACountMCount = (bool)processorObj["AllowACountMCount"];
        ApicMode = (byte)processorObj["ApicMode"];
        AutomaticAllocation = (bool)processorObj["AutomaticAllocation"];
        AutomaticDeallocation = (bool)processorObj["AutomaticDeallocation"];
        ConsumerVisibility = (ushort)processorObj["ConsumerVisibility"];
        CpuBrandString = (string)processorObj["CpuBrandString"];
        CpuGroupId = processorObj["CpuGroupId"].ReadGuid();
        Description = (string)processorObj["Description"];
        DisableSpeculationControls = (bool)processorObj["DisableSpeculationControls"];
        EnableHostResourceProtection = (bool)processorObj["EnableHostResourceProtection"];
        EnableLegacyApicMode = (bool)processorObj["EnableLegacyApicMode"];
        EnablePageShattering = (byte)processorObj["EnablePageShattering"];
        EnablePerfmonArchPmu = (bool)processorObj["EnablePerfmonArchPmu"];
        EnablePerfmonIpt = (bool)processorObj["EnablePerfmonIpt"];
        EnablePerfmonLbr = (bool)processorObj["EnablePerfmonLbr"];
        EnablePerfmonPebs = (bool)processorObj["EnablePerfmonPebs"];
        EnablePerfmonPmu = (bool)processorObj["EnablePerfmonPmu"];
        EnableSocketTopology = (bool)processorObj["EnableSocketTopology"];
        ExposeVirtualizationExtensions = (bool)processorObj["ExposeVirtualizationExtensions"];
        ExtendedVirtualizationExtensions = (uint)processorObj["ExtendedVirtualizationExtensions"];
        HideHypervisorPresent = (bool)processorObj["HideHypervisorPresent"];
        HwThreadsPerCore = (ulong)processorObj["HwThreadsPerCore"];
        InstanceId = (string)processorObj["InstanceID"];
        L3CacheWays = (uint)processorObj["L3CacheWays"];
        L3ProcessorDistributionPolicy = (byte)processorObj["L3ProcessorDistributionPolicy"];
        Limit = (ulong)processorObj["Limit"];
        LimitCpuId = (bool)processorObj["LimitCPUID"];
        LimitProcessorFeatures = (bool)processorObj["LimitProcessorFeatures"];
        LimitProcessorFeaturesMode = (byte)processorObj["LimitProcessorFeaturesMode"];
        MaxClusterCountPerSocket = (uint)processorObj["MaxClusterCountPerSocket"];
        MaxHwIsolatedGuests = (uint)processorObj["MaxHWIsolatedGuests"];
        MaxNumaNodesPerSocket = (ulong)processorObj["MaxNumaNodesPerSocket"];
        MaxProcessorCountPerL3 = (uint)processorObj["MaxProcessorCountPerL3"];
        MaxProcessorsPerNumaNode = (ulong)processorObj["MaxProcessorsPerNumaNode"];
        PerfCpuFreqCapMhz = (uint)processorObj["PerfCPUFreqCapMHz"];
        PoolId = (string)processorObj["PoolID"];
        Reservation = (ulong)processorObj["Reservation"];
        ResourceSubType = (string)processorObj["ResourceSubType"];
        ResourceType = (ushort)processorObj["ResourceType"];
        VirtualQuantity = (ulong)processorObj["VirtualQuantity"];
        VirtualQuantityUnits = (string)processorObj["VirtualQuantityUnits"];
        Weight = (uint)processorObj["Weight"];
    }
}