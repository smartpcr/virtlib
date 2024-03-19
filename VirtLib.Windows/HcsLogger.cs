// -----------------------------------------------------------------------
// <copyright file="HcsLogger.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows;

using System.Runtime.CompilerServices;
using Microsoft.Extensions.Logging;

internal static partial class HcsLogger
{
    [LoggerMessage(
        0,
        LogLevel.Information,
        "ManagementObject: {elementName} \n\tcalled from {memberName}, in file {callerFile}, at line {lineNumber}\n{managementObjectProperties}")]
    public static partial void LogManagementObject(
        this ILogger logger,
        string elementName,
        string managementObjectProperties,
        [CallerMemberName] string memberName = "",
        [CallerFilePath] string callerFile = "",
        [CallerLineNumber] int lineNumber = 0);

    [LoggerMessage(
        1,
        LogLevel.Debug,
        "VM: {vmName} \n\tcalled from {memberName}, in file {callerFile}, at line {lineNumber}\n\t{json}")]
    public static partial void LogVM(
        this ILogger logger,
        string vmName,
        string json,
        [CallerMemberName] string memberName = "",
        [CallerFilePath] string callerFile = "",
        [CallerLineNumber] int lineNumber = 0);

}