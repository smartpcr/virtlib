// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetEventVmSwitchProvider struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetEventVmSwitchProvider {
    #[serde(flatten)]
    pub base: MSFT_NetEventProviderBase,

/// 
    #[serde(rename = "PortIds")]
    pub port_ids: Vec<u32>,

/// 
    #[serde(rename = "SwitchName")]
    pub switch_name: Option<String>,
}

impl MSFT_NetEventVmSwitchProvider {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetEventProviderBase::new(),
            port_ids: Vec::new(),
            switch_name: None,
        }
    }


    /// Sets the value of PortIds
    pub fn set_port_ids(&mut self, value: Vec<u32>) {
        self.port_ids = value;
    }

    /// Gets the value of PortIds
    pub fn get_port_ids(&self) -> &Vec<u32> {
        &self.port_ids
    }

    /// Sets the value of SwitchName
    pub fn set_switch_name(&mut self, value: String) {
        self.switch_name = Some(value);
    }

    /// Gets the value of SwitchName
    pub fn get_switch_name(&self) -> Option<&String> {
        self.switch_name.as_ref()
    }
}

