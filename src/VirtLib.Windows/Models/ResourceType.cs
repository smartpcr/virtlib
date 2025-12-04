// -----------------------------------------------------------------------
// <copyright file="ResourceType.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

using System;
using System.Management;

public enum ResourceType : ushort
{
    Other = 1,
    ComputeSystem = 2,
    Processor = 3,
    Memory = 4,
    IDEController = 5,
    ParallelSCSIHBA = 6,
    FCHBA = 7,
    ISCSIHBA = 8,
    IBHCA = 9,
    EthernetAdapter = 10,
    OtherNetworkAdapter = 11,
    IOSlot = 12,
    IODevice = 13,
    FloppyDrive = 14,
    CDDrive = 15,
    DVDDrive = 16,
    DiskDrive = 17,
    TapeDrive = 18,
    StorageExtent = 19,
    OtherStorageDevice = 20,
    SerialPort = 21,
    ParallelPort = 22,
    USBController = 23,
    GraphicsController = 24,
    Ieee1394Controller = 25,
    PartitionableUnit = 26,
    BasePartitionableUnit = 27,
    PowerSupply = 28,
    CoolingCapacity = 29,
    EthernetSwitchPort = 30,
    LogicalDisk = 31,
    StorageVolume = 32,
    EthernetConnection = 33,
    DMTFReserved = 34,
    VendorReserved = 35,
}

public static class ResourceTypeEx
{
    public static ResourceType ReadResourceType(this object value)
    {
        if (value is ushort val)
        {
            return val switch
            {
                1 => ResourceType.Other,
                2 => ResourceType.ComputeSystem,
                3 => ResourceType.Processor,
                4 => ResourceType.Memory,
                5 => ResourceType.IDEController,
                6 => ResourceType.ParallelSCSIHBA,
                7 => ResourceType.FCHBA,
                8 => ResourceType.ISCSIHBA,
                9 => ResourceType.IBHCA,
                10 => ResourceType.EthernetAdapter,
                11 => ResourceType.OtherNetworkAdapter,
                12 => ResourceType.IOSlot,
                13 => ResourceType.IODevice,
                14 => ResourceType.FloppyDrive,
                15 => ResourceType.CDDrive,
                16 => ResourceType.DVDDrive,
                17 => ResourceType.DiskDrive,
                18 => ResourceType.TapeDrive,
                19 => ResourceType.StorageExtent,
                20 => ResourceType.OtherStorageDevice,
                21 => ResourceType.SerialPort,
                22 => ResourceType.ParallelPort,
                23 => ResourceType.USBController,
                24 => ResourceType.GraphicsController,
                25 => ResourceType.Ieee1394Controller,
                26 => ResourceType.PartitionableUnit,
                27 => ResourceType.BasePartitionableUnit,
                28 => ResourceType.PowerSupply,
                29 => ResourceType.CoolingCapacity,
                30 => ResourceType.EthernetSwitchPort,
                31 => ResourceType.LogicalDisk,
                32 => ResourceType.StorageVolume,
                33 => ResourceType.EthernetConnection,
                34 => ResourceType.DMTFReserved,
                35 => ResourceType.VendorReserved,
                _ => ResourceType.Other,
            };
        }

        return ResourceType.Other;
    }
}
