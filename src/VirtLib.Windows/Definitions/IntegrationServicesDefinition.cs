// -----------------------------------------------------------------------
// <copyright file="IntegrationServicesDefinition.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Definitions;

/// <summary>Defines the Integration Services settings.</summary>
public class IntegrationServicesDefinition
{
    /// <summary>Gets or sets a value indicating whether offer Data Exchange services.</summary>
    public bool DataExchange { get; set; }

    /// <summary>Gets or sets a value indicating whether offer Guest services.</summary>
    public bool GuestServices { get; set; }

    /// <summary>Gets or sets a value indicating whether offer Heartbeat services.</summary>
    public bool Heartbeat { get; set; }

    /// <summary>Gets or sets a value indicating whether offer Operating System Shutdown services.</summary>
    public bool Shutdown { get; set; }

    /// <summary>Gets or sets a value indicating whether offer Time Synchronization services.</summary>
    public bool TimeSynchronisation { get; set; }

    /// <summary>Gets or sets a value indicating whether offer Backup (Volume Shadow Copy) services.</summary>
    public bool VolumeShadowCopy { get; set; }

    /// <summary>Initializes a new instance of the <see cref="IntegrationServicesDefinition"/> class.</summary>
    public IntegrationServicesDefinition()
    {
        DataExchange = true;
        Heartbeat = true;
        Shutdown = true;
        TimeSynchronisation = true;
        VolumeShadowCopy = true;
    }
}