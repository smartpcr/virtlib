// -----------------------------------------------------------------------
// <copyright file="SecuritySettings.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

using System.Management;

public class SecuritySettings
{
    public bool AppContainerLaunchOptOut { get; set; }
    public bool BindToHostTpm { get; set; }
    public bool DataProtectionRequested { get; set; }
    public string Description { get; set; }
    public bool EncryptStateAndVmMigrationTraffic { get; set; }
    public string InstanceId { get; set; }
    public bool KsdEnabled { get; set; }
    public bool ShieldingRequested { get; set; }
    public bool TpmEnabled { get; set; }
    public bool VirtualizationBasedSecurityOptOut { get; set; }

    public SecuritySettings(ManagementObject securitySettingObj)
    {
        HcsLogger.LogManagementObject(securitySettingObj);

        AppContainerLaunchOptOut = (bool)securitySettingObj["AppContainerLaunchOptOut"];
        BindToHostTpm = (bool)securitySettingObj["BindToHostTpm"];
        DataProtectionRequested = (bool)securitySettingObj["DataProtectionRequested"];
        Description = (string)securitySettingObj["Description"];
        EncryptStateAndVmMigrationTraffic = (bool)securitySettingObj["EncryptStateAndVmMigrationTraffic"];
        InstanceId = (string)securitySettingObj["InstanceID"];
        KsdEnabled = (bool)securitySettingObj["KsdEnabled"];
        ShieldingRequested = (bool)securitySettingObj["ShieldingRequested"];
        TpmEnabled = (bool)securitySettingObj["TpmEnabled"];
        VirtualizationBasedSecurityOptOut = (bool)securitySettingObj["VirtualizationBasedSecurityOptOut"];
    }
}