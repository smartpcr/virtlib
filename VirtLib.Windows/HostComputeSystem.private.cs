// -----------------------------------------------------------------------
// <copyright file="HostComputeSystem.private.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows;

using System;
using System.Linq;
using System.Management;
using System.Security.Principal;
using System.Threading;
using Definitions;

public partial class HostComputeSystem
{
    private ManagementObject GetVirtualMachineManagementService()
    {
        using var managementClass = new ManagementClass(MsvmVirtualSystemManagementService);
        managementClass.Scope = this.virtualizationScope;
        using var managementObjects = managementClass.GetInstances();
        return managementObjects.OfType<ManagementObject>().First();
    }

    private ManagementObject GetSecurityService()
    {
        using var managementClass = new ManagementClass(MsvmSecurityService);
        managementClass.Scope = this.virtualizationScope;
        using var managementObjects = managementClass.GetInstances();
        return managementObjects.OfType<ManagementObject>().First();
    }

    private ManagementObject GetImageManagementService()
    {
        using var managementClass = new ManagementClass(MsvmImageManagementService);
        managementClass.Scope = virtualizationScope;
        using var managementObjects = managementClass.GetInstances();
        return managementObjects.OfType<ManagementObject>().First();
    }

    private bool IsElevated()
    {
        var wi = WindowsIdentity.GetCurrent();
        var wp = new WindowsPrincipal(wi);
        return wp.IsInRole(WindowsBuiltInRole.Administrator);
    }
}