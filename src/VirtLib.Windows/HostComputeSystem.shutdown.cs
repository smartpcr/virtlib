// -----------------------------------------------------------------------
// <copyright file="HostComputeSystem.shutdown.cs" company="Microsoft Corp.">
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
    private void UpdateShutdownSetting(VirtualMachineDefinition vm, ManagementObject systemSettings)
    {
        if (vm.IntegrationServices.Shutdown)
        {
            return;
        }

        using var shutdownSettings = systemSettings.GetRelated(VMQueries.GetVMSettingWmiClass(VMSetting.Shutdown))
            .OfType<ManagementObject>().First();
        shutdownSettings["EnabledState"] = vm.IntegrationServices.Shutdown
            ? 2 // Enabled
            : 3; // Disabled
        ModifyGuestServiceSettings(new[] { shutdownSettings }, out var modifiedShutdownSettings);
        modifiedShutdownSettings.DisposeCollection();
    }

    private void ModifyGuestServiceSettings(ManagementObject[] guestServiceSettings, out ManagementObject[] resultingGuestServiceSettings)
    {
        using ManagementBaseObject inputParameters = this.Vmms.GetMethodParameters("ModifyGuestServiceSettings");
        inputParameters["GuestServiceSettings"] = guestServiceSettings.ToStringArray();
        using ManagementBaseObject outputParameters = this.Vmms.InvokeMethod("ModifyGuestServiceSettings", inputParameters, null);
        JobOutputHelper.ValidateOutput(outputParameters, this._logger);
        resultingGuestServiceSettings = ((string[])outputParameters["ResultingGuestServiceSettings"]).ToObjectArray();
    }

}