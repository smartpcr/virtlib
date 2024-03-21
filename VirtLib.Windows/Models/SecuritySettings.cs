// -----------------------------------------------------------------------
// <copyright file="SecuritySettings.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

using System;
using System.Management;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;

public class SecuritySettings
{
    private readonly ILogger<SecuritySettings> _logger;

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

    public SecuritySettings(IServiceProvider serviceProvider, ManagementObject securitySettingObj)
    {
        var loggerFactory = serviceProvider.GetRequiredService<ILoggerFactory>();
        this._logger = loggerFactory.CreateLogger<SecuritySettings>();
        this._logger.LogManagementObject(securitySettingObj);

        AppContainerLaunchOptOut = securitySettingObj["AppContainerLaunchOptOut"].ReadBool();
        BindToHostTpm = securitySettingObj["BindToHostTpm"].ReadBool();
        DataProtectionRequested = securitySettingObj["DataProtectionRequested"].ReadBool();
        Description = (string)securitySettingObj["Description"];
        EncryptStateAndVmMigrationTraffic = securitySettingObj["EncryptStateAndVmMigrationTraffic"].ReadBool();
        InstanceId = (string)securitySettingObj["InstanceID"];
        KsdEnabled = securitySettingObj["KsdEnabled"].ReadBool();
        ShieldingRequested = securitySettingObj["ShieldingRequested"].ReadBool();
        TpmEnabled = securitySettingObj["TpmEnabled"].ReadBool();
        VirtualizationBasedSecurityOptOut = securitySettingObj["VirtualizationBasedSecurityOptOut"].ReadBool();
    }
}