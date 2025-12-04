// -----------------------------------------------------------------------
// <copyright file="NetworkAdapter.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Definitions;

using System;

/// <summary>Defines a Network Adapter.</summary>
public class NetworkAdapter
{
    /// <summary>Gets or sets a value indicating whether propagate the name of the network adapter into the guest operating system.</summary>
    public bool DeviceNaming { get; set; }

    /// <summary>Gets or sets a value indicating whether drop DHCP server messages from unauthorized virtual machines pretending to be DHCP servers.</summary>
    public bool DhcpGuard { get; set; }

    /// <summary>Gets or sets a value indicating whether enable IPsec task offloading.</summary>
    public bool IpsecOffloading { get; set; }

    private ushort _ipsecSecurityAssociations;

    /// <summary>Gets or sets the maximum number of offloaded security associations.</summary>
    /// <value>The number must be between 1 and 4094.</value>
    public ushort IpsecSecurityAssociations
    {
        get => _ipsecSecurityAssociations;
        set
        {
            if (value < 1 || value > 4094)
            {
                throw new ArgumentOutOfRangeException($"{nameof(IpsecSecurityAssociations)} must be between 1 and 4094.");
            }

            IpsecOffloading = true;
            _ipsecSecurityAssociations = value;
        }
    }

    /// <summary>Gets or sets the static MAC address.</summary>
    public MacAddress? MacAddress { get; set; }

    /// <summary>Gets or sets a value indicating whether enable MAC address spoofing.</summary>
    public bool MacAddressSpoofing { get; set; }

    private ulong _maximumBandwidth;

    /// <summary>Gets or sets the maximum amount of network bandwidth in Mbps.</summary>
    /// <value>The amount must be between 0 and 999999999.</value>
    public ulong MaximumBandwidth
    {
        get => _maximumBandwidth;
        set
        {
            if (value > 999_999_999)
            {
                throw new ArgumentOutOfRangeException($"{nameof(MaximumBandwidth)} must be between 0 and 999999999.");
            }

            _maximumBandwidth = value;
        }
    }

    private ulong _minimumBandwidth;

    /// <summary>Gets or sets the minimum amount of network bandwidth in Mbps.</summary>
    /// <value>The amount must be between 0 and 999999999.</value>
    public ulong MinimumBandwidth
    {
        get => _minimumBandwidth;
        set
        {
            ArgumentOutOfRangeException.ThrowIfGreaterThan<ulong>(value, 999_999_999);

            _minimumBandwidth = value;
        }
    }

    /// <summary>Gets or sets a value indicating whether enable this network adapter to be part of a team in the guest operating system.</summary>
    public bool NicTeaming { get; set; }

    /// <summary>Gets or sets the port mirroring mode for this network adapter.</summary>
    public PortMirroringMode PortMirroringMode { get; set; }

    /// <summary>Gets or sets a value indicating whether move this virtual machine to another cluster node if a network disconnection is detected.</summary>
    public bool ProtectedNetwork { get; set; }

    /// <summary>Gets or sets a value indicating whether drops router advertisement and redirection messages from unauthorized virtual machines pretending to be routers.</summary>
    public bool RouterGuard { get; set; }

    /// <summary>Gets or sets a value indicating whether enable SR-IOV.</summary>
    public bool SrIov { get; set; }

    /// <summary>Gets or sets the virtual switch to attach.</summary>
    public string VirtualSwitch { get; set; }

    /// <summary>Gets or sets a value indicating whether enable virtual LAN identification.</summary>
    public bool Vlan { get; set; }

    private ushort _vlanId;

    /// <summary>Gets or sets the VLAN identifier.</summary>
    /// <value>The identifier must be between 1 and 4094.</value>
    public ushort VlanId
    {
        get => _vlanId;
        set
        {
            if (value < 1 || value > 4094)
            {
                throw new ArgumentOutOfRangeException($"{nameof(VlanId)} must be between 1 and 4094.");
            }

            Vlan = true;
            _vlanId = value;
        }
    }

    /// <summary>Gets or sets a value indicating whether enable virtual machine queue.</summary>
    public bool Vmq { get; set; }

    /// <summary>Initializes a new instance of the <see cref="NetworkAdapter"/> class.</summary>
    public NetworkAdapter()
    {
        IpsecOffloading = true;
        IpsecSecurityAssociations = 512;
        PortMirroringMode = PortMirroringMode.None;
        ProtectedNetwork = true;
        Vmq = true;
    }

    /// <summary>Initializes a new instance of the <see cref="NetworkAdapter"/> class with the specified switch already attached.</summary>
    /// <param name="virtualSwitch">The virtual switch to attach.</param>
    public NetworkAdapter(string virtualSwitch) : this()
    {
        VirtualSwitch = virtualSwitch;
    }
}