// -----------------------------------------------------------------------
// <copyright file="JobState.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Definitions;

using System.Linq;

internal enum JobState
{
    New = 2,
    Starting = 3,
    Running = 4,
    Suspended = 5,
    ShuttingDown = 6,
    Completed = 7,
    Terminated = 8,
    Killed = 9,
    Exception = 10,
    CompletedWithWarnings = 32768
}

internal static class JobStateEx
{
    public static bool IsJobComplete(this JobState state) =>
        new[]
            {
                JobState.Completed,
                JobState.CompletedWithWarnings,
                JobState.Terminated,
                JobState.Exception,
                JobState.Killed
            }
            .Contains(state);
}