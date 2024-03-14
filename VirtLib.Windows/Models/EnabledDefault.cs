// -----------------------------------------------------------------------
// <copyright file="EnabledDefault.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

public enum EnabledDefault : ushort
{
    Enabled = 2,
    Disabled = 3,
    NotApplicable = 5
}

public static class EnabledDefaultEx
{
    public static EnabledDefault ReadEnabledDefault(this object value)
    {
        if (value is ushort val)
        {
            return val switch
            {
                2 => EnabledDefault.Enabled,
                3 => EnabledDefault.Disabled,
                5 => EnabledDefault.NotApplicable,
                _ => EnabledDefault.Enabled
            };
        }

        return EnabledDefault.Enabled;
    }
}