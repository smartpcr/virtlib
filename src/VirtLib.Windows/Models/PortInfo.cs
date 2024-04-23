// -----------------------------------------------------------------------
// <copyright file="PortInfo.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

using System.Collections.Generic;

public class PortInfo
{
    public PortConnectionType ConnectionType { get; set; } = PortConnectionType.Nothing;
    public string ConnectedName { get; set; }
    public List<PortFeatureType> FeatureList { get; set; } = new List<PortFeatureType>();
}