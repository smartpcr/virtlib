// -----------------------------------------------------------------------
// <copyright file="HostComputeSystem.dataexchange.cs" company="Microsoft Corp.">
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
    private void UpdateDataExchangeSetting(VirtualMachineDefinition vm, ManagementObject systemSettings)
    {
        if (vm.IntegrationServices.DataExchange)
        {
            return;
        }

        using var dataExchangeSettings = systemSettings.GetRelated(VMQueries.GetVMSettingWmiClass(VMSetting.DataExchange))
            .OfType<ManagementObject>().First();
        dataExchangeSettings["EnabledState"] = vm.IntegrationServices.DataExchange ? 2 : 3;
        ModifyGuestServiceSettings(new[] { dataExchangeSettings }, out var modifiedDataExchangeSettings);
        modifiedDataExchangeSettings.DisposeCollection();
    }
}