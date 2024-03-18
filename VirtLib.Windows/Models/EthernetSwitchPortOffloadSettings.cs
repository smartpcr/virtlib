// -----------------------------------------------------------------------
// <copyright file="EthernetSwitchPortOffloadSettings.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

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
    public bool VmqOffloadWeight { get; set; }
    public bool VrssEnabled { get; set; }
    public bool VrssExcludePrimaryProcessor { get; set; }
    public bool VrssIndependentHostSpreading { get; set; }
    public uint VrssMinQueuePairs { get; set; }
    public uint VrssQueueSchedulingMode { get; set; }
    public uint VrssVmbusChannelAffinityPolicy { get; set; }
}