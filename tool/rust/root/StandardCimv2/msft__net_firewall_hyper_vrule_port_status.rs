// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetFirewallHyperVRulePortStatus struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetFirewallHyperVRulePortStatus {

/// 
    #[serde(rename = "Port")]
    pub port: Option<MSFT_NetFirewallHyperVPort>,

/// 
    #[serde(rename = "Status")]
    pub status: Option<u16>,
}

impl MSFT_NetFirewallHyperVRulePortStatus {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            port: None,
            status: None,
        }
    }


    /// Sets the value of Port
    pub fn set_port(&mut self, value: MSFT_NetFirewallHyperVPort) {
        self.port = Some(value);
    }

    /// Gets the value of Port
    pub fn get_port(&self) -> Option<&MSFT_NetFirewallHyperVPort> {
        self.port.as_ref()
    }

    /// Sets the value of Status
    pub fn set_status(&mut self, value: u16) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&u16> {
        self.status.as_ref()
    }
}

