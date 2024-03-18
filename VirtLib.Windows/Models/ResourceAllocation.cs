// -----------------------------------------------------------------------
// <copyright file="ResourceAllocation.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

using System.Management;

public class ResourceAllocation
{
    public ResourceType ResourceType { get; set; }

    public ResourceAllocation(ManagementObject resourceAllocObj)
    {
        HcsLogger.LogManagementObject(resourceAllocObj);
        ResourceType = resourceAllocObj["ResourceType"].ReadResourceType();
    }
}