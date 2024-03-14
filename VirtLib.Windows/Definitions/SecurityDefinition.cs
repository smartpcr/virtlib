// -----------------------------------------------------------------------
// <copyright file="SecurityDefinition.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Definitions;

using System;

/// <summary>Defines the Security settings.</summary>
public class SecurityDefinition
{
    private bool _encryptTraffic;

    /// <summary>Gets or sets a value indicating whether encrypt state and virtual machine migration traffic.</summary>
    public bool EncryptTraffic
    {
        get => _encryptTraffic;
        set
        {
            if (value)
            {
                TrustedPlatformModule = true;
                _encryptTraffic = true;
            }
            else
            {
                _encryptTraffic = false;
            }
        }
    }

    /// <summary>Gets or sets a value indicating whether enable Secure Boot.</summary>
    public bool SecureBoot { get; set; }

    private Guid _secureBootTemplate;

    /// <summary>Gets or sets the template to use when Secure Boot is enabled.</summary>
    public Guid SecureBootTemplate
    {
        get => _secureBootTemplate;
        set
        {
            SecureBoot = true;
            _secureBootTemplate = value;
        }
    }

    private bool _shielding;

    /// <summary>Gets or sets a value indicating whether enable Shielding.</summary>
    public bool Shielding
    {
        get => _shielding;
        set
        {
            if (value)
            {
                SecureBoot = true;
                TrustedPlatformModule = true;
                _encryptTraffic = true;
                _shielding = true;
            }
            else
            {
                _shielding = false;
            }
        }
    }

    /// <summary>Gets or sets a value indicating whether enable Trusted Platform Module.</summary>
    public bool TrustedPlatformModule { get; set; }

    /// <summary>Initializes a new instance of the <see cref="SecurityDefinition"/> class.</summary>
    public SecurityDefinition()
    {
        SecureBoot = true;
        SecureBootTemplate = SecureBootTemplates.MicrosoftWindows;
    }
}