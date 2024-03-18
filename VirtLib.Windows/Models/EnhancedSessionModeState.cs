// -----------------------------------------------------------------------
// <copyright file="EnhancedSessionModeState.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

public enum EnhancedSessionModeState : ushort
{
    Unknown = 0,
    Disabled = 1,
    Enabled = 2,
    NotSupported = 3,
    RemoteControl = 4,
    Disconnected = 5,
    Connected = 6,
    Max = 7,
    VmConnect = 8,
}

public static class EnhancedSessionModeStateEx
{
    public static EnhancedSessionModeState ReadEnhancedSessionModeState(this object value)
    {
        if (value is ushort val)
        {
            return val switch
            {
                0 => EnhancedSessionModeState.Unknown,
                1 => EnhancedSessionModeState.Disabled,
                2 => EnhancedSessionModeState.Enabled,
                3 => EnhancedSessionModeState.NotSupported,
                4 => EnhancedSessionModeState.RemoteControl,
                5 => EnhancedSessionModeState.Disconnected,
                6 => EnhancedSessionModeState.Connected,
                7 => EnhancedSessionModeState.Max,
                8 => EnhancedSessionModeState.VmConnect,
                _ => EnhancedSessionModeState.Unknown,
            };
        }

        return EnhancedSessionModeState.Unknown;
    }
}
