// -----------------------------------------------------------------------
// <copyright file="Processor.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

using System;
using System.Management;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;

public class Processor
{
    private readonly ILogger<Processor> _logger;

    public string AllocationUnits { get; set; }
    public bool AllowACountMCount { get; set; }
    public byte ApicMode { get; set; }
    public bool AutomaticAllocation { get; set; }
    public bool AutomaticDeallocation { get; set; }
    public string Caption { get; set; }
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

    public Processor(IServiceProvider serviceProvider, ManagementObject processorObj)
    {
        var loggerFactory = serviceProvider.GetRequiredService<ILoggerFactory>();
        this._logger = loggerFactory.CreateLogger<Processor>();
        this._logger.LogManagementObject(processorObj);

        AllocationUnits = (string)processorObj["AllocationUnits"];
        AllowACountMCount = processorObj["AllowACountMCount"].ReadBool();
        ApicMode = (byte)processorObj["ApicMode"];
        AutomaticAllocation = processorObj["AutomaticAllocation"].ReadBool();
        AutomaticDeallocation = processorObj["AutomaticDeallocation"].ReadBool();
        Caption = (string)processorObj["Caption"];
        ConsumerVisibility = processorObj["ConsumerVisibility"].ReadUInt16();
        CpuBrandString = (string)processorObj["CpuBrandString"];
        CpuGroupId = processorObj["CpuGroupId"].ReadGuid();
        Description = (string)processorObj["Description"];
        DisableSpeculationControls = processorObj["DisableSpeculationControls"].ReadBool();
        EnableHostResourceProtection = processorObj["EnableHostResourceProtection"].ReadBool();
        EnableLegacyApicMode = processorObj["EnableLegacyApicMode"].ReadBool();
        EnablePageShattering = (byte)processorObj["EnablePageShattering"];
        EnablePerfmonArchPmu = processorObj["EnablePerfmonArchPmu"].ReadBool();
        EnablePerfmonIpt = processorObj["EnablePerfmonIpt"].ReadBool();
        EnablePerfmonLbr = processorObj["EnablePerfmonLbr"].ReadBool();
        EnablePerfmonPebs = processorObj["EnablePerfmonPebs"].ReadBool();
        EnablePerfmonPmu = processorObj["EnablePerfmonPmu"].ReadBool();
        EnableSocketTopology = processorObj["EnableSocketTopology"].ReadBool();
        ExposeVirtualizationExtensions = processorObj["ExposeVirtualizationExtensions"].ReadBool();
        ExtendedVirtualizationExtensions =  processorObj["ExtendedVirtualizationExtensions"].ReadUInt32();
        HideHypervisorPresent = processorObj["HideHypervisorPresent"].ReadBool();
        HwThreadsPerCore = processorObj["HwThreadsPerCore"].ReadUInt64();
        InstanceId = (string)processorObj["InstanceID"];
        L3CacheWays =  processorObj["L3CacheWays"].ReadUInt32();
        L3ProcessorDistributionPolicy = (byte)processorObj["L3ProcessorDistributionPolicy"];
        Limit = processorObj["Limit"].ReadUInt64();
        LimitCpuId = processorObj["LimitCPUID"].ReadBool();
        LimitProcessorFeatures = processorObj["LimitProcessorFeatures"].ReadBool();
        LimitProcessorFeaturesMode = (byte)processorObj["LimitProcessorFeaturesMode"];
        MaxClusterCountPerSocket =  processorObj["MaxClusterCountPerSocket"].ReadUInt32();
        MaxHwIsolatedGuests =  processorObj["MaxHWIsolatedGuests"].ReadUInt32();
        MaxNumaNodesPerSocket = processorObj["MaxNumaNodesPerSocket"].ReadUInt64();
        MaxProcessorCountPerL3 =  processorObj["MaxProcessorCountPerL3"].ReadUInt32();
        MaxProcessorsPerNumaNode = processorObj["MaxProcessorsPerNumaNode"].ReadUInt64();
        PerfCpuFreqCapMhz =  processorObj["PerfCPUFreqCapMHz"].ReadUInt32();
        PoolId = (string)processorObj["PoolID"];
        Reservation = processorObj["Reservation"].ReadUInt64();
        ResourceSubType = (string)processorObj["ResourceSubType"];
        ResourceType = processorObj["ResourceType"].ReadUInt16();
        VirtualQuantity = processorObj["VirtualQuantity"].ReadUInt64();
        VirtualQuantityUnits = (string)processorObj["VirtualQuantityUnits"];
        Weight =  processorObj["Weight"].ReadUInt32();
    }
}