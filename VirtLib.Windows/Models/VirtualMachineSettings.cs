// -----------------------------------------------------------------------
// <copyright file="VirtualMachineSettings.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

using System;
using System.Collections.Generic;
using System.Linq;
using System.Management;
using Queries;

public class VirtualMachineSettings
{
    public bool AllowReducedFcRedundancy { get; set; }
    public string Architecture { get; set; }
    public bool AutomaticSnapshotsEnabled { get; set; }
    public string BaseBoardSerialNumber { get; set; }
    public Guid BiosGuid { get; set; }
    public bool BiosNumLock { get; set; }
    public string BiosSerialNumber { get; set; }
    public ushort[] BootOrder { get; set; }
    public string[] BootSourceOrder { get; set; }
    public bool BootPciExpress { get; set; }
    public string BootPciExpressInstanceFilter { get; set; }
    public string ChassisAssetTag { get; set; }
    public string ChassisSerialNumber { get; set; }
    public ushort ClusterWideNodeCapabilitiesValidationMode { get; set; }
    public string ConfigurationDataRoot { get; set; }
    public string ConfigurationFile { get; set; }
    public Guid ConfigurationId { get; set; }
    public ushort ConsoleMode { get; set; }
    public DateTime? CreationTime { get; set; }
    public string Description { get; set; }
    public string ElementName { get; set; }
    public bool EnableHibernation { get; set; }
    public ushort EnhancedSessionTransportType { get; set; }
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
    public ushort NetworkBootPreferredProtocol { get; set; }
    public string[] Notes { get; set; }
    public bool PauseAfterBootFailure { get; set; }
    public bool SecureBootEnabled { get; set; }
    public string SecureBootTemplateId { get; set; }
    public string SuspendDataRoot { get; set; }
    public ushort UserSnapshotType { get; set; }
    public string Version { get; set; }
    public bool VirtualNumaEnabled { get; set; }
    public ushort VirtualSlitType { get; set; }
    public Guid VirtualSystemIdentifier { get; set; }
    public string VirtualSystemSubType { get; set; }
    public string VirtualSystemType { get; set; }

    public Processor Processor { get; set; }
    public Memory Memory { get; set; }
    public Shutdown? Shutdown { get; set; }
    public Heartbeat? Heartbeat { get; set; }
    public VolumeShadowCopy? VSS { get; set; }
    public GuestServiceInterface? GuestServiceInterface { get; set; }
    public SecuritySettings? SecuritySettings { get; set; }
    public List<HardDiskImage> HardDiskImages { get; set; } = new List<HardDiskImage>();
    public List<EthernetPort> EthernetPorts { get; set; } = new List<EthernetPort>();
    public List<EthernetSwitchPort> SwitchPorts { get; set; } = new List<EthernetSwitchPort>();
    public List<ResourceAllocation> Allocations { get; set; } = new List<ResourceAllocation>();

