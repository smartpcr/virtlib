// -----------------------------------------------------------------------
// <copyright file="CheckpointsDefinition.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Definitions;

/// <summary>Defines the Checkpoints settings.</summary>
public class CheckpointsDefinition
{
    /// <summary>Gets or sets the type of checkpoint that will be created.</summary>
    public CheckpointType Type { get; set; } = CheckpointType.Production;

    private bool _fallback = true;

    /// <summary>Gets or sets a value indicating whether create standard checkpoints if it's not possible to create a production checkpoint.</summary>
    public bool Fallback
    {
        get => _fallback;
        set
        {
            if (value)
            {
                Type = CheckpointType.Production;
                _fallback = true;
            }
            else
            {
                _fallback = false;
            }
        }
    }

    /// <summary>Gets or sets the path where the Checkpoint files will be stored.</summary>
    public string Path { get; set; }
}