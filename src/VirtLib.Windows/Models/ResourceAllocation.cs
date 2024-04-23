// -----------------------------------------------------------------------
// <copyright file="ResourceAllocation.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

using System;
using System.Collections.Generic;
using System.Management;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;

public class ResourceAllocation
{
    private readonly ILogger<ResourceAllocation> _logger;

    public ResourceType ResourceType { get; set; }
    public string ResourceTypeName { get; set; }
    public List<string> Properties { get; set; }

    public ResourceAllocation(IServiceProvider serviceProvider, ManagementObject resourceAllocObj)
    {
        var loggerFactory = serviceProvider.GetRequiredService<ILoggerFactory>();
        this._logger = loggerFactory.CreateLogger<ResourceAllocation>();
        this._logger.LogManagementObject(resourceAllocObj);

        this.ResourceType = resourceAllocObj["ResourceType"].ReadResourceType();
        ResourceTypeName = this.ResourceType.ToString();
        this.Properties = new List<string>();

        foreach (var prop in resourceAllocObj.Properties)
        {
            this.Properties.Add(prop.IsArray
                ? $"{prop.Name} ({prop.Type}[]): {prop.Value.ReadValueArray()}"
                : $"{prop.Name} ({prop.Type}): {prop.Value}");
        }
    }
}