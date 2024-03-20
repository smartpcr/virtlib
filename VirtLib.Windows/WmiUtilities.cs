// -----------------------------------------------------------------------
// <copyright file="WmiUtilities.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows;

using System;
using System.Management;
using System.Runtime.CompilerServices;
using System.Text;
using Microsoft.Extensions.Logging;

public static class WmiUtilities
{
    public static void LogManagementObject(
        this ILogger logger,
        ManagementObject managementObject,
        [CallerMemberName] string memberName = "",
        [CallerFilePath] string callerFile = "",
        [CallerLineNumber] int lineNumber = 0)
    {
        StringBuilder builder = new();
        var elementName = managementObject["ElementName"]?.ToString() ?? string.Empty;
        builder.AppendLine($"{elementName}: called from {memberName}, in file {callerFile}, at line {lineNumber}");
        foreach (var prop in managementObject.Properties)
        {
            if (prop.Value == null)
            {
                continue;
            }

            builder.AppendLine(prop.IsArray
                ? $"\t{prop.Name} ({prop.Type}[]): {prop.Value.ReadValueArray()}"
                : $"\t{prop.Name} ({prop.Type}): {prop.Value}");
        }

        logger.LogManagementObject(elementName, builder.ToString(), memberName, callerFile, lineNumber);
    }

    public static ManagementObjectCollection GetRelated(this ManagementObject managementObject, string relatedClass, string relatedRole)
    {
        return managementObject.GetRelated(relatedClass, relatedRole, null, null, null, null, false, null);
    }

    public static void Dispose(this ManagementObject[] array)
    {
        foreach (ManagementObject managementObject in array)
        {
            managementObject.Dispose();
        }
    }

    public static ManagementObject[] ToObjectArray(this string[] managementStrings)
    {
        ManagementObject[] managementObjects = new ManagementObject[managementStrings.Length];
        for (int index = 0; index < managementStrings.Length; index++)
        {
            managementObjects[index] = new ManagementObject(managementStrings[index]);
        }

        return managementObjects;
    }

    public static string[] ToStringArray(this ManagementObject[] managementObjects)
    {
        string[] managementStrings = new string[managementObjects.Length];
        for (int index = 0; index < managementObjects.Length; index++)
        {
            managementStrings[index] = managementObjects[index].GetText(TextFormat.WmiDtd20);
        }

        return managementStrings;
    }

    public static bool ReadBool(this object value)
    {
        return value is bool boolVal && boolVal;
    }

    public static Guid ReadGuid(this object value)
    {
        if (value is string strVal && Guid.TryParse(strVal, out var guid))
        {
            return guid;
        }

        if (value is Guid id)
        {
            return id;
        }

        return Guid.Empty;
    }

    public static DateTime? ReadDateTime(this object value)
    {
        if (value is string strVal && !string.IsNullOrEmpty(strVal))
        {
            return ManagementDateTimeConverter.ToDateTime(strVal);
        }

        return null;
    }

    public static string ReadValueArray(this object? value)
    {
        if (value == null)
        {
            return string.Empty;
        }

        var array = (Array)value;
        var buffer = new StringBuilder();
        foreach (var item in array)
        {
            buffer.Append(item);
            buffer.Append(", ");
        }

        return buffer.ToString().TrimEnd(',', ' ');
    }
}