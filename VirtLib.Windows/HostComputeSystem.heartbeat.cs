// -----------------------------------------------------------------------
// <copyright file="HostComputeSystem.heartbeat.cs" company="Microsoft Corp.">
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
    private void UpdateHeartbeatSetting(VirtualMachineDefinition vm, ManagementObject systemSettings)
    {
        if (vm.IntegrationServices.Heartbeat)
        {
            return;
        }

        using var heartbeatSettings = systemSettings.GetRelated(VMQueries.GetVMSettingWmiClass(VMSetting.Heartbeat))
            .OfType<ManagementObject>().First();
        heartbeatSettings["EnabledState"] = vm.IntegrationServices.Heartbeat ? 2 : 3;
        ModifyGuestServiceSettings(new[] { heartbeatSettings }, out var modifiedHeartbeatSettings);
        modifiedHeartbeatSettings.DisposeCollection();
    }
}