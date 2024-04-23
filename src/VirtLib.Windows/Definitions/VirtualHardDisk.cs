// -----------------------------------------------------------------------
// <copyright file="VirtualHardDisk.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Definitions;

using System;

/// <summary>Defines a virtual hard disk.</summary>
public class VirtualHardDisk
{
    /// <summary>Gets or sets the file format of the virtual hard disk.</summary>
    public VirtualHardDiskFormat Format { get; set; }

    /// <summary>Gets or sets the path for the virtual hard disk file.</summary>
    public string Path { get; set; }

    private ulong _size;

    /// <summary>Gets or sets the size in GB of the virtual hard disk.</summary>
    /// <value>The size must be between 1 and 2040 for the VHD format and between 1 and 65536 for the VHDX format.</value>
    public ulong Size
    {
        get => _size;
        set
        {
            switch (Format)
            {
                case VirtualHardDiskFormat.Vhd:
                    if (value < 1 || value > 2040)
                    {
                        throw new ArgumentOutOfRangeException($"{nameof(Size)} must be between 1 and 2040.");
                    }

                    break;

                case VirtualHardDiskFormat.Vhdx:
                    if (value < 1 || value > 65536)
                    {
                        throw new ArgumentOutOfRangeException($"{nameof(Size)} must be between 1 and 65536.");
                    }

                    break;
            }

            _size = value;
        }
    }

    /// <summary>Gets or sets the type of virtual hard disk.</summary>
    public VirtualHardDiskType Type { get; set; }

    /// <summary>Initializes a new instance of the <see cref="VirtualHardDisk"/> class of the specified format, type, size, and path.</summary>
    /// <param name="format">The file format of the virtual hard disk file.</param>
    /// <param name="type">The type of virtual hard disk.</param>
    /// <param name="size">The size in GB of the virtual hard disk.</param>
    /// <param name="path">The path for the virtual hard disk file.</param>
    public VirtualHardDisk(VirtualHardDiskFormat format, VirtualHardDiskType type, ulong size, string path)
    {
        Format = format;
        Path = path;
        Size = size;
        Type = type;
    }
}