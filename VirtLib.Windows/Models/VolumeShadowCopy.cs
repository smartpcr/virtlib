// -----------------------------------------------------------------------
// <copyright file="VolumeShadowCopy.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

using System;
using System.Management;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;

public class VolumeShadowCopy
{
    private readonly ILogger<VolumeShadowCopy> _logger;

    public string AllocationUnits { get; set; }
    public bool AutomaticAllocation { get; set; }
    public bool AutomaticDeallocation { get; set; }
    public string Caption { get; set; }
    public ushort ConsumerVisibility { get; set; }
    public string Description { get; set; }
    public string ElementName { get; set; }
    public ushort EnabledState { get; set; }
    public string InstanceId { get; set; }
    public ulong Limit { get; set; }
    public string OtherResourceType { get; set; }
    public string PoolId { get; set; }
    public ulong Reservation { get; set; }
    public ushort ResourceType { get; set; }
    public ulong VirtualQuantity { get; set; }
    public string VirtualQuantityUnits { get; set; }
    public uint Weight { get; set; }

    public VolumeShadowCopy(IServiceProvider serviceProvider, ManagementObject vssObj)
    {
        var loggerFactory = serviceProvider.GetRequiredService<ILoggerFactory>();
        this._logger = loggerFactory.CreateLogger<VolumeShadowCopy>();
        this._logger.LogManagementObject(vssObj);

        AllocationUnits = (string)vssObj["AllocationUnits"];
        AutomaticAllocation = vssObj["AutomaticAllocation"].ReadBool();
        AutomaticDeallocation = vssObj["AutomaticDeallocation"].ReadBool();
        Caption = (string)vssObj["Caption"];
        ConsumerVisibility = vssObj["ConsumerVisibility"].ReadUInt16();
        Description = (string)vssObj["Description"];
        ElementName = (string)vssObj["ElementName"];
        EnabledState = vssObj["EnabledState"].ReadUInt16();
        InstanceId = (string)vssObj["InstanceID"];
        Limit = vssObj["Limit"].ReadUInt64();
        OtherResourceType = (string)vssObj["OtherResourceType"];
        PoolId = (string)vssObj["PoolID"];
        Reservation = vssObj["Reservation"].ReadUInt64();
        ResourceType = vssObj["ResourceType"].ReadUInt16();
        VirtualQuantity = vssObj["VirtualQuantity"].ReadUInt64();
        VirtualQuantityUnits = (string)vssObj["VirtualQuantityUnits"];
        Weight =  vssObj["Weight"].ReadUInt32();
    }
}