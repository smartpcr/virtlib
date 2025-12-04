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
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;
using Queries;

public class SystemSettings
{
    private readonly ILogger<SystemSettings> _logger;

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

    public SystemSettings(IServiceProvider serviceProvider, ManagementObject vmSettingsObj)
    {
        var loggerFactory = serviceProvider.GetRequiredService<ILoggerFactory>();
        this._logger = loggerFactory.CreateLogger<SystemSettings>();
        this._logger.LogManagementObject(vmSettingsObj);
        AllowReducedFcRedundancy = vmSettingsObj["AllowReducedFcRedundancy"].ReadBool();
        Architecture = (string)vmSettingsObj["Architecture"];
        AutomaticSnapshotsEnabled = vmSettingsObj["AutomaticSnapshotsEnabled"].ReadBool();
        BaseBoardSerialNumber = (string)vmSettingsObj["BaseBoardSerialNumber"];
        BiosGuid =  vmSettingsObj["BIOSGUID"].ReadGuid();
        BiosNumLock = vmSettingsObj["BIOSNumLock"].ReadBool();
        BiosSerialNumber = (string)vmSettingsObj["BIOSSerialNumber"];
        BootSourceOrder = (string[])vmSettingsObj["BootSourceOrder"];
        BootOrder = (ushort[])vmSettingsObj["BootOrder"];
        BootPciExpress = vmSettingsObj["BootPciExpress"].ReadBool();
        BootPciExpressInstanceFilter = (string)vmSettingsObj["BootPciExpressInstanceFilter"];
        ChassisAssetTag = (string)vmSettingsObj["ChassisAssetTag"];
        ChassisSerialNumber = (string)vmSettingsObj["ChassisSerialNumber"];
        ClusterWideNodeCapabilitiesValidationMode = vmSettingsObj["ClusterWideNodeCapabilitiesValidationMode"].ReadUInt16();
        ConfigurationDataRoot = (string)vmSettingsObj["ConfigurationDataRoot"];
        ConfigurationFile = (string)vmSettingsObj["ConfigurationFile"];
        ConfigurationId =  vmSettingsObj["ConfigurationID"].ReadGuid();
        ConsoleMode = vmSettingsObj["ConsoleMode"].ReadUInt16();
        CreationTime = vmSettingsObj["CreationTime"].ReadDateTime();
        Description = (string)vmSettingsObj["Description"];
        ElementName = (string)vmSettingsObj["ElementName"];
        EnableHibernation = vmSettingsObj["EnableHibernation"].ReadBool();
        EnhancedSessionTransportType = vmSettingsObj["EnhancedSessionTransportType"].ReadUInt16();
        GuestControlledCacheTypes = vmSettingsObj["GuestControlledCacheTypes"].ReadBool();
        GuestStateDataRoot = (string)vmSettingsObj["GuestStateDataRoot"];
        GuestStateFile = (string)vmSettingsObj["GuestStateFile"];
        GuestStateIsolationEnabled = vmSettingsObj["GuestStateIsolationEnabled"].ReadBool();
        GuestStateIsolationType = vmSettingsObj["GuestStateIsolationType"].ReadUInt16();
        HighMmioGapBase = vmSettingsObj["HighMmioGapBase"].ReadUInt64();
        HighMmioGapSize = vmSettingsObj["HighMmioGapSize"].ReadUInt64();
        InstanceId = (string)vmSettingsObj["InstanceID"];
        IsAutomaticSnapshot = vmSettingsObj["IsAutomaticSnapshot"].ReadBool();
        IsSaved = vmSettingsObj["IsSaved"].ReadBool();
        LockOnDisconnect = vmSettingsObj["LockOnDisconnect"].ReadBool();
        LowMmioGapSize = vmSettingsObj["LowMmioGapSize"].ReadUInt64();
        NetworkBootPreferredProtocol = vmSettingsObj["NetworkBootPreferredProtocol"].ReadUInt16();
        Notes = (string[])vmSettingsObj["Notes"];
        PauseAfterBootFailure = vmSettingsObj["PauseAfterBootFailure"].ReadBool();
        SecureBootEnabled = vmSettingsObj["SecureBootEnabled"].ReadBool();
        SecureBootTemplateId = (string)vmSettingsObj["SecureBootTemplateId"];
        SuspendDataRoot = (string)vmSettingsObj["SuspendDataRoot"];
        UserSnapshotType = vmSettingsObj["UserSnapshotType"].ReadUInt16();
        Version = (string)vmSettingsObj["Version"];
        VirtualNumaEnabled = vmSettingsObj["VirtualNumaEnabled"].ReadBool();
        VirtualSlitType = vmSettingsObj["VirtualSlitType"].ReadUInt16();
        VirtualSystemIdentifier =  vmSettingsObj["VirtualSystemIdentifier"].ReadGuid();
        VirtualSystemSubType = (string)vmSettingsObj["VirtualSystemSubType"];
        VirtualSystemType = (string)vmSettingsObj["VirtualSystemType"];

        using var processorCollection = vmSettingsObj.GetRelated(VMQueries.GetVMSettingWmiClass(VMSetting.Processor));
        using var processorSettings = processorCollection.OfType<ManagementObject>().FirstOrDefault();
        if (processorSettings != null)
        {
            this.Processor = new Processor(serviceProvider, processorSettings);
        }

        using var memoryCollection = vmSettingsObj.GetRelated(VMQueries.GetVMSettingWmiClass(VMSetting.Memory));
        using var memorySettings = memoryCollection.OfType<ManagementObject>().FirstOrDefault();
        if (memorySettings != null)
        {
            this.Memory = new Memory(serviceProvider, memorySettings);
        }

        using var shutdownCollection = vmSettingsObj.GetRelated(VMQueries.GetVMSettingWmiClass(VMSetting.Shutdown));
        using var shutdownSettings = shutdownCollection.OfType<ManagementObject>().FirstOrDefault();
        if (shutdownSettings != null)
        {
            this.Shutdown = new Shutdown(serviceProvider, shutdownSettings);
        }

        using var heartbeatCollection = vmSettingsObj.GetRelated(VMQueries.GetVMSettingWmiClass(VMSetting.Heartbeat));
        using var heartbeatSettings = heartbeatCollection.OfType<ManagementObject>().FirstOrDefault();
        if (heartbeatSettings != null)
        {
            this.Heartbeat = new Heartbeat(serviceProvider, heartbeatSettings);
        }

        using var vssCollection = vmSettingsObj.GetRelated(VMQueries.GetVMSettingWmiClass(VMSetting.VolumeShadowCopy));
        using var vssSettings = vssCollection.OfType<ManagementObject>().FirstOrDefault();
        if (vssSettings != null)
        {
            this.VSS = new VolumeShadowCopy(serviceProvider, vssSettings);
        }

        using var guestServiceCollection = vmSettingsObj.GetRelated(VMQueries.GetVMSettingWmiClass(VMSetting.GuestServiceInterface));
        using var guestServicesSettings = guestServiceCollection.OfType<ManagementObject>().FirstOrDefault();
        if (guestServicesSettings != null)
        {
            this.GuestServiceInterface = new GuestServiceInterface(serviceProvider, guestServicesSettings);
        }

        using var securityCollection = vmSettingsObj.GetRelated(VMQueries.GetVMSettingWmiClass(VMSetting.Security));
        using var securitySettings = securityCollection.OfType<ManagementObject>().FirstOrDefault();
        if (securitySettings != null)
        {
            this.SecuritySettings = new SecuritySettings(serviceProvider, securitySettings);
        }

        using var storageCollection = vmSettingsObj.GetRelated(VMQueries.GetVMSettingWmiClass(VMSetting.Storage));
        foreach (var storageSettings in storageCollection)
        {
            using (storageSettings)
            {
                if (storageSettings is ManagementObject storageObj)
                {
                    this.HardDiskImages.Add(new HardDiskImage(serviceProvider, storageObj));
                }
            }
        }

        using var virtualHardDiskCollection = vmSettingsObj.GetRelated(VMQueries.GetVMSettingWmiClass(VMSetting.VirtualHardDisk));
        foreach (var virtualHardDisk in virtualHardDiskCollection)
        {
            using (virtualHardDisk)
            {
                if (virtualHardDisk is ManagementObject virtualHardDiskSettings)
                {
                    this.HardDiskImages.Add(new HardDiskImage(serviceProvider, virtualHardDiskSettings));
                }
            }
        }

        using var syntheticPortCollection = vmSettingsObj.GetRelated(VMQueries.GetVMSettingWmiClass(VMSetting.SyntheticEthernetPort));
        foreach (var syntheticPort in syntheticPortCollection)
        {
            using (syntheticPort)
            {
                if (syntheticPort is ManagementObject syntheticPortObj)
                {
                    this.EthernetPorts.Add(new EthernetPort(serviceProvider, syntheticPortObj));
                }
            }
        }

        using var emulatedPortCollection = vmSettingsObj.GetRelated(VMQueries.GetVMSettingWmiClass(VMSetting.EmulatedEthernetPort));
        foreach (var emulatedPort in emulatedPortCollection)
        {
            using (emulatedPort)
            {
                if (emulatedPort is ManagementObject emulatedPortObj)
                {
                    this.EthernetPorts.Add(new EthernetPort(serviceProvider, emulatedPortObj));
                }
            }
        }

        using var ethernetPortAllocationCollection = vmSettingsObj.GetRelated(VMQueries.GetVMSettingWmiClass(VMSetting.EthernetPortAllocation));
        using var switchPortSettings = ethernetPortAllocationCollection.OfType<ManagementObject>().FirstOrDefault();
        if (switchPortSettings != null)
        {
            this.SwitchPorts.Add(new EthernetSwitchPort(serviceProvider, switchPortSettings));
        }

        using var snapshotCollection = vmSettingsObj.GetRelated(VMQueries.GetVMSettingWmiClass(VMSetting.Sanpshot));
        foreach (var snapshot in snapshotCollection)
        {
            using (snapshot)
            {
                if (snapshot is ManagementObject snapshotObj)
                {
                    this._logger.LogManagementObject(snapshotObj);
                }
            }
        }

        using var resourceAllocCollection = vmSettingsObj.GetRelated(VMQueries.GetVMSettingWmiClass(VMSetting.Resource));
        foreach (var resourceAllocation in resourceAllocCollection)
        {
            using (resourceAllocation)
            {
                if (resourceAllocation is ManagementObject resourceAllocationObj)
                {
                    this.Allocations.Add(new ResourceAllocation(serviceProvider, resourceAllocationObj));
                }
            }
        }
    }
}