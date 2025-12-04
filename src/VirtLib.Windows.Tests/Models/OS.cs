// -----------------------------------------------------------------------
// <copyright file="OS.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Tests.Models;

using System.Runtime.InteropServices;

public class OS
{
    public OSPlatform Platform { get; set; }
    public string Version { get; set; }
    public Architecture Architecture { get; set; }
    public string Description { get; set; }
}