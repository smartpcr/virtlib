// -----------------------------------------------------------------------
// <copyright file="HostComputeSystem.vss.cs" company="Microsoft Corp.">
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
    private void UpdateVolumeShadownCopySetting(VirtualMachineDefinition vm, ManagementObject systemSettings)
    {
        if (vm.IntegrationServices.VolumeShadowCopy)
        {
            return;
        }

        using var volumeShadowCopySettings = systemSettings.GetRelated(VMQueries.GetVMSettingWmiClass(VMSetting.VolumeShadowCopy))
            .OfType<ManagementObject>().First();
        volumeShadowCopySettings["EnabledState"] = vm.IntegrationServices.VolumeShadowCopy ? 2 : 3;
        ModifyGuestServiceSettings(new[] { volumeShadowCopySettings }, out var modifiedVolumeShadowCopySettings);
        modifiedVolumeShadowCopySettings.DisposeCollection();
    }
}