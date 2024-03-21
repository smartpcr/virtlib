// -----------------------------------------------------------------------
// <copyright file="HostComputeSystem.time.cs" company="Microsoft Corp.">
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
    private void UpdateTimeSyncSetting(VirtualMachineDefinition vm, ManagementObject systemSettings)
    {
        if (vm.IntegrationServices.TimeSynchronisation)
        {
            return;
        }

        using var timeSynchronizationSettings = systemSettings.GetRelated(VMQueries.GetVMSettingWmiClass(VMSetting.TimeSynchronization))
            .OfType<ManagementObject>().First();
        timeSynchronizationSettings["EnabledState"] = vm.IntegrationServices.TimeSynchronisation ? 2 : 3;
        ModifyGuestServiceSettings(new[] { timeSynchronizationSettings }, out var modifiedTimeSynchronizationSettings);
        modifiedTimeSynchronizationSettings.DisposeCollection();
    }
}