// -----------------------------------------------------------------------
// <copyright file="NetworkAdaptersDefinition.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Definitions;

using System;
using System.Collections.Generic;

/// <summary>Defines the settings for up to 8 Network Adapters.</summary>
public class NetworkAdaptersDefinition : List<NetworkAdapter>
{
    /// <summary>Add settings for a single Network Adapter.</summary>
    /// <remarks>A maximum of 8 Network Adapters can be added.</remarks>
    public new void Add(NetworkAdapter networkAdapter)
    {
        if (this.Count > 8)
        {
            throw new ArgumentOutOfRangeException($"Cannot add more than 8 {nameof(NetworkAdapter)}.");
        }

        base.Add(networkAdapter);
    }
}