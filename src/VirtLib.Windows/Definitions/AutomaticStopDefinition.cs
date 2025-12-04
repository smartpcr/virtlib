// -----------------------------------------------------------------------
// <copyright file="AutomaticStopDefinition.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Definitions;

public class AutomaticStopDefinition
{
    /// <summary>
    /// Gets or sets the action when the physical computer shuts down.
    /// </summary>
    public AutomaticStopAction Action { get; set; } = AutomaticStopAction.Save;
}