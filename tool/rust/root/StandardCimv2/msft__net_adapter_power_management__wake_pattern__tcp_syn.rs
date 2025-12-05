// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetAdapterPowerManagement_WakePattern_TcpSyn struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetAdapterPowerManagement_WakePattern_TcpSyn {
    #[serde(flatten)]
    pub base: MSFT_NetAdapterPowerManagement_WakePattern,

/// 
    #[serde(rename = "DestinationAddress")]
    pub destination_address: Option<String>,

/// 
    #[serde(rename = "DestinationPort")]
    pub destination_port: Option<u16>,

/// 
    #[serde(rename = "SourceAddress")]
    pub source_address: Option<String>,

/// 
    #[serde(rename = "SourcePort")]
    pub source_port: Option<u16>,
}

impl MSFT_NetAdapterPowerManagement_WakePattern_TcpSyn {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetAdapterPowerManagement_WakePattern::new(),
            destination_address: None,
            destination_port: None,
            source_address: None,
            source_port: None,
        }
    }


    /// Sets the value of DestinationAddress
    pub fn set_destination_address(&mut self, value: String) {
        self.destination_address = Some(value);
    }

    /// Gets the value of DestinationAddress
    pub fn get_destination_address(&self) -> Option<&String> {
        self.destination_address.as_ref()
    }

    /// Sets the value of DestinationPort
    pub fn set_destination_port(&mut self, value: u16) {
        self.destination_port = Some(value);
    }

    /// Gets the value of DestinationPort
    pub fn get_destination_port(&self) -> Option<&u16> {
        self.destination_port.as_ref()
    }

    /// Sets the value of SourceAddress
    pub fn set_source_address(&mut self, value: String) {
        self.source_address = Some(value);
    }

    /// Gets the value of SourceAddress
    pub fn get_source_address(&self) -> Option<&String> {
        self.source_address.as_ref()
    }

    /// Sets the value of SourcePort
    pub fn set_source_port(&mut self, value: u16) {
        self.source_port = Some(value);
    }

    /// Gets the value of SourcePort
    pub fn get_source_port(&self) -> Option<&u16> {
        self.source_port.as_ref()
    }
}

