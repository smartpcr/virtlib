// -----------------------------------------------------------------------
// <copyright file="Heartbeat.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

using System;
using System.Management;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;

public class Heartbeat
{
    private readonly ILogger<Heartbeat> _logger;

    public string AllocationUnits { get; set; }
    public bool AutomaticAllocation { get; set; }
    public bool AutomaticDeallocation { get; set; }
    public string Caption { get; set; }
    public ushort ConsumerVisibility { get; set; }
    public string Description { get; set; }
    public string ElementName { get; set; }
    public ushort EnabledState { get; set; }
    public uint ErrorThreshold { get; set; }
    public string InstanceId { get; set; }
    public uint Interval { get; set; }
    public uint Latency { get; set; }
    public ulong Limit { get; set; }
    public string OtherResourceType { get; set; }
    public string PoolId { get; set; }
    public ulong Reservation { get; set; }
    public ushort ResourceType { get; set; }
    public ulong VirtualQuantity { get; set; }
    public string VirtualQuantityUnits { get; set; }
    public uint Weight { get; set; }

    public Heartbeat(IServiceProvider serviceProvider, ManagementObject heartbeatObj)
    {
        var loggerFactory = serviceProvider.GetRequiredService<ILoggerFactory>();
        this._logger = loggerFactory.CreateLogger<Heartbeat>();
        this._logger.LogManagementObject(heartbeatObj);

        AllocationUnits = (string)heartbeatObj["AllocationUnits"];
        AutomaticAllocation = heartbeatObj["AutomaticAllocation"].ReadBool();
        AutomaticDeallocation = heartbeatObj["AutomaticDeallocation"].ReadBool();
        Caption = (string)heartbeatObj["Caption"];
        ConsumerVisibility = heartbeatObj["ConsumerVisibility"].ReadUInt16();
        Description = (string)heartbeatObj["Description"];
        ElementName = (string)heartbeatObj["ElementName"];
        EnabledState = heartbeatObj["EnabledState"].ReadUInt16();
        ErrorThreshold =  heartbeatObj["ErrorThreshold"].ReadUInt32();
        InstanceId = (string)heartbeatObj["InstanceID"];
        Interval =  heartbeatObj["Interval"].ReadUInt32();
        Latency =  heartbeatObj["Latency"].ReadUInt32();
        Limit = heartbeatObj["Limit"].ReadUInt64();
        OtherResourceType = (string)heartbeatObj["OtherResourceType"];
        PoolId = (string)heartbeatObj["PoolID"];
        Reservation = heartbeatObj["Reservation"].ReadUInt64();
        ResourceType = heartbeatObj["ResourceType"].ReadUInt16();
        VirtualQuantity = heartbeatObj["VirtualQuantity"].ReadUInt64();
        VirtualQuantityUnits = (string)heartbeatObj["VirtualQuantityUnits"];
        Weight =  heartbeatObj["Weight"].ReadUInt32();
    }
}