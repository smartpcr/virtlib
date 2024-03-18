// -----------------------------------------------------------------------
// <copyright file="VirtualMachineSettings.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

using System;

public class VirtualMachineSettings
{
    public string Architecture { get; set; }
    public bool AutomaticSnapshotsEnabled { get; set; }
    public string BaseBoardSerialNumber { get; set; }
    public Guid BiosGuid { get; set; }
    public bool BiosNumLock { get; set; }
    public string BiosSerialNumber { get; set; }
    public uint[] BootOrder { get; set; }
    public bool BootPciExpress { get; set; }
    public string[] BootSourceOrder { get; set; }
    public string ChassisAssetTag { get; set; }
    public string ChassisSerialNumber { get; set; }
    public string ConfigurationDataRoot { get; set; }
    public string ConfigurationFile { get; set; }
    public string ConfigurationId { get; set; }
    public uint ConsoleMode { get; set; }
    public DateTime DCreationTime { get; set; }
    public string Description { get; set; }
    public string ElementName { get; set; }
    public bool EnableHibernation { get; set; }
    public uint EnhancedSessionTransportType { get; set; }
    public bool GuestControlledCacheTypes { get; set; }
    public string GuestStateDataRoot { get; set; }
    public string GuestStateFile { get; set; }
    public bool GuestStateIsolationEnabled { get; set; }
    public ushort GuestStateIsolationType { get; set; }
    public ulong HighMmioGapBase { get; set; }
    public ulong HighMmioGapSize { get; set; }
    public string InstanceId { get; set; }
    public bool IsAutomaticSnapshot { get; set; }
    public bool IsSaved { get; set; }
    public bool LockOnDisconnect { get; set; }
    public ulong LowMmioGapSize { get; set; }
    public uint NetworkBootPreferredProtocol { get; set; }
    public bool PauseAfterBootFailure { get; set; }
    public bool SecureBootEnabled { get; set; }
    public string SecureBootTemplateId { get; set; }
    public string SuspendDataRoot { get; set; }
    public ushort UserSnapshotType { get; set; }
    public string Version { get; set; }
    public bool VirtualNumaEnabled { get; set; }
    public ushort VirtualSlitType { get; set; }
    public string VirtualSystemIdentifier { get; set; }
    public string VirtualSystemSubType { get; set; }
    public string VirtualSystemType { get; set; }
}