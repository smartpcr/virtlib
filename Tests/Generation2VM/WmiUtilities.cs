// -----------------------------------------------------------------------
// <copyright file="WmiUtilities.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace Generation2VM;

using System;
using System.Linq;
using System.Management;

public class WmiUtilities
{
    public static ManagementObject GetVirtualMachineManagementService(ManagementScope scope)
    {
        using var managementServiceClass = new ManagementClass("Msvm_VirtualSystemManagementService");
        managementServiceClass.Scope = scope;
        var managementService = GetFirstObjectFromCollection(managementServiceClass.GetInstances());
        return managementService;
    }

    public static ManagementObject GetFirstObjectFromCollection(ManagementObjectCollection collection)
    {
        var firstInstance = collection.OfType<ManagementObject>().FirstOrDefault();
        if (firstInstance == null)
        {
            throw new ArgumentException("There are no instances in the collection.", nameof(collection));
        }

        return firstInstance;
    }
}