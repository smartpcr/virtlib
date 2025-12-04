// -----------------------------------------------------------------------
// <copyright file="HostComputeSystem.storage.cs" company="Microsoft Corp.">
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
    private void UpdateScsiControllers(VirtualMachineDefinition vm, ManagementObject systemSettings)
    {
        foreach (ScsiController scsiControllerDefinition in vm.ScsiControllers)
        {
            using var scsiControllerResource = ResourceQueries.CreateDefaultResource(ResourceSubtype.SCSIController, this.VirtualizationScope);
            this.Vmms.AddResourceSettings(this._logger, systemSettings, new[] { scsiControllerResource }, out ManagementObject[] scsiControllers);
            using var scsiController = scsiControllers.First();

            for (var address = 0; address < scsiControllerDefinition.Drives.Length; address++)
            {
                switch (scsiControllerDefinition.Drives[address])
                {
                    case VirtualHardDrive virtualHardDiskDefinition:
                        this.AddVirtualHardDisk(systemSettings, scsiController, virtualHardDiskDefinition, address);
                        break;

                    case VirtualDvdDrive virtualDvdDriveDefinition:
                        this.AddVirtualDvdDisk(systemSettings, scsiController, virtualDvdDriveDefinition, address);
                        break;
                }
            }

            scsiControllers.DisposeCollection();
        }
    }

    private void AddVirtualHardDisk(ManagementObject systemSettings, ManagementObject scsiController, VirtualHardDrive virtualHardDriveDefinition, int address)
    {
        using ManagementObject virtualHardDriveResource = ResourceQueries.CreateDefaultResource(ResourceSubtype.VirtualHardDrive, this.VirtualizationScope);
        virtualHardDriveResource["Parent"] = scsiController.Path.Path; // Scsi Controller
        virtualHardDriveResource["AddressOnParent"] = address; // Port
        this.Vmms.AddResourceSettings(this._logger, systemSettings, new[] { virtualHardDriveResource }, out ManagementObject[] virtualHardDrives);

        if (virtualHardDriveDefinition.VirtualHardDisk != null)
        {
            ManagementObject virtualHardDrive = virtualHardDrives[0];

            // ==================================================================================
            // Create Virtual Hard Disk
            // ==================================================================================
            using ManagementObject virtualHardDiskSettings = VMQueries.CreateVMSettingObject(VMSetting.VirtualHardDisk, this.VirtualizationScope);
            virtualHardDiskSettings["Type"] = virtualHardDriveDefinition.VirtualHardDisk.Type;
            virtualHardDiskSettings["Format"] = virtualHardDriveDefinition.VirtualHardDisk.Format;
            virtualHardDiskSettings["MaxInternalSize"] = virtualHardDriveDefinition.VirtualHardDisk.Size * 1_073_741_824; // Bytes
            virtualHardDiskSettings["Path"] = virtualHardDriveDefinition.VirtualHardDisk.Path;
            CreateVirtualHardDisk(virtualHardDiskSettings);

            // ==================================================================================
            // Attach Virtual Hard Disk
            // ==================================================================================
            using ManagementObject virtualHardDiskResource = ResourceQueries.CreateDefaultResource(ResourceSubtype.VirtualHardDisk, this.VirtualizationScope);
            virtualHardDiskResource["Parent"] = virtualHardDrive.Path.Path;
            virtualHardDiskResource["HostResource"] = new[] { virtualHardDriveDefinition.VirtualHardDisk.Path };
            if ((virtualHardDriveDefinition.MinimumIops > 0 || virtualHardDriveDefinition.MaximumIops > 0) &&
                virtualHardDriveDefinition.MaximumIops >= virtualHardDriveDefinition.MinimumIops)
            {
                // Minimum IOPS
                virtualHardDiskResource["IOPSReservation"] = virtualHardDriveDefinition.MinimumIops;

                // Maximum IOPS
                virtualHardDiskResource["IOPSLimit"] = virtualHardDriveDefinition.MaximumIops;
            }

            this.Vmms.AddResourceSettings(this._logger, systemSettings, new[] { virtualHardDiskResource }, out _);
        }

        virtualHardDrives.DisposeCollection();
    }

    private void CreateVirtualHardDisk(ManagementObject virtualHardDiskSettings)
    {
        using ManagementBaseObject inputParameters = Ims.GetMethodParameters("CreateVirtualHardDisk");
        inputParameters["VirtualDiskSettingData"] = virtualHardDiskSettings.GetText(TextFormat.WmiDtd20);
        using ManagementBaseObject outputParameters = Ims.InvokeMethod("CreateVirtualHardDisk", inputParameters, null);
        JobOutputHelper.ValidateOutput(outputParameters, this._logger);
    }

    private void AddVirtualDvdDisk(ManagementObject systemSettings, ManagementObject scsiController, VirtualDvdDrive virtualDvdDriveDefinition, int address)
    {
        // ==================================================================================
        // Create DVD Drive
        // ==================================================================================
        using ManagementObject virtualDvdDriveResource = ResourceQueries.CreateDefaultResource(ResourceSubtype.VirtualDvdDrive, this.VirtualizationScope);
        virtualDvdDriveResource["Parent"] = scsiController.Path.Path; // Scsi Controller
        virtualDvdDriveResource["AddressOnParent"] = address; // Port
        this.Vmms.AddResourceSettings(this._logger, systemSettings, new[] { virtualDvdDriveResource }, out ManagementObject[] virtualDvdDrives);
        if (virtualDvdDriveDefinition.VirtualDvdDisk != null)
        {
            //==================================================================================
            // Attach Virtual DVD Disk
            //==================================================================================
            ManagementObject virtualDvdDrive = virtualDvdDrives[0];
            using ManagementObject virtualDvdDiskResource = ResourceQueries.CreateDefaultResource(ResourceSubtype.VirtualDvdDisk, this.VirtualizationScope);
            virtualDvdDiskResource["Parent"] = virtualDvdDrive.Path.Path;
            virtualDvdDiskResource["HostResource"] = new[] { virtualDvdDriveDefinition.VirtualDvdDisk.Path };
            this.Vmms.AddResourceSettings(this._logger, systemSettings, new[] { virtualDvdDiskResource }, out _);
        }

        virtualDvdDrives.DisposeCollection();
    }
}