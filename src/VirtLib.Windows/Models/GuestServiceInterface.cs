// -----------------------------------------------------------------------
// <copyright file="GuestServiceInterface.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

using System;
using System.Management;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;

public class GuestServiceInterface
{
    private readonly ILogger<GuestServiceInterface> _logger;

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

    public GuestServiceInterface(IServiceProvider serviceProvider, ManagementObject guestServiceObj)
    {
        var loggerFactory = serviceProvider.GetRequiredService<ILoggerFactory>();
        this._logger = loggerFactory.CreateLogger<GuestServiceInterface>();
        this._logger.LogManagementObject(guestServiceObj);

        AllocationUnits = (string)guestServiceObj["AllocationUnits"];
        AutomaticAllocation = guestServiceObj["AutomaticAllocation"].ReadBool();
        AutomaticDeallocation = guestServiceObj["AutomaticDeallocation"].ReadBool();
        ConsumerVisibility = guestServiceObj["ConsumerVisibility"].ReadUInt16();
        DefaultEnabledStatePolicy = guestServiceObj["DefaultEnabledStatePolicy"].ReadUInt16();
        Description = (string)guestServiceObj["Description"];
        EnabledState = guestServiceObj["EnabledState"].ReadUInt16();
        InstanceId = (string)guestServiceObj["InstanceID"];
        Limit = guestServiceObj["Limit"].ReadUInt64();
        OtherResourceType = (string)guestServiceObj["OtherResourceType"];
        PoolId = (string)guestServiceObj["PoolID"];
        Reservation = guestServiceObj["Reservation"].ReadUInt64();
        ResourceSubType = (string)guestServiceObj["ResourceSubType"];
        ResourceType = guestServiceObj["ResourceType"].ReadUInt16();
        VirtualQuantity = guestServiceObj["VirtualQuantity"].ReadUInt64();
        VirtualQuantityUnits = (string)guestServiceObj["VirtualQuantityUnits"];
        Weight =  guestServiceObj["Weight"].ReadUInt32();
    }
}