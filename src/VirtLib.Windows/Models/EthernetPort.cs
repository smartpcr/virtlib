// -----------------------------------------------------------------------
// <copyright file="EthernetPort.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

using System;
using System.Management;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;

public class EthernetPort
{
    private readonly ILogger<EthernetPort> _logger;

    public string Address { get; set; }
    public string AllocationUnits { get; set; }
    public bool AllowDirectTranslatedP2P { get; set; }
    public bool AllowPacketDirect { get; set; }
    public bool AutomaticAllocation { get; set; }
    public bool AutomaticDeallocation { get; set; }
    public bool ClusterMonitored { get; set; }
    public string[] Connection { get; set; }
    public ushort ConsumerVisibility { get; set; }
    public string Description { get; set; }
    public bool DeviceNamingEnabled { get; set; }
    public string InstanceId { get; set; }
    public bool InterruptModeration { get; set; }
    public ulong Limit { get; set; }
    public uint MediaType { get; set; }
    public bool NumaAwarePlacement { get; set; }
    public string PoolId { get; set; }
    public ulong Reservation { get; set; }
    public string ResourceSubType { get; set; }
    public ushort ResourceType { get; set; }
    public bool StaticMacAddress { get; set; }
    public ulong VirtualQuantity { get; set; }
    public string VirtualQuantityUnits { get; set; }
    public string[] VirtualSystemIdentifiers { get; set; }
    public uint Weight { get; set; }

    public EthernetPort(IServiceProvider serviceProvider, ManagementObject portObj)
    {
        var loggerFactory = serviceProvider.GetRequiredService<ILoggerFactory>();
        this._logger = loggerFactory.CreateLogger<EthernetPort>();
        this._logger.LogManagementObject(portObj);

        Address = (string)portObj["Address"];
        AllocationUnits = (string)portObj["AllocationUnits"];
        AllowDirectTranslatedP2P = portObj["AllowDirectTranslatedP2P"].ReadBool();
        AllowPacketDirect = portObj["AllowPacketDirect"].ReadBool();
        AutomaticAllocation = portObj["AutomaticAllocation"].ReadBool();
        AutomaticDeallocation = portObj["AutomaticDeallocation"].ReadBool();
        ClusterMonitored = portObj["ClusterMonitored"].ReadBool();
        Connection = (string[])portObj["Connection"];
        ConsumerVisibility = portObj["ConsumerVisibility"].ReadUInt16();
        Description = (string)portObj["Description"];
        DeviceNamingEnabled = portObj["DeviceNamingEnabled"].ReadBool();
        InstanceId = (string)portObj["InstanceID"];
        InterruptModeration = portObj["InterruptModeration"].ReadBool();
        Limit = portObj["Limit"].ReadUInt64();
        MediaType =  portObj["MediaType"].ReadUInt32();
        NumaAwarePlacement = portObj["NumaAwarePlacement"].ReadBool();
        PoolId = (string)portObj["PoolID"];
        Reservation = portObj["Reservation"].ReadUInt64();
        ResourceSubType = (string)portObj["ResourceSubType"];
        ResourceType = portObj["ResourceType"].ReadUInt16();
        StaticMacAddress = portObj["StaticMacAddress"].ReadBool();
        VirtualQuantity = portObj["VirtualQuantity"].ReadUInt64();
        VirtualQuantityUnits = (string)portObj["VirtualQuantityUnits"];
        VirtualSystemIdentifiers = (string[])portObj["VirtualSystemIdentifiers"];
        Weight =  portObj["Weight"].ReadUInt32();
    }
}