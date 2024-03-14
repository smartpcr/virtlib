// -----------------------------------------------------------------------
// <copyright file="HcsLogger.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows;

using System;
using System.Management;
using System.Runtime.CompilerServices;
using Microsoft.Extensions.Logging;

internal static partial class HcsLogger
{
    [LoggerMessage(
        0,
        LogLevel.Debug,
        "ManagementObject: {managementObject} \n\tcalled from {memberName}, in file {callerFile}, at line {lineNumber}")]
    public static partial void LogManagementObject(
        this ILogger logger,
        ManagementObject managementObject,
        [CallerMemberName] string memberName = "",
        [CallerFilePath] string callerFile = "",
        [CallerLineNumber] int lineNumber = 0);

    internal static void LogManagementObject(
        ManagementObject managementObject,
        [CallerMemberName] string memberName = "",
        [CallerFilePath] string callerFile = "",
        [CallerLineNumber] int lineNumber = 0)
    {
        Console.WriteLine($"{managementObject["ElementName"]}: called from {memberName}, in file {callerFile}, at line {lineNumber}");
        foreach (var prop in managementObject.Properties)
        {
            if (prop.Value == null)
            {
                continue;
            }

            Console.WriteLine(prop.IsArray
                ? $"\t{prop.Name} ({prop.Type}): {string.Join(", ", prop.Value)}"
                : $"\t{prop.Name} ({prop.Type}): {prop.Value}");
        }
    }
}