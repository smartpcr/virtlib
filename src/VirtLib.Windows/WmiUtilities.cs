// -----------------------------------------------------------------------
// <copyright file="WmiUtilities.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows;

using System;
using System.Collections.Generic;
using System.Diagnostics.Eventing.Reader;
using System.Linq;
using System.Management;
using System.Runtime.CompilerServices;
using System.Text;
using System.Threading;
using System.Xml.Linq;
using Microsoft.Extensions.Logging;

public static class WmiUtilities
{
    private const string WmiActivityLogName = "Microsoft-Windows-WMI-Activity/Operational";
    private static readonly XNamespace ns = "http://schemas.microsoft.com/win/2004/08/events/event";
    private static readonly XNamespace userDataNs = "http://manifests.microsoft.com/win/2006/windows/WMI";

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

    public static List<WmiEventLog> GetWmiEventLogs(int processId)
    {
        Thread.Sleep(1000);
        var eventLogQuery = new EventLogQuery(WmiActivityLogName, PathType.LogName);
        using var eventLogReader = new EventLogReader(eventLogQuery);
        var output = new List<WmiEventLog>();
        EventRecord eventRecord = eventLogReader.ReadEvent();
        while (eventRecord != null)
        {
            using (eventRecord)
            {
                if (eventRecord.Level >= 3)
                {
                    continue;
                }

                var eventXml = eventRecord.ToXml();
                var xdoc = XDocument.Parse(eventXml);
                var userData = xdoc
                    .Descendants(ns + "UserData")
                    .Descendants(userDataNs + "Operation_ClientFailure")
                    .Elements(userDataNs + "ClientProcessId")
                    .FirstOrDefault();
                if (userData != null && userData.Value == processId.ToString())
                {
                    var operationElement = xdoc
                        .Descendants(ns + "UserData")
                        .Descendants(userDataNs + "Operation_ClientFailure")
                        .Elements(userDataNs + "Operation")
                        .FirstOrDefault();
                    var eventRecordData = new WmiEventLog
                    {
                        TimeCreated = eventRecord.TimeCreated ?? DateTime.UtcNow,
                        Id = eventRecord.Id.ToString(),
                        ActivityId = eventRecord.ActivityId,
                        Level = eventRecord.LevelDisplayName,
                        Message = eventRecord.FormatDescription(),
                        ProcessId = processId,
                        EventId = eventRecord.Id,
                        Operation = operationElement?.Value ?? eventRecord.TaskDisplayName
                    };
                    output.Add(eventRecordData);
                }
            }

            eventRecord = eventLogReader.ReadEvent();
        }

        return output;
    }

    public static string GetPropertyValues(this ManagementObject managementObject)
    {
        StringBuilder builder = new();
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

        return builder.ToString();
    }

    public static ManagementObjectCollection GetRelated(this ManagementObject managementObject, string relatedClass, string relatedRole)
    {
        return managementObject.GetRelated(relatedClass, relatedRole, null, null, null, null, false, null);
    }

    public static void DisposeCollection(this ManagementObject[] array)
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

    public static ushort ReadUInt16(this object value)
    {
        return value is ushort ushortVal ? ushortVal : (ushort)0;
    }

    public static uint ReadUInt32(this object value)
    {
        return value is uint uintVal ? uintVal : 0;
    }

    public static ulong ReadUInt64(this object value)
    {
        return value is ulong ulongVal ? ulongVal : 0;
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