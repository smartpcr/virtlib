// -----------------------------------------------------------------------
// <copyright file="VirtualHardDrive.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Definitions;

using System;

/// <summary>Defines a virtual hard drive.</summary>
public class VirtualHardDrive : IScsiDrive
{
    private ulong _maximumIops;

    /// <summary>Gets or sets the maximum number of 8 KB IOPS.</summary>
    /// <value>The number must be between 0 and 1000000000.</value>
    public ulong MaximumIops
    {
        get => _maximumIops;
        set
        {
            if (value > 1_000_000_000)
            {
                throw new ArgumentOutOfRangeException($"{nameof(MaximumIops)} must be between 0 and 1000000000.");
            }

            _maximumIops = value;
        }
    }

    private ulong _minimumIops;

    /// <summary>Gets or sets the minimum number of 8 KB IOPS.</summary>
    /// <value>The number must be between 0 and 1000000000.</value>
    public ulong MinimumIops
    {
        get => _minimumIops;
        set
        {
            if (value > 1_000_000_000)
            {
                throw new ArgumentOutOfRangeException($"{nameof(MinimumIops)} must be between 0 and 1000000000.");
            }

            _minimumIops = value;
        }
    }

    /// <summary>Gets or sets the attached virtual hard disk.</summary>
    public VirtualHardDisk? VirtualHardDisk { get; set; }

    /// <summary>Initializes a new instance of the <see cref="VirtualHardDrive"/> class.</summary>
    public VirtualHardDrive()
    {
    }

    /// <summary>Initializes a new instance of the <see cref="VirtualHardDrive"/> class with the specified disk already attached.</summary>
    /// <param name="virtualHardDisk">The virtual hard disk to attach.</param>
    public VirtualHardDrive(VirtualHardDisk virtualHardDisk)
    {
        VirtualHardDisk = virtualHardDisk;
    }

    /// <summary>Initializes a new instance of the <see cref="VirtualHardDrive"/> class using the specified quality of service settings with the specified disk already attached.</summary>
    /// <param name="virtualHardDisk">The virtual hard disk to attach.</param>
    /// <param name="minimumIOPS">The minimum number of 8 KB IOPS.</param>
    /// <param name="maximumIOPS">The maximum number of 8 KB IOPS.</param>
    public VirtualHardDrive(VirtualHardDisk virtualHardDisk, ulong minimumIOPS, ulong maximumIOPS)
    {
        VirtualHardDisk = virtualHardDisk;
        MinimumIops = minimumIOPS;
        MaximumIops = maximumIOPS;
    }
}