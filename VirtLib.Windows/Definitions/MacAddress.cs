// -----------------------------------------------------------------------
// <copyright file="MacAddress.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Definitions;

using System;
using System.Text.RegularExpressions;

/// <summary>Defines a MAC address.</summary>
public class MacAddress
{
    private readonly string macAddress;

    /// <summary>Initializes a new instance of the <see cref="MacAddress"/> class for the specified MAC address.</summary>
    /// <param name="macAddress">The MAC address.</param>
    public MacAddress(string macAddress)
    {
        string sanitizedMacAddress = Regex.Replace(macAddress.ToUpperInvariant(), "[^0-9A-F]", "");

        if (sanitizedMacAddress.Length != 12)
            throw new ArgumentException($"{macAddress} is not a valid MAC address.");

        byte firstOctect = Convert.ToByte(sanitizedMacAddress.Substring(0, 2), 16);
        if ((firstOctect & (1 << 0)) != 0)
            throw new ArgumentException($"{macAddress} is not a unicast MAC address.");

        this.macAddress = sanitizedMacAddress;
    }

    /// <summary>Converts the current <see cref="MacAddress"/> object to its equivalent string representation.</summary>
    public override string ToString()
    {
        return ToString(false);
    }

    /// <summary>Converts the current <see cref="MacAddress"/> object to its equivalent string representation, optionally in lower case.</summary>
    /// <param name="lowercase"><c>true</c> for lower case; <c>false</c> for upper case.</param>
    public string ToString(bool lowercase)
    {
        if (lowercase)
        {
            return macAddress.ToLowerInvariant();
        }

        return macAddress;
    }

    /// <summary>Converts the current <see cref="MacAddress"/> object to its equivalent string representation using the specified separator.</summary>
    /// <param name="separator">A character that delimits the octects.</param>
    public string ToString(char separator)
    {
        return ToString(separator, false);
    }

    /// <summary>Converts the current <see cref="MacAddress"/> object to its equivalent string representation using the specified separator, optionally in lower case.</summary>
    /// <param name="separator">A character that delimits the octects.</param>
    /// <param name="lowercase"><c>true</c> for lower case; <c>false</c> for upper case.</param>
    public string ToString(char separator, bool lowercase)
    {
        if (lowercase)
        {
            return $"{macAddress.Substring(0, 2).ToLowerInvariant()}{separator}{macAddress.Substring(2, 2).ToLowerInvariant()}" +
                $"{separator}{macAddress.Substring(4, 2).ToLowerInvariant()}{separator}{macAddress.Substring(6, 2).ToLowerInvariant()}" +
                $"{separator}{macAddress.Substring(8, 2).ToLowerInvariant()}{separator}{macAddress.Substring(10, 2).ToLowerInvariant()}";
        }

        return $"{macAddress.Substring(0, 2)}{separator}{macAddress.Substring(2, 2)}" +
               $"{separator}{macAddress.Substring(4, 2)}{separator}{macAddress.Substring(6, 2)}" +
               $"{separator}{macAddress.Substring(8, 2)}{separator}{macAddress.Substring(10, 2)}";
    }
}