// -----------------------------------------------------------------------
// <copyright file="VMLogger.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows;

using System.Runtime.CompilerServices;
using Microsoft.Extensions.Logging;

internal static partial class VMLogger
{
    [LoggerMessage(
        1,
        LogLevel.Debug,
        "VM: {vmName} \n\tcalled from {memberName}, in file {callerFile}, at line {lineNumber}\n\t{json}")]
    public static partial void GetVMFinished(
        this ILogger logger,
        string vmName,
        string json,
        [CallerMemberName] string memberName = "",
        [CallerFilePath] string callerFile = "",
        [CallerLineNumber] int lineNumber = 0);

    [LoggerMessage(
        31,
        LogLevel.Information,
        "VM: {vmName} already deleted" +
        "\n\tcalled from {memberName}, in file {callerFile}, at line {lineNumber}")]
    public static partial void VMAlreadyDeleted(
        this ILogger logger,
        string vmName,
        [CallerMemberName] string memberName = "",
        [CallerFilePath] string callerFile = "",
        [CallerLineNumber] int lineNumber = 0);

    [LoggerMessage(
        32,
        LogLevel.Information,
        "VM: {vmName} already deleted, management path = {managementPath}" +
        "\n\tcalled from {memberName}, in file {callerFile}, at line {lineNumber}")]
    public static partial void VMDeleted(
        this ILogger logger,
        string vmName,
        string managementPath,
        [CallerMemberName] string memberName = "",
        [CallerFilePath] string callerFile = "",
        [CallerLineNumber] int lineNumber = 0);

    [LoggerMessage(
        33,
        LogLevel.Information,
        "VM: {vmName} hard disk deleted: {hardDisk}" +
        "\n\tcalled from {memberName}, in file {callerFile}, at line {lineNumber}")]
    public static partial void VMHardDiskDeleted(
        this ILogger logger,
        string vmName,
        string hardDisk,
        [CallerMemberName] string memberName = "",
        [CallerFilePath] string callerFile = "",
        [CallerLineNumber] int lineNumber = 0);
}