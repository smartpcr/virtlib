// -----------------------------------------------------------------------
// <copyright file="DynamicEthernetSwitchPort.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

using System;
using System.Collections.Generic;
using System.Management;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;
using Queries;

public class EthernetSwitchPort
{
    private readonly ILogger<EthernetSwitchPort> _logger;

    public string AllocationUnits { get; set; }
    public bool AutomaticAllocation { get; set; }
    public bool AutomaticDeallocation { get; set; }
    public string Caption { get; set; }
    public ushort ConsumerVisibility { get; set; }
    public string Description { get; set; }
    public string ElementName { get; set; }
    public ushort EnabledState { get; set; }
    public string[] HostResource { get; set; }
    public string InstanceId { get; set; }
    public string LastKnownSwitchName { get; set; }
    public ulong Limit { get; set; }
    public string Parent { get; set; }
    public string PoolId { get; set; }
    public string[] RequiredFeatureHints { get; set; }
    public string[] RequiredFeatures { get; set; }
    public ulong Reservation { get; set; }
    public string ResourceSubType { get; set; }
    public ushort ResourceType { get; set; }
    public string TestReplicaPoolId { get; set; }
    public string TestReplicaSwitchName { get; set; }
    public ulong VirtualQuantity { get; set; }
    public string VirtualQuantityUnits { get; set; }
    public uint Weight { get; set; }

    public List<EthernetSwitchPortOffloadSettings> OffloadSettings { get; set; } = new List<EthernetSwitchPortOffloadSettings>();

    public EthernetSwitchPort(IServiceProvider serviceProvider, ManagementObject ethernetObj)
    {
        var loggerFactory = serviceProvider.GetRequiredService<ILoggerFactory>();
        this._logger = loggerFactory.CreateLogger<EthernetSwitchPort>();
        this._logger.LogManagementObject(ethernetObj);

        AllocationUnits = (string)ethernetObj["AllocationUnits"];
        AutomaticAllocation = (bool)ethernetObj["AutomaticAllocation"];
        AutomaticDeallocation = (bool)ethernetObj["AutomaticDeallocation"];
        Caption = (string)ethernetObj["Caption"];
        ConsumerVisibility = (ushort)ethernetObj["ConsumerVisibility"];
        Description = (string)ethernetObj["Description"];
        ElementName = (string)ethernetObj["ElementName"];
        EnabledState = (ushort)ethernetObj["EnabledState"];
        HostResource = (string[])ethernetObj["HostResource"];
        InstanceId = (string)ethernetObj["InstanceID"];
        LastKnownSwitchName = (string)ethernetObj["LastKnownSwitchName"];
        Limit = (ulong)ethernetObj["Limit"];
        Parent = (string)ethernetObj["Parent"];
        PoolId = (string)ethernetObj["PoolID"];
        RequiredFeatureHints = (string[])ethernetObj["RequiredFeatureHints"];
        RequiredFeatures = (string[])ethernetObj["RequiredFeatures"];
        Reservation = (ulong)ethernetObj["Reservation"];
        ResourceSubType = (string)ethernetObj["ResourceSubType"];
        ResourceType = (ushort)ethernetObj["ResourceType"];
        TestReplicaPoolId = (string)ethernetObj["TestReplicaPoolID"];
        TestReplicaSwitchName = (string)ethernetObj["TestReplicaSwitchName"];
        VirtualQuantity = (ulong)ethernetObj["VirtualQuantity"];
        VirtualQuantityUnits = (string)ethernetObj["VirtualQuantityUnits"];
        Weight = (uint)ethernetObj["Weight"];

        using var portOffloadCollection = ethernetObj.GetRelated(VMQueries.GetVMSettingWmiClass(VMSetting.EthernetPortOffload));
        foreach (var offload in portOffloadCollection)
        {
            using (offload)
            {
                if (offload is ManagementObject offloadObj)
                {
                    this.OffloadSettings.Add(new EthernetSwitchPortOffloadSettings(serviceProvider, offloadObj));
                }
            }
        }
    }
}