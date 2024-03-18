// -----------------------------------------------------------------------
// <copyright file="GuestServiceInterface.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

using System.Management;

public class GuestServiceInterface
{
    public string AllocationUnits { get; set; }
    public bool AutomaticAllocation { get; set; }
    public bool AutomaticDeallocation { get; set; }
    public ushort ConsumerVisibility { get; set; }
    public ushort DefaultEnabledStatePolicy { get; set; }
    public string Description { get; set; }
    public ushort EnabledState { get; set; }
    public string InstanceId { get; set; }
    public ulong Limit { get; set; }
    public string OtherResourceType { get; set; }
    public string PoolId { get; set; }
    public ulong Reservation { get; set; }
    public string ResourceSubType { get; set; }
    public ushort ResourceType { get; set; }
    public ulong VirtualQuantity { get; set; }
    public string VirtualQuantityUnits { get; set; }
    public uint Weight { get; set; }

    public GuestServiceInterface(ManagementObject guestServiceObj)
    {
        HcsLogger.LogManagementObject(guestServiceObj);

        AllocationUnits = (string)guestServiceObj["AllocationUnits"];
        AutomaticAllocation = (bool)guestServiceObj["AutomaticAllocation"];
        AutomaticDeallocation = (bool)guestServiceObj["AutomaticDeallocation"];
        ConsumerVisibility = (ushort)guestServiceObj["ConsumerVisibility"];
        DefaultEnabledStatePolicy = (ushort)guestServiceObj["DefaultEnabledStatePolicy"];
        Description = (string)guestServiceObj["Description"];
        EnabledState = (ushort)guestServiceObj["EnabledState"];
        InstanceId = (string)guestServiceObj["InstanceID"];
        Limit = (ulong)guestServiceObj["Limit"];
        OtherResourceType = (string)guestServiceObj["OtherResourceType"];
        PoolId = (string)guestServiceObj["PoolID"];
        Reservation = (ulong)guestServiceObj["Reservation"];
        ResourceSubType = (string)guestServiceObj["ResourceSubType"];
        ResourceType = (ushort)guestServiceObj["ResourceType"];
        VirtualQuantity = (ulong)guestServiceObj["VirtualQuantity"];
        VirtualQuantityUnits = (string)guestServiceObj["VirtualQuantityUnits"];
        Weight = (uint)guestServiceObj["Weight"];
    }
}