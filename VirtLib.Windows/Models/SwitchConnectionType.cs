// -----------------------------------------------------------------------
// <copyright file="ConnectionType.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Models;

public enum SwitchConnectionType : ushort
{
    Private,
    Internal,
    ExternalOnly,
    External
}

public static class SwitchConnectionTypeEx
{
    public static SwitchConnectionType ReadSwitchConnejctionType(this SwitchInfo switchInfo)
    {
        var switchConnectionType = SwitchConnectionType.Private;
        var internallyConnected = false;
        var externallyConnected = false;

        foreach (var port in switchInfo.Ports)
        {
            if (port.ConnectionType == PortConnectionType.Internal)
            {
                internallyConnected = true;
            }
            else if (port.ConnectionType == PortConnectionType.External)
            {
                externallyConnected = true;
            }
        }

        if (internallyConnected && externallyConnected)
        {
            switchConnectionType = SwitchConnectionType.External;
        }
        else if (internallyConnected)
        {
            switchConnectionType = SwitchConnectionType.Internal;
        }
        else if (externallyConnected)
        {
            switchConnectionType = SwitchConnectionType.ExternalOnly;
        }

        return switchConnectionType;
    }
}