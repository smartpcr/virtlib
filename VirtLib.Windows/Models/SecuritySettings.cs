// -----------------------------------------------------------------------
// <copyright file="SecuritySettings.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

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
}