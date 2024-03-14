// -----------------------------------------------------------------------
// <copyright file="ProcessorDefinition.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Definitions;

using System;

/// <summary>Defines the Processor settings.</summary>
public class ProcessorDefinition
{
    private ulong _hardwareThreadsPerCore;

    /// <summary>Gets or sets the number of hardware threads per core.</summary>
    /// <value>The limit must be between 0 and 2.</value>
    public ulong HardwareThreadsPerCore
    {
        get => _hardwareThreadsPerCore;
        set
        {
            if (value > 2)
            {
                throw new ArgumentOutOfRangeException($"{nameof(HardwareThreadsPerCore)} must be between 0 and 2.");
            }

            _hardwareThreadsPerCore = value;
        }
    }

    private ulong _limit;

    /// <summary>Gets or sets the virtual machine limit (percentage).</summary>
    /// <value>The limit must be between 0 and 100.</value>
    public ulong Limit
    {
        get => _limit;
        set
        {
            if (value > 100)
            {
                throw new ArgumentOutOfRangeException($"{nameof(Limit)} must be between 0 and 100.");
            }

            _limit = value;
        }
    }

    /// <summary>Migrate to a physical computer with a different processor version.</summary>
    public bool LimitFeatures;

    private ulong _numaMemoryPerNode;

    /// <summary>Gets or sets the maximum amount of memory (MB) per NUMA node.</summary>
    /// <value>The amount must be between 32 and 12582912.</value>
    public ulong NumaMemoryPerNode
    {
        get => _numaMemoryPerNode;
        set
        {
            if (value < 32 || value > 12_582_912)
            {
                throw new ArgumentOutOfRangeException($"{nameof(NumaMemoryPerNode)} must be between 32 and 12582912.");
            }

            _numaMemoryPerNode = value;
        }
    }

    private ulong _numaNodesPerSocket;

    /// <summary>Gets or sets the maximum number of NUMA nodes allowed on a single socket.</summary>
    /// <value>The number must be between 1 and 64.</value>
    public ulong NumaNodesPerSocket
    {
        get => _numaNodesPerSocket;
        set
        {
            if (value < 1 || value > 64)
            {
                throw new ArgumentOutOfRangeException($"{nameof(NumaNodesPerSocket)} must be between 1 and 64.");
            }

            _numaNodesPerSocket = value;
        }
    }

    private ulong _numaProcessorsPerNode;

    /// <summary>Gets or sets the maximum number of processors per NUMA node.</summary>
    /// <value>The number must be between 1 and 64.</value>
    public ulong NumaProcessorsPerNode
    {
        get => _numaProcessorsPerNode;
        set
        {
            if (value < 1 || value > 64)
            {
                throw new ArgumentOutOfRangeException($"{nameof(NumaProcessorsPerNode)} must be between 1 and 64.");
            }

            _numaProcessorsPerNode = value;
        }
    }

    private ulong _quantity;

    /// <summary>Gets or sets the number of virtual processors.</summary>
    /// <value>The number must be between 1 and 240.</value>
    public ulong Quantity
    {
        get => _quantity;
        set
        {
            if (value < 1 || value > 240)
            {
                throw new ArgumentOutOfRangeException($"{nameof(Quantity)} must be between 1 and 240.");
            }

            _quantity = value;
        }
    }

    private ulong _reservation;

    /// <summary>Gets or sets the virtual machine reserve (percentage).</summary>
    /// <value>The reserve must be between 0 and 100.</value>
    public ulong Reservation
    {
        get => _reservation;
        set
        {
            if (value > 100)
            {
                throw new ArgumentOutOfRangeException($"{nameof(Reservation)} must be between 0 and 100.");
            }

            _reservation = value;
        }
    }

    private uint _weight;

    /// <summary>Gets or sets the priority to use when balancing processor resources compared to other virtual machines.</summary>
    /// <value>The weight must be between 0 and 10000.</value>
    public uint Weight
    {
        get => _weight;
        set
        {
            if (value > 10000)
            {
                throw new ArgumentOutOfRangeException($"{nameof(Weight)} must be between 0 and 10000.");
            }

            _weight = value;
        }
    }

    /// <summary>Initializes a new instance of the <see cref="ProcessorDefinition"/> class.</summary>
    public ProcessorDefinition()
    {
        Quantity = 1;
        Limit = 100;
        Weight = 100;
        NumaMemoryPerNode = 131072;
        NumaNodesPerSocket = 1;
        NumaProcessorsPerNode = 8;
        HardwareThreadsPerCore = 0;
    }
}