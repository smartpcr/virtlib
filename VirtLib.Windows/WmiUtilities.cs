// -----------------------------------------------------------------------
// <copyright file="WmiUtilities.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows;

using System;
using System.Management;

public static class WmiUtilities
{
    public static ManagementObjectCollection GetRelated(this ManagementObject managementObject, string relatedClass, string relatedRole)
    {
        return managementObject.GetRelated(relatedClass, relatedRole, null, null, null, null, false, null);
    }

    public static DateTime? ReadDateTime(this object value)
    {
        if (value is string strVal && !string.IsNullOrEmpty(strVal))
        {
            return ManagementDateTimeConverter.ToDateTime(strVal);
        }

        return null;
    }
}