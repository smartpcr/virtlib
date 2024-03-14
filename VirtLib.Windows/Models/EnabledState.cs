// -----------------------------------------------------------------------
// <copyright file="EnabledState.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

using System;

public enum EnabledState : ushort
{
    Unknown = 0,
    Other = 1,
    Enabled = 2,
    Disabled = 3,
    ShuttingDown = 4,
    NotApplicable = 5,
    EnabledButOffline = 6,
    InTest = 7,
    Deferred = 8,
    Quiesce = 9,
    Starting = 10,
    Pausing = 11,
    Suspended = 12,
    StartingWithSnapshot = 13,
    PausingWithSnapshot = 14,
    SuspendedWithSnapshot = 15,
    Stopping = 16,
    Stopped = 17,
    Saving = 18,
    Saved = 19,
    FastSuspended = 20
}

public static class EnabledStateEx
{
    public static EnabledState ReadEnabledState(this object value)
    {
        if (value is ushort val)
        {
            return val switch
            {
                1 => EnabledState.Other,
                2 => EnabledState.Enabled,
                3 => EnabledState.Disabled,
                4 => EnabledState.ShuttingDown,
                5 => EnabledState.NotApplicable,
                6 => EnabledState.EnabledButOffline,
                7 => EnabledState.InTest,
                8 => EnabledState.Deferred,
                9 => EnabledState.Quiesce,
                10 => EnabledState.Starting,
                11 => EnabledState.Pausing,
                12 => EnabledState.Suspended,
                13 => EnabledState.StartingWithSnapshot,
                14 => EnabledState.PausingWithSnapshot,
                15 => EnabledState.SuspendedWithSnapshot,
                16 => EnabledState.Stopping,
                17 => EnabledState.Stopped,
                18 => EnabledState.Saving,
                19 => EnabledState.Saved,
                20 => EnabledState.FastSuspended,
                _ => EnabledState.Unknown
            };
        }

        return EnabledState.Unknown;
    }
}