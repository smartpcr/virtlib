// -----------------------------------------------------------------------
// <copyright file="WmiEventLog.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows;

using System;

public class WmiEventLog
{
    public DateTime TimeCreated { get; set; }
    public string Id { get; set; }
    public Guid? ActivityId { get; set; }
    public string Level { get; set; }
    public string Message { get; set; }
    public int ProcessId { get; set; }
    public int EventId { get; set; }
    public string Operation { get; set; }
}