// -----------------------------------------------------------------------
// <copyright file="HardDiskImage.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

using System;
using System.Management;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;

public class HardDiskImage
{
    private readonly ILogger<HardDiskImage> _logger;

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

    public HardDiskImage(IServiceProvider serviceProvider, ManagementObject imageObject)
    {
        var loggerFactory = serviceProvider.GetRequiredService<ILoggerFactory>();
        this._logger = loggerFactory.CreateLogger<HardDiskImage>();
        this._logger.LogManagementObject(imageObject);

        AllocationUnits = (string)imageObject["AllocationUnits"];
        AutomaticAllocation = imageObject["AutomaticAllocation"].ReadBool();
        AutomaticDeallocation = imageObject["AutomaticDeallocation"].ReadBool();
        CachingMode = imageObject["CachingMode"].ReadUInt16();
        ConsumerVisibility = imageObject["ConsumerVisibility"].ReadUInt16();
        Description = (string)imageObject["Description"];
        HostResource = (string[])imageObject["HostResource"];
        IgnoreFlushes = imageObject["IgnoreFlushes"].ReadBool();
        InstanceId = (string)imageObject["InstanceID"];
        IopsAllocationUnits = (string)imageObject["IOPSAllocationUnits"];
        IopsLimit = imageObject["IOPSLimit"].ReadUInt64();
        IopsReservation = imageObject["IOPSReservation"].ReadUInt64();
        Limit = imageObject["Limit"].ReadUInt64();
        Parent = (string)imageObject["Parent"];
        PersistentReservationsSupported = imageObject["PersistentReservationsSupported"].ReadBool();
        PoolId = (string)imageObject["PoolID"];
        Reservation = imageObject["Reservation"].ReadUInt64();
        ResourceSubType = (string)imageObject["ResourceSubType"];
        ResourceType = imageObject["ResourceType"].ReadUInt16();
        SnapshotId =  imageObject["SnapshotId"].ReadGuid();
        StorageQoSPolicyId =  imageObject["StorageQoSPolicyID"].ReadGuid();
        VirtualQuantity = imageObject["VirtualQuantity"].ReadUInt64();
        VirtualQuantityUnits = (string)imageObject["VirtualQuantityUnits"];
        Weight =  imageObject["Weight"].ReadUInt32();
        WriteHardeningMethod = imageObject["WriteHardeningMethod"].ReadUInt16();
    }
}