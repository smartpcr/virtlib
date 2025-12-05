// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.TerminalServices
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_TSNetworkAdapterListSetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_TSNetworkAdapterListSetting {
    #[serde(flatten)]
    pub base: Win32_TerminalSetting,

/// 
    #[serde(rename = "NetworkAdapterID")]
    pub network_adapter_id: Option<String>,

/// 
    #[serde(rename = "NetworkAdapterIP")]
    pub network_adapter_ip: Option<String>,
}

impl Win32_TSNetworkAdapterListSetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_TerminalSetting::new(),
            network_adapter_id: None,
            network_adapter_ip: None,
        }
    }


    /// Sets the value of NetworkAdapterID
    pub fn set_network_adapter_id(&mut self, value: String) {
        self.network_adapter_id = Some(value);
    }

    /// Gets the value of NetworkAdapterID
    pub fn get_network_adapter_id(&self) -> Option<&String> {
        self.network_adapter_id.as_ref()
    }

    /// Sets the value of NetworkAdapterIP
    pub fn set_network_adapter_ip(&mut self, value: String) {
        self.network_adapter_ip = Some(value);
    }

    /// Gets the value of NetworkAdapterIP
    pub fn get_network_adapter_ip(&self) -> Option<&String> {
        self.network_adapter_ip.as_ref()
    }
}

