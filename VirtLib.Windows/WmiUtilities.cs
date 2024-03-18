// -----------------------------------------------------------------------
// <copyright file="WmiUtilities.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows;

using System;
using System.Management;
using System.Text;

public static class WmiUtilities
{
    public static ManagementObjectCollection GetRelated(this ManagementObject managementObject, string relatedClass, string relatedRole)
    {
        return managementObject.GetRelated(relatedClass, relatedRole, null, null, null, null, false, null);
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