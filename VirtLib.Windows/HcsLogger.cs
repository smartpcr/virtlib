// -----------------------------------------------------------------------
// <copyright file="HcsLogger.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows;

using System.Runtime.CompilerServices;
using Definitions;
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
        LogLevel.Information,
        "VMSS: {vmssName} \n\tcalled from {memberName}, in file {callerFile}, at line {lineNumber}\n\t{json}")]
    public static partial void LogVmss(
        this ILogger logger,
        string vmssName,
        string json,
        [CallerMemberName] string memberName = "",
        [CallerFilePath] string callerFile = "",
        [CallerLineNumber] int lineNumber = 0);

    [LoggerMessage(
        2,
        LogLevel.Information,
        "SS: {securityServiceName} \n\tcalled from {memberName}, in file {callerFile}, at line {lineNumber}\n\t{json}")]
    public static partial void LogSecurityService(
        this ILogger logger,
        string securityServiceName,
        string json,
        [CallerMemberName] string memberName = "",
        [CallerFilePath] string callerFile = "",
        [CallerLineNumber] int lineNumber = 0);

    [LoggerMessage(
        3,
        LogLevel.Information,
        "IMS: {imageManagementServiceName} \n\tcalled from {memberName}, in file {callerFile}, at line {lineNumber}\n\t{json}")]
    public static partial void LogImageManagementService(
        this ILogger logger,
        string imageManagementServiceName,
        string json,
        [CallerMemberName] string memberName = "",
        [CallerFilePath] string callerFile = "",
        [CallerLineNumber] int lineNumber = 0);

    [LoggerMessage(
        4,
        LogLevel.Information,
        "VM: {vmName} \n\tcalled from {memberName}, in file {callerFile}, at line {lineNumber}\n\t{json}")]
    public static partial void LogVM(
        this ILogger logger,
        string vmName,
        string json,
        [CallerMemberName] string memberName = "",
        [CallerFilePath] string callerFile = "",
        [CallerLineNumber] int lineNumber = 0);

    [LoggerMessage(
        5,
        LogLevel.Information,
        "VM System Setting Definition: \n\tcalled from {memberName}, in file {callerFile}, at line {lineNumber}\n\t{json}")]
    public static partial void LogSystemSettingDefinition(
        this ILogger logger,
        string json,
        [CallerMemberName] string memberName = "",
        [CallerFilePath] string callerFile = "",
        [CallerLineNumber] int lineNumber = 0);

    [LoggerMessage(
        6,
        LogLevel.Information,
        "VM Processor Definition: \n\tcalled from {memberName}, in file {callerFile}, at line {lineNumber}\n\t{json}")]
    public static partial void LogProcessorDefinition(
        this ILogger logger,
        string json,
        [CallerMemberName] string memberName = "",
        [CallerFilePath] string callerFile = "",
        [CallerLineNumber] int lineNumber = 0);

    [LoggerMessage(
        7,
        LogLevel.Information,
        "VM Memory Definition: \n\tcalled from {memberName}, in file {callerFile}, at line {lineNumber}\n\t{json}")]
    public static partial void LogMemoryDefinition(
        this ILogger logger,
        string json,
        [CallerMemberName] string memberName = "",
        [CallerFilePath] string callerFile = "",
        [CallerLineNumber] int lineNumber = 0);

    [LoggerMessage(
        10,
        LogLevel.Error,
        "Job failed with error code {errorCode}, job state {jobState}, error message: {errorMessage}" +
        "\n\tcalled from {memberName}, in file {callerFile}, at line {lineNumber}")]
    public static partial void JobFailed(
        this ILogger logger,
        uint errorCode,
        JobState jobState,
        string errorMessage,
        [CallerMemberName] string memberName = "",
        [CallerFilePath] string callerFile = "",
        [CallerLineNumber] int lineNumber = 0);
}