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
        AutomaticAllocation = (bool)heartbeatObj["AutomaticAllocation"];
        AutomaticDeallocation = (bool)heartbeatObj["AutomaticDeallocation"];
        Caption = (string)heartbeatObj["Caption"];
        ConsumerVisibility = (ushort)heartbeatObj["ConsumerVisibility"];
        Description = (string)heartbeatObj["Description"];
        ElementName = (string)heartbeatObj["ElementName"];
        EnabledState = (ushort)heartbeatObj["EnabledState"];
        ErrorThreshold = (uint)heartbeatObj["ErrorThreshold"];
        InstanceId = (string)heartbeatObj["InstanceID"];
        Interval = (uint)heartbeatObj["Interval"];
        Latency = (uint)heartbeatObj["Latency"];
        Limit = (ulong)heartbeatObj["Limit"];
        OtherResourceType = (string)heartbeatObj["OtherResourceType"];
        PoolId = (string)heartbeatObj["PoolID"];
        Reservation = (ulong)heartbeatObj["Reservation"];
        ResourceType = (ushort)heartbeatObj["ResourceType"];
        VirtualQuantity = (ulong)heartbeatObj["VirtualQuantity"];
        VirtualQuantityUnits = (string)heartbeatObj["VirtualQuantityUnits"];
        Weight = (uint)heartbeatObj["Weight"];
    }
}