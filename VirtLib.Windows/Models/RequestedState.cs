// -----------------------------------------------------------------------
// <copyright file="RequestedState.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

public enum RequestedState : ushort
{
    Unknown = 0,
    Enabled = 2,
    Disabled = 3,
    ShutDown = 4,
    NoChange = 5,
    Offline = 6,
    Test = 7,
    Deferred = 8,
    Quiesce = 9,
    Reboot = 10,
    Reset = 11,
    NotApplicable = 12,
}

public static class RequestedStateEx
{
    public static RequestedState ReadRequestedState(this object value)
    {
        if (value is ushort val)
        {
            return val switch
            {
                0 => RequestedState.Unknown,
                2 => RequestedState.Enabled,
                3 => RequestedState.Disabled,
                4 => RequestedState.ShutDown,
                5 => RequestedState.NoChange,
                6 => RequestedState.Offline,
                7 => RequestedState.Test,
                8 => RequestedState.Deferred,
                9 => RequestedState.Quiesce,
                10 => RequestedState.Reboot,
                11 => RequestedState.Reset,
                12 => RequestedState.NotApplicable,
                _ => RequestedState.Unknown
            };
        }

        return RequestedState.Unknown;
    }
}