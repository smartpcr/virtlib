// -----------------------------------------------------------------------
// <copyright file="HostComputeSystem.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace virt.net
{
    using System;
    using System.Linq;
    using System.Management;
    using System.Runtime.Versioning;

    [SupportedOSPlatform("windows")]
    public class HostComputeSystem
    {
        private const string MsvmVirtualSystemManagementService = "Msvm_VirtualSystemManagementService";
        private const string Win32OperatingSystem = "Win32_OperatingSystem";
        private readonly string host;
        private readonly ManagementObject vmms; // virtual machine management service

        public HostComputeSystem(string? hyperVHost)
        {
            this.host = string.IsNullOrEmpty(hyperVHost) ? Environment.MachineName : hyperVHost;
            this.vmms = new ManagementObject(
                new ManagementPath(
                    $@"\\{this.host}\root\virtualization\v2:{MsvmVirtualSystemManagementService}"));
        }

        public (string? name, string? version) GetOsInfo()
        {
            using var mc = new ManagementClass(Win32OperatingSystem);
            var moc = mc.GetInstances();
            var os = moc.OfType<ManagementObject>().First();
            return (os["Name"]?.ToString(), os["Version"]?.ToString());
        }

        // public void DefineSystem(
        //     ManagementObject systemSettings,
        //     ManagementObject[] resourceSettings,
        //     out ManagementObject resultSystem)
        // {
        //     using ManagementBaseObject inputParameters = GetVirtualMachineManagementService();
        //
        // }

        public HyperVHost? GetHyperVHost()
        {
            ManagementScope scope = new ManagementScope(@$"\\{this.host}\root\virtualization\v2");
            ObjectQuery query = new ObjectQuery($"SELECT * FROM {MsvmVirtualSystemManagementService}");
            using var searcher = new ManagementObjectSearcher(scope, query);
            var managementObject = searcher.Get().OfType<ManagementObject>().FirstOrDefault();
            return HyperVHost.FromManagementObject(managementObject);
        }

        private ManagementObject GetVirtualMachineManagementService()
        {
            using var managementClass = new ManagementClass(MsvmVirtualSystemManagementService);
            using var managementObjects = managementClass.GetInstances();
            return managementObjects.OfType<ManagementObject>().First();
        }
    }
}