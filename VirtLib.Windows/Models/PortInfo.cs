// -----------------------------------------------------------------------
// <copyright file="PortInfo.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

using System;
using System.Collections.Generic;
using System.Management;

public class PortInfo : IDisposable
{
    public ManagementObject Inner { get; set; }
    public PortConnectionType ConnectionType { get; set; } = PortConnectionType.Nothing;
    public string ConnectedName { get; set; }
    public List<PortFeatureType> FeatureList { get; set; } = new List<PortFeatureType>();

    public PortInfo(ManagementObject portObj)
    {
        Inner = portObj;
    }

    public void Dispose() => Inner.Dispose();
}