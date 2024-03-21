// -----------------------------------------------------------------------
// <copyright file="Program.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace Generation2VM;

using System;

public class Program
{
    public static void Main(string[] args)
    {
        if (args.Length == 3)
        {
            if (args[0].Equals("Create", StringComparison.OrdinalIgnoreCase))
            {
                var hostName = args[1];
                var vmName = args[2];
                VMCreator.CreateGeneration2VM(hostName, vmName);
            }
        }
    }
}