// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetAdapterRscCapabilities struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetAdapterRscCapabilities {

/// 
    #[serde(rename = "IPv4Supported")]
    pub ipv4_supported: Option<bool>,

/// 
    #[serde(rename = "IPv6Supported")]
    pub ipv6_supported: Option<bool>,
}

impl MSFT_NetAdapterRscCapabilities {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            ipv4_supported: None,
            ipv6_supported: None,
        }
    }


    /// Sets the value of IPv4Supported
    pub fn set_ipv4_supported(&mut self, value: bool) {
        self.ipv4_supported = Some(value);
    }

    /// Gets the value of IPv4Supported
    pub fn get_ipv4_supported(&self) -> Option<&bool> {
        self.ipv4_supported.as_ref()
    }

    /// Sets the value of IPv6Supported
    pub fn set_ipv6_supported(&mut self, value: bool) {
        self.ipv6_supported = Some(value);
    }

    /// Gets the value of IPv6Supported
    pub fn get_ipv6_supported(&self) -> Option<&bool> {
        self.ipv6_supported.as_ref()
    }
}

