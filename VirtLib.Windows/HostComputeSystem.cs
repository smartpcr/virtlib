// -----------------------------------------------------------------------
// <copyright file="HostComputeSystem.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows
{
    using System;
    using System.Management;
    using System.Runtime.Versioning;

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
            var virtualizationNamespace = @$"\\{this.host}\root\virtualization\v2";
            virtualizationScope = new ManagementScope(virtualizationNamespace);
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
    }
}