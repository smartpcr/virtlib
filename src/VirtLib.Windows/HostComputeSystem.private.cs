// -----------------------------------------------------------------------
// <copyright file="HostComputeSystem.private.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows;

using System.Linq;
using System.Management;
using System.Security.Principal;
using Models;
using Newtonsoft.Json;

public partial class HostComputeSystem
{
    private (ManagementObject vmms, VirtualMachineManagementService vsms) GetVirtualMachineManagementService()
    {
        using var managementClass = new ManagementClass(MsvmVirtualSystemManagementService);
        managementClass.Scope = this.VirtualizationScope;
        using var managementObjects = managementClass.GetInstances();
        var vmms = managementObjects.OfType<ManagementObject>().First();
        var vsms = new VirtualMachineManagementService(this._serviceProvider, vmms);
        this._logger.LogVmss(vsms.Name, JsonConvert.SerializeObject(vsms, this._serializerSetting));
        return (vmms, vsms);
    }

    private (ManagementObject securityServiceObj, SecurityService securityService) GetSecurityService()
    {
        using var managementClass = new ManagementClass(MsvmSecurityService);
        managementClass.Scope = this.VirtualizationScope;
        using var managementObjects = managementClass.GetInstances();
        var securityServiceObj = managementObjects.OfType<ManagementObject>().First();
        var securityService = new SecurityService(this._serviceProvider, securityServiceObj);
        this._logger.LogSecurityService(securityService.Name, JsonConvert.SerializeObject(securityService, this._serializerSetting));
        return (securityServiceObj, securityService);
    }

    private (ManagementObject ims, ImageManagementService imageManagementService) GetImageManagementService()
    {
        using var managementClass = new ManagementClass(MsvmImageManagementService);
        managementClass.Scope = VirtualizationScope;
        using var managementObjects = managementClass.GetInstances();
        var ims = managementObjects.OfType<ManagementObject>().First();
        var imageManagementService = new ImageManagementService(this._serviceProvider, ims);
        this._logger.LogImageManagementService(imageManagementService.Name, JsonConvert.SerializeObject(imageManagementService, this._serializerSetting));
        return (ims, imageManagementService);
    }

    private bool IsElevated()
    {
        var wi = WindowsIdentity.GetCurrent();
        var wp = new WindowsPrincipal(wi);
        return wp.IsInRole(WindowsBuiltInRole.Administrator);
    }
}