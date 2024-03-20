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
    using Models;
    using Newtonsoft.Json;
    using Newtonsoft.Json.Converters;
    using Newtonsoft.Json.Serialization;

    [SupportedOSPlatform("windows")]
    public partial class HostComputeSystem : IDisposable
    {
        private const string MsvmVirtualSystemManagementService = "Msvm_VirtualSystemManagementService";
        private const string MsvmSecurityService = "Msvm_SecurityService";
        private const string MsvmImageManagementService = "Msvm_ImageManagementService";
        private const string Win32OperatingSystem = "Win32_OperatingSystem";

        private readonly string host;
        private readonly ManagementScope virtualizationScope;
        private readonly ManagementScope hgsScope;
        private readonly IServiceProvider _serviceProvider;
        private readonly ILogger<HostComputeSystem> _logger;

        public ManagementObject Vmms { get; }
        public VirtualMachineManagementService VMManagementService { get; }
        public ManagementObject Ss { get; }
        public SecurityService SecurityService { get; }
        public ManagementObject Ims { get; }
        public ImageManagementService ImageManagementService { get; }

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

            (this.Vmms, this.VMManagementService) = GetVirtualMachineManagementService();
            (this.Ss, this.SecurityService) = GetSecurityService();
            (this.Ims, this.ImageManagementService) = GetImageManagementService();
        }

        public void Dispose()
        {
            this.Vmms.Dispose();
            this.Ss.Dispose();
            this.Ims.Dispose();
        }
    }
}