    public VirtualMachineSettings(ManagementObject vmSettingsObj)
    {
        HcsLogger.LogManagementObject(vmSettingsObj);
        AllowReducedFcRedundancy = (bool)vmSettingsObj["AllowReducedFcRedundancy"];
        Architecture = (string)vmSettingsObj["Architecture"];
        AutomaticSnapshotsEnabled = (bool)vmSettingsObj["AutomaticSnapshotsEnabled"];
        BaseBoardSerialNumber = (string)vmSettingsObj["BaseBoardSerialNumber"];
        BiosGuid = Guid.Parse((string)vmSettingsObj["BIOSGUID"]);
        BiosNumLock = (bool)vmSettingsObj["BIOSNumLock"];
        BiosSerialNumber = (string)vmSettingsObj["BIOSSerialNumber"];
        BootSourceOrder = (string[])vmSettingsObj["BootSourceOrder"];
        BootOrder = (ushort[])vmSettingsObj["BootOrder"];
        BootPciExpress = (bool)vmSettingsObj["BootPciExpress"];
        BootPciExpressInstanceFilter = (string)vmSettingsObj["BootPciExpressInstanceFilter"];
        ChassisAssetTag = (string)vmSettingsObj["ChassisAssetTag"];
        ChassisSerialNumber = (string)vmSettingsObj["ChassisSerialNumber"];
        ClusterWideNodeCapabilitiesValidationMode = (ushort)vmSettingsObj["ClusterWideNodeCapabilitiesValidationMode"];
        ConfigurationDataRoot = (string)vmSettingsObj["ConfigurationDataRoot"];
        ConfigurationFile = (string)vmSettingsObj["ConfigurationFile"];
        ConfigurationId = Guid.Parse((string)vmSettingsObj["ConfigurationID"]);
        ConsoleMode = (ushort)vmSettingsObj["ConsoleMode"];
        CreationTime = vmSettingsObj["CreationTime"].ReadDateTime();
        Description = (string)vmSettingsObj["Description"];
        ElementName = (string)vmSettingsObj["ElementName"];
        EnableHibernation = (bool)vmSettingsObj["EnableHibernation"];
        EnhancedSessionTransportType = (ushort)vmSettingsObj["EnhancedSessionTransportType"];
        GuestControlledCacheTypes = (bool)vmSettingsObj["GuestControlledCacheTypes"];
        GuestStateDataRoot = (string)vmSettingsObj["GuestStateDataRoot"];
        GuestStateFile = (string)vmSettingsObj["GuestStateFile"];
        GuestStateIsolationEnabled = (bool)vmSettingsObj["GuestStateIsolationEnabled"];
        GuestStateIsolationType = (ushort)vmSettingsObj["GuestStateIsolationType"];
        HighMmioGapBase = (ulong)vmSettingsObj["HighMmioGapBase"];
        HighMmioGapSize = (ulong)vmSettingsObj["HighMmioGapSize"];
        InstanceId = (string)vmSettingsObj["InstanceID"];
        IsAutomaticSnapshot = (bool)vmSettingsObj["IsAutomaticSnapshot"];
        IsSaved = (bool)vmSettingsObj["IsSaved"];
        LockOnDisconnect = (bool)vmSettingsObj["LockOnDisconnect"];
        LowMmioGapSize = (ulong)vmSettingsObj["LowMmioGapSize"];
        NetworkBootPreferredProtocol = (ushort)vmSettingsObj["NetworkBootPreferredProtocol"];
        Notes = (string[])vmSettingsObj["Notes"];
        PauseAfterBootFailure = (bool)vmSettingsObj["PauseAfterBootFailure"];
        SecureBootEnabled = (bool)vmSettingsObj["SecureBootEnabled"];
        SecureBootTemplateId = (string)vmSettingsObj["SecureBootTemplateId"];
        SuspendDataRoot = (string)vmSettingsObj["SuspendDataRoot"];
        UserSnapshotType = (ushort)vmSettingsObj["UserSnapshotType"];
        Version = (string)vmSettingsObj["Version"];
        VirtualNumaEnabled = (bool)vmSettingsObj["VirtualNumaEnabled"];
        VirtualSlitType = (ushort)vmSettingsObj["VirtualSlitType"];
        VirtualSystemIdentifier = Guid.Parse((string)vmSettingsObj["VirtualSystemIdentifier"]);
        VirtualSystemSubType = (string)vmSettingsObj["VirtualSystemSubType"];
        VirtualSystemType = (string)vmSettingsObj["VirtualSystemType"];

        using var processorCollection = vmSettingsObj.GetRelated(VMQueries.RelatedSettings.Processor);
        using var processorSettings = processorCollection.OfType<ManagementObject>().FirstOrDefault();
        if (processorSettings != null)
        {
            this.Processor = new Processor(processorSettings);
        }

        using var memoryCollection = vmSettingsObj.GetRelated(VMQueries.RelatedSettings.Memory);
        using var memorySettings = memoryCollection.OfType<ManagementObject>().FirstOrDefault();
        if (memorySettings != null)
        {
            this.Memory = new Memory(memorySettings);
        }

        using var shutdownCollection = vmSettingsObj.GetRelated(VMQueries.RelatedSettings.Shutdown);
        using var shutdownSettings = shutdownCollection.OfType<ManagementObject>().FirstOrDefault();
        if (shutdownSettings != null)
        {
            this.Shutdown = new Shutdown(shutdownSettings);
        }

        using var heartbeatCollection = vmSettingsObj.GetRelated(VMQueries.RelatedSettings.Heartbeat);
        using var heartbeatSettings = heartbeatCollection.OfType<ManagementObject>().FirstOrDefault();
        if (heartbeatSettings != null)
        {
            this.Heartbeat = new Heartbeat(heartbeatSettings);
        }

        using var vssCollection = vmSettingsObj.GetRelated(VMQueries.RelatedSettings.VolumeShadowCopy);
        using var vssSettings = vssCollection.OfType<ManagementObject>().FirstOrDefault();
        if (vssSettings != null)
        {
            this.VSS = new VolumeShadowCopy(vssSettings);
        }

        using var guestServiceCollection = vmSettingsObj.GetRelated(VMQueries.RelatedSettings.GuestServiceInterface);
        using var guestServicesSettings = guestServiceCollection.OfType<ManagementObject>().FirstOrDefault();
        if (guestServicesSettings != null)
        {
            this.GuestServiceInterface = new GuestServiceInterface(guestServicesSettings);
        }

        using var securityCollection = vmSettingsObj.GetRelated(VMQueries.RelatedSettings.Security);
        using var securitySettings = securityCollection.OfType<ManagementObject>().FirstOrDefault();
        if (securitySettings != null)
        {
            this.SecuritySettings = new SecuritySettings(securitySettings);
        }

        using var storageCollection = vmSettingsObj.GetRelated(VMQueries.RelatedSettings.Storage);
        foreach (var storageSettings in storageCollection)
        {
            using (storageSettings)
            {
                if (storageSettings is ManagementObject storageObj)
                {
                    this.HardDiskImages.Add(new HardDiskImage(storageObj));
                }
            }
        }

        using var virtualHardDiskCollection = vmSettingsObj.GetRelated(VMQueries.RelatedSettings.VirtualHardDisk);
        foreach (var virtualHardDisk in virtualHardDiskCollection)
        {
            using (virtualHardDisk)
            {
                if (virtualHardDisk is ManagementObject virtualHardDiskSettings)
                {
                    this.HardDiskImages.Add(new HardDiskImage(virtualHardDiskSettings));
                }
            }
        }

        using var syntheticPortCollection = vmSettingsObj.GetRelated(VMQueries.RelatedSettings.SyntheticEthernetPort);
        foreach (var syntheticPort in syntheticPortCollection)
        {
            using (syntheticPort)
            {
                if (syntheticPort is ManagementObject syntheticPortObj)
                {
                    this.EthernetPorts.Add(new EthernetPort(syntheticPortObj));
                }
            }
        }

        using var emulatedPortCollection = vmSettingsObj.GetRelated(VMQueries.RelatedSettings.EmulatedEthernetPort);
        foreach (var emulatedPort in emulatedPortCollection)
        {
            using (emulatedPort)
            {
                if (emulatedPort is ManagementObject emulatedPortObj)
                {
                    this.EthernetPorts.Add(new EthernetPort(emulatedPortObj));
                }
            }
        }

        using var ethernetPortAllocationCollection = vmSettingsObj.GetRelated(VMQueries.RelatedSettings.EthernetPortAllocation);
        using var switchPortSettings = ethernetPortAllocationCollection.OfType<ManagementObject>().FirstOrDefault();
        if (switchPortSettings != null)
        {
            this.SwitchPorts.Add(new EthernetSwitchPort(switchPortSettings));
        }

        using var snapshotCollection = vmSettingsObj.GetRelated(VMQueries.RelatedSettings.Sanpshot);
        foreach (var snapshot in snapshotCollection)
        {
            using (snapshot)
            {
                if (snapshot is ManagementObject snapshotObj)
                {
                    HcsLogger.LogManagementObject(snapshotObj);
                }
            }
        }

        using var resourceAllocCollection = vmSettingsObj.GetRelated(VMQueries.RelatedSettings.Resource);
        foreach (var resourceAllocation in resourceAllocCollection)
        {
            using (resourceAllocation)
            {
                if (resourceAllocation is ManagementObject resourceAllocationObj)
                {
                    this.Allocations.Add(new ResourceAllocation(resourceAllocationObj));
                }
            }
        }
    }
}