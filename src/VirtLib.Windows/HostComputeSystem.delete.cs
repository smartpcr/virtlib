// -----------------------------------------------------------------------
// <copyright file="HostComputeSystem.delete.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows;

using System.IO;
using System.Linq;
using Queries;

public partial class HostComputeSystem
{
    public void DeleteVirtualMachine(string vmName, bool removeHardDisk)
    {
        var vm = this.GetVirtualMachine(vmName);
        if (vm == null)
        {
            this._logger.VMAlreadyDeleted(vmName);
            return;
        }

        using var inParams = this.Vmms.GetMethodParameters("DestroySystem");
        inParams["AffectedSystem"] = vm.ManagementPath;
        using var outParams = this.Vmms.InvokeMethod("DestroySystem", inParams, null);
        JobOutputHelper.ValidateOutput(outParams, this._logger);
        this._logger.VMDeleted(vmName, vm.ManagementPath);

        if (removeHardDisk)
        {
            var resourceSubtypeName = ResourceQueries.GetResourceSubtypeName(ResourceSubtype.VirtualHardDisk);
            var hardDisks = vm.Settings.SelectMany(s => s.HardDiskImages
                .Where(h => h.ResourceSubType == resourceSubtypeName)
                .SelectMany(h => h.HostResource)).Distinct().ToList();
            foreach (var hardDisk in hardDisks)
            {
                if (File.Exists(hardDisk))
                {
                    File.Delete(hardDisk);
                    this._logger.VMHardDiskDeleted(vmName, hardDisk);
                }
            }
        }
    }
}