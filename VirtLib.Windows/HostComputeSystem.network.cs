// -----------------------------------------------------------------------
// <copyright file="HostComputeSystem.network.cs" company="Microsoft Corp.">
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
    private void UpdateNetworkAdapters(VirtualMachineDefinition vm, ManagementObject systemSettings)
    {
        foreach (NetworkAdapter networkAdapterDefinition in vm.NetworkAdapters)
        {
            // ==================================================================================
            // Create Network Adapter
            // ==================================================================================
            using var networkAdapterResource = ResourceQueries.CreateDefaultResource(ResourceSubtype.NetworkAdapter, this.virtualizationScope);
            networkAdapterResource["ElementName"] = "Network Adapter";

            //----------------------------------------------------------------------------------
            // Configure MAC Address
            //----------------------------------------------------------------------------------
            if (networkAdapterDefinition.MacAddress != null)
            {
                networkAdapterResource["StaticMacAddress"] = true;
                networkAdapterResource["Address"] = networkAdapterDefinition.MacAddress.ToString();
            }

            //----------------------------------------------------------------------------------
            // Configure Protected Network
            //----------------------------------------------------------------------------------
            networkAdapterResource["ClusterMonitored"] = networkAdapterDefinition.ProtectedNetwork;

            //----------------------------------------------------------------------------------
            // Configure Device Naming
            //----------------------------------------------------------------------------------
            networkAdapterResource["DeviceNamingEnabled"] = networkAdapterDefinition.DeviceNaming;

            //----------------------------------------------------------------------------------
            this.vmms.AddResourceSettings(systemSettings, new[] { networkAdapterResource }, out ManagementObject[] networkAdapters);
            ManagementObject networkAdapter = networkAdapters[0];

            //==================================================================================
            // Configure Ethernet Port
            //==================================================================================
            using var switchPortResource = ResourceQueries.CreateDefaultResource(ResourceSubtype.SwitchPort, this.virtualizationScope);
            switchPortResource["ElementName"] = "Dynamic Ethernet Switch Port";
            switchPortResource["Parent"] = networkAdapter.Path.Path; // Network Adapter

            //----------------------------------------------------------------------------------
            // Connect Virtual Switch
            //----------------------------------------------------------------------------------
            if (!string.IsNullOrWhiteSpace(networkAdapterDefinition.VirtualSwitch))
            {
                using ManagementObject virtualSwitch = GetVirtualSwitch(networkAdapterDefinition.VirtualSwitch);
                switchPortResource["HostResource"] = new[] { virtualSwitch.Path.Path }; // Virtual Switch
            }
            else
            {
                switchPortResource["EnabledState"] = 3; // Disabled
            }

            //----------------------------------------------------------------------------------
            this.vmms.AddResourceSettings(systemSettings, new[] { switchPortResource }, out ManagementObject[] switchPorts);
            ManagementObject switchPort = switchPorts[0];

            // ==================================================================================
            // Configure VLAN
            // ==================================================================================
            if (networkAdapterDefinition.Vlan)
            {
                using ManagementObject portVlanSettings = NetworkFeatures.Vlan.CreateFeatureSettings(this.virtualizationScope);
                portVlanSettings["OperationMode"] = 1; // Access
                portVlanSettings["AccessVlanId"] = networkAdapterDefinition.VlanId;
                this.vmms.AddFeatureSettings(switchPort, new[] { portVlanSettings }, out _);
            }

            //==================================================================================
            // Configure Quality of Service
            //==================================================================================
            if ((networkAdapterDefinition.MinimumBandwidth > 0 || networkAdapterDefinition.MaximumBandwidth > 0) &&
                networkAdapterDefinition.MaximumBandwidth >= networkAdapterDefinition.MinimumBandwidth)
            {
                using ManagementObject portBandwidthSettings = NetworkFeatures.Bandwidth.CreateFeatureSettings(this.virtualizationScope);
                portBandwidthSettings["Reservation"] = networkAdapterDefinition.MinimumBandwidth * 1_000_000;
                portBandwidthSettings["Limit"] = networkAdapterDefinition.MaximumBandwidth * 1_000_000;
                this.vmms.AddFeatureSettings(switchPort, new[] { portBandwidthSettings }, out _);
            }

            // ==================================================================================
            // Configure Hardware Acceleration
            // ==================================================================================
            if (networkAdapterDefinition.Vmq ||
                networkAdapterDefinition.IpsecOffloading ||
                networkAdapterDefinition.SrIov)
            {
                using var ethernetOffloads = switchPort.GetRelated(VMQueries.GetVMSettingWmiClass(VMSetting.EthernetPortOffload));
                using ManagementObject portOffloadSettings = ethernetOffloads.OfType<ManagementObject>().First();

                // ----------------------------------------------------------------------------------
                // Configure Virtual Machine Queue
                // ----------------------------------------------------------------------------------
                if (networkAdapterDefinition.Vmq)
                {
                    portOffloadSettings["VMQOffloadWeight"] = 100;
                }
                else
                {
                    portOffloadSettings["VMQOffloadWeight"] = 0;
                }

                //----------------------------------------------------------------------------------
                // Configure IPsec Task Offloading
                //----------------------------------------------------------------------------------
                if (networkAdapterDefinition.IpsecOffloading)
                {
                    portOffloadSettings["IPSecOffloadLimit"] = networkAdapterDefinition.IpsecSecurityAssociations;
                }
                else
                {
                    portOffloadSettings["IPSecOffloadLimit"] = 0;
                }

                //----------------------------------------------------------------------------------
                // Configure SR-IOV
                //----------------------------------------------------------------------------------
                if (networkAdapterDefinition.SrIov)
                {
                    portOffloadSettings["IOVOffloadWeight"] = 100;
                }
                else
                {
                    portOffloadSettings["IOVOffloadWeight"] = 0;
                }

                //----------------------------------------------------------------------------------
                ModifyFeatureSettings(new[] { portOffloadSettings }, out _);
            }

            //==================================================================================
            // Configure Advanced Features
            //==================================================================================
            if (networkAdapterDefinition.MacAddressSpoofing ||
                networkAdapterDefinition.DhcpGuard ||
                networkAdapterDefinition.RouterGuard ||
                networkAdapterDefinition.PortMirroringMode != PortMirroringMode.None ||
                networkAdapterDefinition.NicTeaming)
            {
                using ManagementObject portSecuritySettings = NetworkFeatures.Security.CreateFeatureSettings(this.virtualizationScope);

                // ----------------------------------------------------------------------------------
                // Configure MAC Address Spoofing
                // ----------------------------------------------------------------------------------
                portSecuritySettings["AllowMacSpoofing"] = networkAdapterDefinition.MacAddressSpoofing;

                //----------------------------------------------------------------------------------
                // Configure DHCP Guard
                //----------------------------------------------------------------------------------
                portSecuritySettings["EnableDhcpGuard"] = networkAdapterDefinition.DhcpGuard;

                //----------------------------------------------------------------------------------
                // Configure Router Advertisement Guard
                //----------------------------------------------------------------------------------
                portSecuritySettings["EnableRouterGuard"] = networkAdapterDefinition.RouterGuard;

                //----------------------------------------------------------------------------------
                // Configure Port Mirroring
                //----------------------------------------------------------------------------------
                switch (networkAdapterDefinition.PortMirroringMode)
                {
                    case PortMirroringMode.None:
                        portSecuritySettings["MonitorMode"] = 0; // None
                        break;

                    case PortMirroringMode.Destination:
                        portSecuritySettings["MonitorMode"] = 1; // Destination
                        break;

                    case PortMirroringMode.Source:
                        portSecuritySettings["MonitorMode"] = 2; // Source
                        break;
                }

                //----------------------------------------------------------------------------------
                // Configure NIC Teaming
                //----------------------------------------------------------------------------------
                portSecuritySettings["AllowTeaming"] = networkAdapterDefinition.NicTeaming;

                //----------------------------------------------------------------------------------
                AddFeatureSettings(switchPort, new[] { portSecuritySettings }, out _);
            }

            networkAdapters.Dispose();
            switchPorts.Dispose();
        }
    }

    private ManagementObject GetVirtualSwitch(string name)
    {
        ObjectQuery query = new ObjectQuery($"SELECT * FROM {VmWmiClasses.VirtualEthernetSwitch} WHERE Caption = \"Virtual Switch\" AND ElementName = \"{name}\"");
        using ManagementObjectSearcher searcher = new ManagementObjectSearcher(virtualizationScope, query);
        using ManagementObjectCollection collection = searcher.Get();
        if (collection.Count == 0)
        {
            throw new ManagementException("Unable to find the Virtual Switch.");
        }

        return collection.OfType<ManagementObject>().First();
    }

    private void ModifyFeatureSettings(
        ManagementObject[] featureSettings,
        out ManagementObject[] resultingFeatureSettings)
    {
        using ManagementBaseObject inputParameters = vmms.GetMethodParameters("ModifyFeatureSettings");
        inputParameters["FeatureSettings"] = featureSettings.ToStringArray();
        using ManagementBaseObject outputParameters = vmms.InvokeMethod("ModifyFeatureSettings", inputParameters, null);
        JobOutputHelper.ValidateOutput(outputParameters);
        resultingFeatureSettings = ((string[])outputParameters["ResultingFeatureSettings"]).ToObjectArray();
    }

    private void AddFeatureSettings(
        ManagementObject ethernetPortAllocationSettings,
        ManagementObject[] featureSettings,
        out ManagementObject[] resultingFeatureSettings)
    {
        using ManagementBaseObject inputParameters = vmms.GetMethodParameters("AddFeatureSettings");
        inputParameters["AffectedConfiguration"] = ethernetPortAllocationSettings.Path.Path;
        inputParameters["FeatureSettings"] = featureSettings.ToStringArray();
        using ManagementBaseObject outputParameters = vmms.InvokeMethod("AddFeatureSettings", inputParameters, null);
        JobOutputHelper.ValidateOutput(outputParameters);
        resultingFeatureSettings = ((string[])outputParameters["ResultingFeatureSettings"]).ToObjectArray();
    }
}