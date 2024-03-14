// -----------------------------------------------------------------------
// <copyright file="AutomaticStopAction.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Definitions;

using System;

///<summary>Defines the automatic stop action.</summary>
public enum AutomaticStopAction : ushort
{
    ///<summary>Save the virtual machine state.</summary>
    Save = 3,

    ///<summary>Turn off the virtual machine.</summary>
    TurnOff = 2,

    ///<summary>Shutdown the guest operating system.</summary>
    Shutdown = 4
}