// -----------------------------------------------------------------------
// <copyright file="HostComputeSystem.guestservice.cs" company="Microsoft Corp.">
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
    private void UpdateGuestServiceSetting(VirtualMachineDefinition vm, ManagementObject systemSettings)
    {
        if (vm.IntegrationServices.GuestServices)
        {
            return;
        }

        using var guestServiceSettings = systemSettings.GetRelated(VMQueries.GetVMSettingWmiClass(VMSetting.GuestServiceInterface))
            .OfType<ManagementObject>().First();
        guestServiceSettings["EnabledState"] = vm.IntegrationServices.GuestServices ? 2 : 3;
        ModifyGuestServiceSettings(new[] { guestServiceSettings }, out var modifiedGuestServiceSettings);
        modifiedGuestServiceSettings.DisposeCollection();
    }
}