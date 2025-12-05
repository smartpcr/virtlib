// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetAdapterPowerManagement_Offload_Arp struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetAdapterPowerManagement_Offload_Arp {
    #[serde(flatten)]
    pub base: MSFT_NetAdapterPowerManagement_Offload,

/// 
    #[serde(rename = "HostIPv4Address")]
    pub host_ipv4_address: Option<String>,

/// 
    #[serde(rename = "MACAddress")]
    pub macaddress: Option<String>,

/// 
    #[serde(rename = "RemoteIPv4Address")]
    pub remote_ipv4_address: Option<String>,
}

impl MSFT_NetAdapterPowerManagement_Offload_Arp {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetAdapterPowerManagement_Offload::new(),
            host_ipv4_address: None,
            macaddress: None,
            remote_ipv4_address: None,
        }
    }


    /// Sets the value of HostIPv4Address
    pub fn set_host_ipv4_address(&mut self, value: String) {
        self.host_ipv4_address = Some(value);
    }

    /// Gets the value of HostIPv4Address
    pub fn get_host_ipv4_address(&self) -> Option<&String> {
        self.host_ipv4_address.as_ref()
    }

    /// Sets the value of MACAddress
    pub fn set_macaddress(&mut self, value: String) {
        self.macaddress = Some(value);
    }

    /// Gets the value of MACAddress
    pub fn get_macaddress(&self) -> Option<&String> {
        self.macaddress.as_ref()
    }

    /// Sets the value of RemoteIPv4Address
    pub fn set_remote_ipv4_address(&mut self, value: String) {
        self.remote_ipv4_address = Some(value);
    }

    /// Gets the value of RemoteIPv4Address
    pub fn get_remote_ipv4_address(&self) -> Option<&String> {
        self.remote_ipv4_address.as_ref()
    }
}

