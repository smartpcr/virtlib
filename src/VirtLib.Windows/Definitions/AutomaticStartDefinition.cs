// -----------------------------------------------------------------------
// <copyright file="AutomaticStartDefinition.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Definitions;

using System;

public class AutomaticStartDefinition
{
    private ulong _delay;

    public AutomaticStartAction Action { get; set; } = AutomaticStartAction.Nothing;

    public ulong Delay
    {
        get => _delay;
        set
        {
            if (value > 999_999_999)
            {
                throw new ArgumentOutOfRangeException($"{nameof(Delay)} must be between 0 and 999999999.");
            }

            Action = AutomaticStartAction.AlwaysStart;
            _delay = value;
        }
    }
}