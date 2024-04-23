// -----------------------------------------------------------------------
// <copyright file="HostComputeSystem.validation.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows;

using System;
using System.IO;
using Definitions;

public partial class HostComputeSystem
{
    private void ValidateHardDisksNotExist(VirtualMachineDefinition vm)
    {
        foreach (var scsiController in vm.ScsiControllers)
        {
            foreach (var drive in scsiController.Drives)
            {
                if (drive is VirtualHardDrive virtualHardDrive)
                {
                    if (!string.IsNullOrWhiteSpace(virtualHardDrive.VirtualHardDisk.Path))
                    {
                        var diskPath = $@"\\{this.host}\{virtualHardDrive.VirtualHardDisk.Path.Replace(':', '$')}";
                        if (File.Exists(diskPath))
                        {
                            throw new ArgumentException($"Virtual Hard Disk already exists at {diskPath}.");
                        }
                    }
                }
            }
        }
    }

    private void ValidateVirtualSwitchExists(VirtualMachineDefinition vm)
    {
        foreach (var networkAdapter in vm.NetworkAdapters)
        {
            if (!string.IsNullOrWhiteSpace(networkAdapter.VirtualSwitch))
            {
                if (!this.IsVirtualSwitchExist(networkAdapter.VirtualSwitch))
                {
                    throw new ArgumentException($"Virtual Switch {networkAdapter.VirtualSwitch} does not exist.");
                }
            }
        }
    }
}