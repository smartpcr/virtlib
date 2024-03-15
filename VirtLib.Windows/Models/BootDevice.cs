// -----------------------------------------------------------------------
// <copyright file="BootDevice.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

using System.IO;
using System.Linq;
using System.Management;
using Queries;

/// <summary>Defines a boot device.</summary>
public class BootDevice
{
    /// <summary>Gets the firmware description of the boot device.</summary>
    public string Description { get; }

    /// <summary>Gets the type of boot device.</summary>
    public BootDeviceType DeviceType { get; }

    /// <summary>Gets a device specific value describing the boot media.</summary>
    public string Value { get; }

    private readonly string bootEntry;

    /// <summary>Initializes a new instance of the <see cref="BootDevice"/> class from the specified WMI string.</summary>
    /// <param name="bootEntry">The WMI string for this boot device.</param>
    public BootDevice(string bootEntry)
    {
        this.bootEntry = bootEntry;
        DeviceType = BootDeviceType.Unknown;
        Value = "";

        using ManagementObject bootSource = new ManagementObject(bootEntry);
        Description = (string)bootSource["BootSourceDescription"];
        switch ((uint)bootSource["BootSourceType"])
        {
            case 1:
                ManagementObject? resource = bootSource.GetRelated(VMQueries.RelatedSettings.Resource).OfType<ManagementObject>().FirstOrDefault();
                if (resource != null)
                {
                    using (resource)
                    {
                        switch ((ushort)resource["ResourceType"])
                        {
                            case 16: DeviceType = BootDeviceType.DvdDrive; break;
                            case 17: DeviceType = BootDeviceType.HardDrive; break;
                        }

                        ManagementObject? storage = resource.GetRelated(VMQueries.RelatedSettings.Storage).OfType<ManagementObject>().FirstOrDefault();
                        if (storage != null)
                        {
                            using (storage)
                            {
                                Value = Path.GetFileName(((string[])storage["HostResource"])[0]);
                            }
                        }
                    }
                }

                break;
            case 2:
                ManagementObject? networkAdapter = bootSource.GetRelated(VMQueries.RelatedSettings.NetworkAdapter).OfType<ManagementObject>().FirstOrDefault();
                if (networkAdapter != null)
                {
                    using (networkAdapter)
                    {
                        DeviceType = BootDeviceType.NetworkAdapter;
                        ManagementObject? switchPort = networkAdapter.GetRelated(VMQueries.RelatedSettings.SwitchPort).OfType<ManagementObject>().FirstOrDefault();
                        if (switchPort != null)
                        {
                            using (switchPort)
                            {
                                if ((ushort)switchPort["EnabledState"] == 2)
                                {
                                    Value = (string)switchPort["LastKnownSwitchName"];
                                }
                                else
                                {
                                    Value = "None";
                                }
                            }
                        }
                    }
                }

                break;

            case 3:
                DeviceType = BootDeviceType.File;
                Value = Path.GetFileName(((string)bootSource["FirmwareDevicePath"]).Split('/')[1]);
                break;
        }

    }

    /// <summary>Returns the WMI string for this boot device.</summary>
    public override string ToString()
    {
        return bootEntry;
    }
}