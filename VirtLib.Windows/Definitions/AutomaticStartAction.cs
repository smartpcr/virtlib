// -----------------------------------------------------------------------
// <copyright file="AutomaticStartAction.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Definitions;

public enum AutomaticStartAction : ushort
{
    Nothing = 2,
    StartIfRunning = 3,
    AlwaysStart = 4
}