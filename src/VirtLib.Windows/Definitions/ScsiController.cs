// -----------------------------------------------------------------------
// <copyright file="ScsiController.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Definitions;

/// <summary>Defines a SCSI Controller.</summary>
public class ScsiController
{
    /// <summary>Gets or sets represents the drives attached to this SCSI Controller.</summary>
    public IScsiDrive[] Drives { get; set; }

    /// <summary>Initializes a new instance of the <see cref="ScsiController"/> class.</summary>
    public ScsiController()
    {
        Drives = new IScsiDrive[64];
    }
}

/// <summary>Defines a drive that can be attached to a SCSI Controller.</summary>
public interface IScsiDrive
{
}