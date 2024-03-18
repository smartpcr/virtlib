// -----------------------------------------------------------------------
// <copyright file="ReplicationHealth.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

public enum ReplicationHealth : ushort
{
    NotApplicable = 0,
    Ok = 1,
    Warning = 2,
    Critical = 3,
}

public static class ReplicationHealthEx
{
    public static ReplicationHealth ReadReplicationHealth(this object value)
    {
        if (value is ushort val)
        {
            return val switch
            {
                0 => ReplicationHealth.NotApplicable,
                1 => ReplicationHealth.Ok,
                2 => ReplicationHealth.Warning,
                3 => ReplicationHealth.Critical,
                _ => ReplicationHealth.NotApplicable,
            };
        }

        return ReplicationHealth.NotApplicable;
    }
}
