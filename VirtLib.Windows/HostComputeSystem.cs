// -----------------------------------------------------------------------
// <copyright file="HostComputeSystem.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows
{
    using System;
    using System.Linq;
    using System.Management;
    using System.Runtime.Versioning;
    using Definitions;
    using Models;
    using Queries;

    [SupportedOSPlatform("windows")]
    public partial class HostComputeSystem
    {
        private const string MsvmVirtualSystemManagementService = "Msvm_VirtualSystemManagementService";
        private const string MsvmSecurityService = "Msvm_SecurityService";
        private const string MsvmImageManagementService = "Msvm_ImageManagementService";
        private const string Win32OperatingSystem = "Win32_OperatingSystem";

        private readonly string host;
        private readonly ManagementScope virtualizationScope;
        private readonly ManagementScope hgsScope;
        private readonly ManagementObject ss;       // Security Service
        private readonly ManagementObject ims;      // Image Management Service
        private readonly ManagementObject vmms;     // Virtual Machine Management Service

        public HostComputeSystem() : this(Environment.MachineName)
        {
        }

        public HostComputeSystem(string? hyperVHost)
        {
            this.host = hyperVHost ?? Environment.MachineName;
            virtualizationScope = new ManagementScope(@$"\\{this.host}\root\virtualization\v2");
            hgsScope = new ManagementScope(@"\ROOT\Microsoft\Windows\Hgs");
            var isRunningAsAdmin = IsElevated();
            if (!isRunningAsAdmin)
            {
                throw new UnauthorizedAccessException("You must run as an administrator to access Hyper-V");
            }

            this.vmms = GetVirtualMachineManagementService();
            HcsLogger.LogManagementObject(this.vmms);
            this.ss = GetSecurityService();
            HcsLogger.LogManagementObject(this.ss);
            this.ims = GetImageManagementService();
            HcsLogger.LogManagementObject(this.ims);
        }

        #region get
        public OSInfo? GetOsInfo()
        {
            using var mc = new ManagementClass(Win32OperatingSystem);
            var moc = mc.GetInstances();
            var os = moc.OfType<ManagementObject>().FirstOrDefault();
            return os == null ? null : new OSInfo(os);
        }

        public HyperVHostInfo? GetHyperVHost()
        {
            ObjectQuery query = new ObjectQuery($"SELECT * FROM {MsvmVirtualSystemManagementService}");
            using var searcher = new ManagementObjectSearcher(this.virtualizationScope, query);
            var managementObject = searcher.Get().OfType<ManagementObject>().FirstOrDefault();
            return managementObject == null ? null : new HyperVHostInfo(managementObject);
        }

        public SwitchInfo? GetVSwitch(string virtualSwitchName)
        {
            return WmiUtilities.FindVSwitch(virtualSwitchName, this.virtualizationScope);
        }
        #endregion

        public bool IsVirtualMachineExist(string vmName)
        {
            var query = new ObjectQuery(string.Format(VMQueries.GetVMByName, vmName));
            using var searcher = new ManagementObjectSearcher(this.virtualizationScope, query);
            using var collection = searcher.Get();
            return collection.Count > 0;
        }

        public bool IsVirtualSwitchExist(string virtualSwitchName)
        {
            var vswitch = GetVSwitch(virtualSwitchName);
            return vswitch != null;
        }

        public void CreateVirtualMachine(VirtualMachineDefinition vm)
        {
            if (IsVirtualMachineExist(vm.Name))
            {
                throw new InvalidOperationException($"Virtual machine {vm.Name} already exists.");
            }

            ValidateHardDisksNotExist(vm);
            ValidateVirtualSwitchExists(vm);
        }

        public void DefineSystem(
            ManagementObject systemSettings,
            ManagementObject[] resourceSettings,
            out ManagementObject resultSystem)
        {
            using var inputParameters = this.vmms.GetMethodParameters("DefineSystem");
            inputParameters["SystemSettings"] = systemSettings;
            inputParameters["ResourceSettings"] = resourceSettings;
            using var output = this.vmms.InvokeMethod("DefineSystem", inputParameters, null);
            resultSystem = new ManagementClass(output["ResultingSystem"].ToString());
        }

    }
}