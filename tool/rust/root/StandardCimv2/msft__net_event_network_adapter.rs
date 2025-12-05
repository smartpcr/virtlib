// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetEventNetworkAdapter struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetEventNetworkAdapter {
    #[serde(flatten)]
    pub base: MSFT_NetEventPacketCaptureTarget,

/// 
    #[serde(rename = "InterfaceDescription")]
    pub interface_description: Option<String>,

/// 
    #[serde(rename = "PromiscuousMode")]
    pub promiscuous_mode: Option<bool>,
}

impl MSFT_NetEventNetworkAdapter {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetEventPacketCaptureTarget::new(),
            interface_description: None,
            promiscuous_mode: None,
        }
    }


    /// Sets the value of InterfaceDescription
    pub fn set_interface_description(&mut self, value: String) {
        self.interface_description = Some(value);
    }

    /// Gets the value of InterfaceDescription
    pub fn get_interface_description(&self) -> Option<&String> {
        self.interface_description.as_ref()
    }

    /// Sets the value of PromiscuousMode
    pub fn set_promiscuous_mode(&mut self, value: bool) {
        self.promiscuous_mode = Some(value);
    }

    /// Gets the value of PromiscuousMode
    pub fn get_promiscuous_mode(&self) -> Option<&bool> {
        self.promiscuous_mode.as_ref()
    }
}

