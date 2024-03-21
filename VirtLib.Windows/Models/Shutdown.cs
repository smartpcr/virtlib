// -----------------------------------------------------------------------
// <copyright file="Shutdown.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

using System;
using System.Management;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;

public class Shutdown
{
    private readonly ILogger<Shutdown> _logger;

    public string AllocationUnits { get; set; }
    public bool AutomaticAllocation { get; set; }
    public bool AutomaticDeallocation { get; set; }
    public ushort ConsumerVisibility { get; set; }
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

    public Shutdown(IServiceProvider serviceProvider, ManagementObject shutdownObj)
    {
        var loggerFactory = serviceProvider.GetRequiredService<ILoggerFactory>();
        this._logger = loggerFactory.CreateLogger<Shutdown>();
        this._logger.LogManagementObject(shutdownObj);

        AllocationUnits = (string)shutdownObj["AllocationUnits"];
        AutomaticAllocation = shutdownObj["AutomaticAllocation"].ReadBool();
        AutomaticDeallocation = shutdownObj["AutomaticDeallocation"].ReadBool();
        ConsumerVisibility = shutdownObj["ConsumerVisibility"].ReadUInt16();
        Description = (string)shutdownObj["Description"];
        EnabledState = shutdownObj["EnabledState"].ReadUInt16();
        InstanceId = (string)shutdownObj["InstanceID"];
        Limit = shutdownObj["Limit"].ReadUInt64();
        OtherResourceType = (string)shutdownObj["OtherResourceType"];
        PoolId = (string)shutdownObj["PoolID"];
        Reservation = shutdownObj["Reservation"].ReadUInt64();
        ResourceSubType = (string)shutdownObj["ResourceSubType"];
        ResourceType = shutdownObj["ResourceType"].ReadUInt16();
        VirtualQuantity = shutdownObj["VirtualQuantity"].ReadUInt64();
        VirtualQuantityUnits = (string)shutdownObj["VirtualQuantityUnits"];
        Weight =  shutdownObj["Weight"].ReadUInt32();
    }
}