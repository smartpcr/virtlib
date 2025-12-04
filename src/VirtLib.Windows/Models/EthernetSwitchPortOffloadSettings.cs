// -----------------------------------------------------------------------
// <copyright file="EthernetSwitchPortOffloadSettings.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

using System;
using System.Management;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;

public class EthernetSwitchPortOffloadSettings
{
    private readonly ILogger<EthernetSwitchPortOffloadSettings> _logger;

    public string Description { get; set; }
    public string InstanceId { get; set; }
    public uint IovInterruptModeration { get; set; }
    public uint IovOffloadWeight { get; set; }
    public uint IovQueuePairsRequested { get; set; }
    public uint IpSecOffloadLimit { get; set; }
    public uint PacketDirectModerationCount { get; set; }
    public uint PacketDirectModerationInterval { get; set; }
    public uint PacketDirectNumProcs { get; set; }
    public bool RscEnabled { get; set; }
    public bool VmmqEnabled { get; set; }
    public uint VmqOffloadWeight { get; set; }
    public bool VrssEnabled { get; set; }
    public bool VrssExcludePrimaryProcessor { get; set; }
    public bool VrssIndependentHostSpreading { get; set; }
    public uint VrssMinQueuePairs { get; set; }
    public uint VrssQueueSchedulingMode { get; set; }
    public uint VrssVmbusChannelAffinityPolicy { get; set; }

    public EthernetSwitchPortOffloadSettings(IServiceProvider serviceProvider, ManagementObject offloadObj)
    {
        var loggerFactory = serviceProvider.GetRequiredService<ILoggerFactory>();
        this._logger = loggerFactory.CreateLogger<EthernetSwitchPortOffloadSettings>();
        this._logger.LogManagementObject(offloadObj);

        Description = (string)offloadObj["Description"];
        InstanceId = (string)offloadObj["InstanceID"];
        IovInterruptModeration =  offloadObj["IovInterruptModeration"].ReadUInt32();
        IovOffloadWeight =  offloadObj["IovOffloadWeight"].ReadUInt32();
        IovQueuePairsRequested =  offloadObj["IovQueuePairsRequested"].ReadUInt32();
        IpSecOffloadLimit =  offloadObj["IpSecOffloadLimit"].ReadUInt32();
        PacketDirectModerationCount =  offloadObj["PacketDirectModerationCount"].ReadUInt32();
        PacketDirectModerationInterval =  offloadObj["PacketDirectModerationInterval"].ReadUInt32();
        PacketDirectNumProcs =  offloadObj["PacketDirectNumProcs"].ReadUInt32();
        RscEnabled = offloadObj["RscEnabled"].ReadBool();
        VmmqEnabled = offloadObj["VmmqEnabled"].ReadBool();
        VmqOffloadWeight =  offloadObj["VmqOffloadWeight"].ReadUInt32();
        VrssEnabled = offloadObj["VrssEnabled"].ReadBool();
        VrssExcludePrimaryProcessor = offloadObj["VrssExcludePrimaryProcessor"].ReadBool();
        VrssIndependentHostSpreading = offloadObj["VrssIndependentHostSpreading"].ReadBool();
        VrssMinQueuePairs =  offloadObj["VrssMinQueuePairs"].ReadUInt32();
        VrssQueueSchedulingMode =  offloadObj["VrssQueueSchedulingMode"].ReadUInt32();
        VrssVmbusChannelAffinityPolicy =  offloadObj["VrssVmbusChannelAffinityPolicy"].ReadUInt32();
    }
}