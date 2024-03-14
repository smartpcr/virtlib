// -----------------------------------------------------------------------
// <copyright file="HealthState.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

public enum HealthState : ushort
{
    Unknown = 0,
    OK = 5,
    Degraded = 10,
    MinorFailure = 15,
    MajorFailure = 20,
    CriticalFailure = 25,
    NonRecoverableError = 30
}

public static class HealthSateEx
{
    public static HealthState ReadHealthState(this object value)
    {
        if (value is ushort val)
        {
            return val switch
            {
                5 => HealthState.OK,
                10 => HealthState.Degraded,
                15 => HealthState.MinorFailure,
                20 => HealthState.MajorFailure,
                25 => HealthState.CriticalFailure,
                30 => HealthState.NonRecoverableError,
                _ => HealthState.Unknown
            };
        }

        return HealthState.Unknown;
    }
}