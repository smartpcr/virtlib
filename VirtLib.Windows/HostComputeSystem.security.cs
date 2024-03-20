// -----------------------------------------------------------------------
// <copyright file="HostComputeSystem.security.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows;

using System.Linq;
using System.Management;
using Definitions;
using Queries;

public partial class HostComputeSystem
{
    private void UpdateSecuritySettings(VirtualMachineDefinition vm, ManagementObject systemSettings)
    {
        var enableSecurity = vm.Security.TrustedPlatformModule ||
                             vm.Security.EncryptTraffic ||
                             vm.Security.Shielding;
        if (enableSecurity)
        {
            using var securitySettings = systemSettings.GetRelated(VMQueries.GetVMSettingWmiClass(VMSetting.Security))
                .OfType<ManagementObject>().First();
            securitySettings["TpmEnabled"] = vm.Security.TrustedPlatformModule;
            if (vm.Security.TrustedPlatformModule)
            {
                securitySettings["EncryptStateAndVmMigrationTraffic"] = vm.Security.EncryptTraffic;
            }

            securitySettings["ShieldingRequested"] = vm.Security.Shielding;
            byte[] localKeyProtector = NewByGuardians();
            SetKeyProtector(securitySettings, localKeyProtector);
            ModifySecuritySettings(securitySettings);
        }
    }

    private byte[] NewByGuardians()
    {
        ManagementObject? defaultGuardian = GetUntrustedGuardian();
        if (defaultGuardian == null)
        {
            NewByGenerateCertificates(out defaultGuardian);
        }

        using (defaultGuardian)
        using (ManagementObject keyProtector = CreateKeyProtector())
        using (ManagementBaseObject inputParameters = keyProtector.GetMethodParameters("NewByGuardians"))
        {
            inputParameters["Owner"] = defaultGuardian;
            inputParameters["AllowUntrustedRoot"] = true;
            using ManagementBaseObject outputParameters = keyProtector.InvokeMethod("NewByGuardians", inputParameters, null);
            JobOutputHelper.ValidateOutput(outputParameters);
            using ManagementBaseObject hgsKeyProtector = (ManagementBaseObject)outputParameters["cmdletOutput"];
            return (byte[])hgsKeyProtector["RawData"];
        }
    }

    private ManagementObject CreateKeyProtector()
    {
        using ManagementClass managementClass = new ManagementClass(VmWmiClasses.HgsKeyProtector);
        managementClass.Scope = hgsScope;
        return managementClass.CreateInstance();
    }

    private void NewByGenerateCertificates(out ManagementObject hgsGuardian)
    {
        using ManagementClass hgsGuardianClass = new ManagementClass("MSFT_HgsGuardian");
        hgsGuardianClass.Scope = hgsScope;
        using ManagementBaseObject inputParameters = hgsGuardianClass.GetMethodParameters("NewByGenerateCertificates");
        inputParameters["Name"] = "UntrustedGuardian";
        inputParameters["GenerateCertificates"] = true;
        using ManagementBaseObject outputParameters = hgsGuardianClass.InvokeMethod("NewByGenerateCertificates", inputParameters, null);
        JobOutputHelper.ValidateOutput(outputParameters);
        hgsGuardian = (ManagementObject)outputParameters["cmdletOutput"];
    }

    private ManagementObject? GetUntrustedGuardian()
    {
        ObjectQuery query = new ObjectQuery($"SELECT * FROM {VmWmiClasses.HgsGuardian} WHERE Name = \"UntrustedGuardian\"");
        using ManagementObjectSearcher searcher = new ManagementObjectSearcher(hgsScope, query);
        using ManagementObjectCollection collection = searcher.Get();
        return collection.Count == 0
            ? null
            : collection.OfType<ManagementObject>().First();
    }

    private void SetKeyProtector(ManagementObject securitySettings, byte[] keyProtector)
    {
        using ManagementBaseObject inputParameters = this.Ss.GetMethodParameters("SetKeyProtector");
        inputParameters["SecuritySettingData"] = securitySettings.GetText(TextFormat.WmiDtd20);
        inputParameters["KeyProtector"] = keyProtector;
        using ManagementBaseObject outputParameters = this.Ss.InvokeMethod("SetKeyProtector", inputParameters, null);
        JobOutputHelper.ValidateOutput(outputParameters);
    }

    private void ModifySecuritySettings(ManagementObject securitySettings)
    {
        using ManagementBaseObject inputParameters = this.Ss.GetMethodParameters("ModifySecuritySettings");
        inputParameters["SecuritySettingData"] = securitySettings.GetText(TextFormat.WmiDtd20);
        using ManagementBaseObject outputParameters = this.Ss.InvokeMethod("ModifySecuritySettings", inputParameters, null);
        JobOutputHelper.ValidateOutput(outputParameters);
    }

}