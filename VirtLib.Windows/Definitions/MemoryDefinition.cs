// -----------------------------------------------------------------------
// <copyright file="MemoryDefinition.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Definitions;

using System;

/// <summary>Defines the Memory settings.</summary>
public class MemoryDefinition
{
    private uint _buffer;

    /// <summary>Gets or sets the percentage of memory to try to reserve as a buffer.</summary>
    /// <value>The buffer must be between 0 and 100.</value>
    public uint Buffer
    {
        get => _buffer;
        set
        {
            if (value > 100)
            {
                throw new ArgumentOutOfRangeException($"{nameof(Buffer)} must be between 0 and 100.");
            }

            _buffer = value;
        }
    }

    private ulong _maximum;

    /// <summary>Gets or sets the maximum RAM (MB) allowed when Dynamic Memory is enabled.</summary>
    /// <value>The maximum must be between 32 and 12582912.</value>
    public ulong Maximum
    {
        get => _maximum;
        set
        {
            if (value < 32 || value > 12_582_912)
            {
                throw new ArgumentOutOfRangeException($"{nameof(Maximum)} must be between 32 and 12582912.");
            }

            _maximum = value;
        }
    }

    private ulong _minimum;

    /// <summary>Gets or sets the minimum RAM (MB) allowed when Dynamic Memory is enabled.</summary>
    /// <value>The minimum must be between 32 and 12582912.</value>
    public ulong Minimum
    {
        get => _minimum;
        set
        {
            if (value < 32 || value > 12_582_912)
            {
                throw new ArgumentOutOfRangeException($"{nameof(Minimum)} must be between 32 and 12582912.");
            }

            _minimum = value;
        }
    }

    private ulong _startup;

    /// <summary>Gets or sets the amount of RAM (MB) to use at startup.</summary>
    /// <value>The startup must be between 32 and 12582912.</value>
    public ulong Startup
    {
        get => _startup;
        set
        {
            if (value < 32 || value > 12_582_912)
            {
                throw new ArgumentOutOfRangeException($"{nameof(Startup)} must be between 32 and 12582912.");
            }

            _startup = value;
        }
    }

    /// <summary>Gets or sets the priority when balancing memory availability compared to other virtual machines.</summary>
    public MemoryWeight Weight { get; set; }

    /// <summary>Initializes a new instance of the <see cref="MemoryDefinition"/> class.</summary>
    public MemoryDefinition()
    {
        Buffer = 20;
        Minimum = 512;
        Maximum = 1_048_576;
        Startup = 1024;
        Weight = MemoryWeight.Balanced;
    }
}