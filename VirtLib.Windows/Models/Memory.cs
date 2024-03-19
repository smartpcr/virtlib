// -----------------------------------------------------------------------
// <copyright file="Memory.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

using System;
using System.Management;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;

public class Memory
{
    private readonly ILogger<Memory> _logger;
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

    public Memory(IServiceProvider serviceProvider, ManagementObject memoryObj)
    {
        var loggerFactory = serviceProvider.GetRequiredService<ILoggerFactory>();
        this._logger = loggerFactory.CreateLogger<Memory>();
        this._logger.LogManagementObject(memoryObj);

        AllocationUnits = (string)memoryObj["AllocationUnits"];
        AutomaticAllocation = (bool)memoryObj["AutomaticAllocation"];
        AutomaticDeallocation = (bool)memoryObj["AutomaticDeallocation"];
        ConsumerVisibility = (ushort)memoryObj["ConsumerVisibility"];
        Description = (string)memoryObj["Description"];
        DynamicMemoryEnabled = (bool)memoryObj["DynamicMemoryEnabled"];
        HugePagesEnabled = (bool)memoryObj["HugePagesEnabled"];
        InstanceId = (string)memoryObj["InstanceID"];
        IsVirtualized = (bool)memoryObj["IsVirtualized"];
        Limit = (ulong)memoryObj["Limit"];
        MaxMemoryBlocksPerNumaNode = (ulong)memoryObj["MaxMemoryBlocksPerNumaNode"];
        MemoryEncryptionPolicy = (byte)memoryObj["MemoryEncryptionPolicy"];
        PoolId = (string)memoryObj["PoolID"];
        Reservation = (ulong)memoryObj["Reservation"];
        ResourceSubType = (string)memoryObj["ResourceSubType"];
        ResourceType = (ushort)memoryObj["ResourceType"];
        SgxEnabled = (bool)memoryObj["SgxEnabled"];
        SgxLaunchControlDefault = (string)memoryObj["SgxLaunchControlDefault"];
        SgxLaunchControlMode = (uint)memoryObj["SgxLaunchControlMode"];
        SgxSize = (ulong)memoryObj["SgxSize"];
        SwapFilesInUse = (bool)memoryObj["SwapFilesInUse"];
        TargetMemoryBuffer = (uint)memoryObj["TargetMemoryBuffer"];
        VirtualQuantity = (ulong)memoryObj["VirtualQuantity"];
        VirtualQuantityUnits = (string)memoryObj["VirtualQuantityUnits"];
        Weight = (uint)memoryObj["Weight"];
    }
}