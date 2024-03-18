// -----------------------------------------------------------------------
// <copyright file="EthernetSwitchPortOffloadSettings.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

using System.Management;

public class EthernetSwitchPortOffloadSettings
{
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

    public EthernetSwitchPortOffloadSettings(ManagementObject offloadObj)
    {
        HcsLogger.LogManagementObject(offloadObj);

        Description = (string)offloadObj["Description"];
        InstanceId = (string)offloadObj["InstanceID"];
        IovInterruptModeration = (uint)offloadObj["IovInterruptModeration"];
        IovOffloadWeight = (uint)offloadObj["IovOffloadWeight"];
        IovQueuePairsRequested = (uint)offloadObj["IovQueuePairsRequested"];
        IpSecOffloadLimit = (uint)offloadObj["IpSecOffloadLimit"];
        PacketDirectModerationCount = (uint)offloadObj["PacketDirectModerationCount"];
        PacketDirectModerationInterval = (uint)offloadObj["PacketDirectModerationInterval"];
        PacketDirectNumProcs = (uint)offloadObj["PacketDirectNumProcs"];
        RscEnabled = (bool)offloadObj["RscEnabled"];
        VmmqEnabled = (bool)offloadObj["VmmqEnabled"];
        VmqOffloadWeight = (uint)offloadObj["VmqOffloadWeight"];
        VrssEnabled = (bool)offloadObj["VrssEnabled"];
        VrssExcludePrimaryProcessor = (bool)offloadObj["VrssExcludePrimaryProcessor"];
        VrssIndependentHostSpreading = (bool)offloadObj["VrssIndependentHostSpreading"];
        VrssMinQueuePairs = (uint)offloadObj["VrssMinQueuePairs"];
        VrssQueueSchedulingMode = (uint)offloadObj["VrssQueueSchedulingMode"];
        VrssVmbusChannelAffinityPolicy = (uint)offloadObj["VrssVmbusChannelAffinityPolicy"];
    }
}