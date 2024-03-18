// -----------------------------------------------------------------------
// <copyright file="ReplicationState.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

public enum ReplicationState : ushort
{
    Disabled = 0,
    ReadyForReplication = 1,
    WaitingForInitialReplication = 2,
    Replicating = 3,
    Synced = 4,
    Recovered = 5,
    Committed = 6,
    Suspended = 7,
    Critical = 8,
    WaitingResynchronization = 9,
    Resynchronizing = 10,
    ResynchronizationSuspended = 11,
    FailoverInProgress = 12,
    FailbackInProgress = 13,
    FailbackComplete = 14
}

public static class ReplicationStateEx
{
    public static ReplicationState ReadReplicationState(this object value)
    {
        if (value is ushort val)
        {
            return val switch
            {
                0 => ReplicationState.Disabled,
                1 => ReplicationState.ReadyForReplication,
                2 => ReplicationState.WaitingForInitialReplication,
                3 => ReplicationState.Replicating,
                4 => ReplicationState.Synced,
                5 => ReplicationState.Recovered,
                6 => ReplicationState.Committed,
                7 => ReplicationState.Suspended,
                8 => ReplicationState.Critical,
                9 => ReplicationState.WaitingResynchronization,
                10 => ReplicationState.Resynchronizing,
                11 => ReplicationState.ResynchronizationSuspended,
                12 => ReplicationState.FailoverInProgress,
                13 => ReplicationState.FailbackInProgress,
                14 => ReplicationState.FailbackComplete,
                _ => ReplicationState.Disabled,
            };
        }

        return ReplicationState.Disabled;
    }
}