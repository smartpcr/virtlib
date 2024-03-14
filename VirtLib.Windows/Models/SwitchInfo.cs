// -----------------------------------------------------------------------
// <copyright file="VSwitch.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

using System;
using System.Collections.Generic;
using System.Linq;
using System.Management;
using Queries;

public class SwitchInfo : IDisposable
{
    public ManagementObject Inner { get; set; }
    public string? Name { get; set; }
    public SwitchConnectionType ConnectionType { get; set; } = SwitchConnectionType.Private;
    public List<PortInfo> Ports { get; set; } = new List<PortInfo>();
    public List<SwitchFeatureType> Features { get; set; } = new List<SwitchFeatureType>();
    public bool VLanEnabled { get; set; }
    public int? VLanId { get; set; }
    public EnabledDefault EnabledDefault { get; set; }
    public EnabledState EnabledState { get; set; }
    public HealthState HealthState { get; set; }
    public RequestedState RequestedState { get; set; }
    public string Description { get; set; }

    public SwitchInfo(ManagementObject managementObject)
    {
        Inner = managementObject;
        HcsLogger.LogManagementObject(managementObject);
        Name = managementObject["Name"]?.ToString();
        EnabledDefault = managementObject["EnabledDefault"].ReadEnabledDefault();
        EnabledState = managementObject["EnabledState"].ReadEnabledState();
        HealthState = managementObject["HealthState"].ReadHealthState();
        RequestedState = managementObject["RequestedState"].ReadRequestedState();
        Description = managementObject["Description"]?.ToString() ?? string.Empty;

        using var portList = this.GetPorts();
        foreach (var portObj in portList.OfType<ManagementObject>())
        {
            using (portObj)
            {
#pragma warning disable CA2000
                var portInfo = new PortInfo(portObj);
#pragma warning restore CA2000
                using var portSettings = portInfo.GetPortSettings();
                var portSetting = portSettings.OfType<ManagementObject>().FirstOrDefault();
                if (portSetting != null)
                {
                    portInfo.ConnectionType = portSetting.ReadPortConnectionType();
                    if (portInfo.ConnectionType == PortConnectionType.VirtualMachine)
                    {
                        using var vmSettings = portSetting.GetRelated(
                            VmWmiClasses.VirtualSystemSettingData,
                            VmWmiClasses.VirtualSystemSettingDataComponent);
                        var vmSetting = vmSettings.OfType<ManagementObject>().FirstOrDefault();
                        if (vmSetting != null)
                        {
                            portInfo.ConnectedName = vmSetting["ElementName"]?.ToString() ?? string.Empty;
                        }
                    }
                    else if (portInfo.ConnectionType == PortConnectionType.External)
                    {
                        if (portSetting["HostResource"] is string[] { Length: > 0 } hostResources)
                        {
                            using var externalAdapter = new ManagementObject(hostResources[0]);
                            portInfo.ConnectedName = externalAdapter["ElementName"]?.ToString() ?? string.Empty;
                        }
                    }

                    using var portFeatures = portSetting.GetRelated(
                        VmWmiClasses.EthernetSwitchPortFeatureSettingData,
                        VmWmiClasses.EthernetPortSettingDataComponent);
                    foreach (ManagementObject portFeature in portFeatures.OfType<ManagementObject>())
                    {
                        using (portFeature)
                        {
                            portInfo.FeatureList.Add(portFeature.ReadPortFeatureType());
                        }
                    }
                }

                Ports.Add(portInfo);
            }
        }

        using var switchFeatures = managementObject.GetRelated(
            VmWmiClasses.EthernetSwitchFeatureSettingData,
            VmWmiClasses.VirtualEthernetSwitchSettingDataComponent);
        foreach (ManagementObject switchFeature in switchFeatures.OfType<ManagementObject>())
        {
            using (switchFeature)
            {
                Features.Add(switchFeature.ReadSwitchFeatureType());
            }
        }

        this.ConnectionType = this.ReadSwitchConnejctionType();
    }

    public void Dispose()
    {
        Inner.Dispose();
        foreach (var port in Ports)
        {
            port.Dispose();
        }
    }
}
