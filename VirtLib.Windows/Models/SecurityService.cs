// -----------------------------------------------------------------------
// <copyright file="SecurityService.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

using System;
using System.Management;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;

public class SecurityService
{
    private readonly ILogger<SecurityService> _logger;

    public string Caption { get; set; }
    public string CreationClassName { get; set; }
    public string Description { get; set; }
    public string ElementName { get; set; }
    public string Name { get; set; }
    public string SystemCreationClassName { get; set; }
    public string SystemName { get; set; }

    public SecurityService(IServiceProvider serviceProvider, ManagementObject securityServiceObj)
    {
        var loggerFactory = serviceProvider.GetRequiredService<ILoggerFactory>();
        this._logger = loggerFactory.CreateLogger<SecurityService>();
        this._logger.LogManagementObject(securityServiceObj);

        Caption = (string)securityServiceObj["Caption"];
        CreationClassName = (string)securityServiceObj["CreationClassName"];
        Description = (string)securityServiceObj["Description"];
        ElementName = (string)securityServiceObj["ElementName"];
        Name = (string)securityServiceObj["Name"];
        SystemCreationClassName = (string)securityServiceObj["SystemCreationClassName"];
        SystemName = (string)securityServiceObj["SystemName"];
    }
}