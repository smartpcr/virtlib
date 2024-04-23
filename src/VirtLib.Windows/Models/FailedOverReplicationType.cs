// -----------------------------------------------------------------------
// <copyright file="FailedOverReplicationType.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

public enum FailedOverReplicationType : ushort
{
    None = 0,
    Regular = 1,
    ApplicationConsistent = 2,
    Planned = 3
}

public static class FailedOverReplicationTypeEx
{
    public static FailedOverReplicationType ReadFailedOverReplicationType(this object value)
    {
        if (value is ushort val)
        {
            return val switch
            {
                0 => FailedOverReplicationType.None,
                1 => FailedOverReplicationType.Regular,
                2 => FailedOverReplicationType.ApplicationConsistent,
                3 => FailedOverReplicationType.Planned,
                _ => FailedOverReplicationType.None,
            };
        }

        return FailedOverReplicationType.None;
    }
}