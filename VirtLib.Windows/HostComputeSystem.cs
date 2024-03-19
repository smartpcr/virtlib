// -----------------------------------------------------------------------
// <copyright file="HostComputeSystem.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows
{
    using System;
    using System.Collections.Generic;
    using System.Management;
    using System.Runtime.Versioning;
    using Microsoft.Extensions.DependencyInjection;
    using Microsoft.Extensions.Logging;
    using Newtonsoft.Json;
    using Newtonsoft.Json.Converters;
    using Newtonsoft.Json.Serialization;

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
        private readonly IServiceProvider _serviceProvider;
        private readonly ILogger<HostComputeSystem> _logger;

        private readonly JsonSerializerSettings _serializerSetting = new JsonSerializerSettings
        {
            Formatting = Formatting.Indented,
            ContractResolver = new CamelCasePropertyNamesContractResolver(),
            Converters = new List<JsonConverter> { new StringEnumConverter() }
        };

        public HostComputeSystem(IServiceProvider serviceProvider) : this(serviceProvider, Environment.MachineName)
        {
        }

        public HostComputeSystem(IServiceProvider serviceProvider, string? hyperVHost)
        {
            this._serviceProvider = serviceProvider;
            this.host = hyperVHost ?? Environment.MachineName;
            var loggerFactory = serviceProvider.GetRequiredService<ILoggerFactory>();
            this._logger = loggerFactory.CreateLogger<HostComputeSystem>();
            var virtualizationNamespace = @$"\\{this.host}\root\virtualization\v2";
            virtualizationScope = new ManagementScope(virtualizationNamespace);
            hgsScope = new ManagementScope(@"\ROOT\Microsoft\Windows\Hgs");
            var isRunningAsAdmin = IsElevated();
            if (!isRunningAsAdmin)
            {
                throw new UnauthorizedAccessException("You must run as an administrator to access Hyper-V");
            }

            this.vmms = GetVirtualMachineManagementService();
            this._logger.LogManagementObject(this.vmms);
            this.ss = GetSecurityService();
            this._logger.LogManagementObject(this.ss);
            this.ims = GetImageManagementService();
            this._logger.LogManagementObject(this.ims);
        }
    }
}