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
        AutomaticAllocation = memoryObj["AutomaticAllocation"].ReadBool();
        AutomaticDeallocation = memoryObj["AutomaticDeallocation"].ReadBool();
        ConsumerVisibility = memoryObj["ConsumerVisibility"].ReadUInt16();
        Description = (string)memoryObj["Description"];
        DynamicMemoryEnabled = memoryObj["DynamicMemoryEnabled"].ReadBool();
        HugePagesEnabled = memoryObj["HugePagesEnabled"].ReadBool();
        InstanceId = (string)memoryObj["InstanceID"];
        IsVirtualized = memoryObj["IsVirtualized"].ReadBool();
        Limit = memoryObj["Limit"].ReadUInt64();
        MaxMemoryBlocksPerNumaNode = memoryObj["MaxMemoryBlocksPerNumaNode"].ReadUInt64();
        MemoryEncryptionPolicy = (byte)memoryObj["MemoryEncryptionPolicy"];
        PoolId = (string)memoryObj["PoolID"];
        Reservation = memoryObj["Reservation"].ReadUInt64();
        ResourceSubType = (string)memoryObj["ResourceSubType"];
        ResourceType = memoryObj["ResourceType"].ReadUInt16();
        SgxEnabled = memoryObj["SgxEnabled"].ReadBool();
        SgxLaunchControlDefault = (string)memoryObj["SgxLaunchControlDefault"];
        SgxLaunchControlMode =  memoryObj["SgxLaunchControlMode"].ReadUInt32();
        SgxSize = memoryObj["SgxSize"].ReadUInt64();
        SwapFilesInUse = memoryObj["SwapFilesInUse"].ReadBool();
        TargetMemoryBuffer =  memoryObj["TargetMemoryBuffer"].ReadUInt32();
        VirtualQuantity = memoryObj["VirtualQuantity"].ReadUInt64();
        VirtualQuantityUnits = (string)memoryObj["VirtualQuantityUnits"];
        Weight =  memoryObj["Weight"].ReadUInt32();
    }
}