// -----------------------------------------------------------------------
// <copyright file="ReplicationMode.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

public enum ReplicationMode
{
    None = 0,
    Primary = 1,
    Replica = 2,
    TestReplica = 3,
    ExtendedReplica = 4,
}

public static class ReplicationModeEx
{
    public static ReplicationMode ReadReplicationMode(this object value)
    {
        if (value is ushort val)
        {
            return val switch
            {
                0 => ReplicationMode.None,
                1 => ReplicationMode.Primary,
                2 => ReplicationMode.Replica,
                3 => ReplicationMode.TestReplica,
                4 => ReplicationMode.ExtendedReplica,
                _ => ReplicationMode.None,
            };
        }

        return ReplicationMode.None;
    }
}
