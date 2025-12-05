// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetUDPSetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetUDPSetting {
    #[serde(flatten)]
    pub base: CIM_PolicyAction,

/// 
    #[serde(rename = "DynamicPortRangeNumberOfPorts")]
    pub dynamic_port_range_number_of_ports: Option<u16>,

/// 
    #[serde(rename = "DynamicPortRangeStartPort")]
    pub dynamic_port_range_start_port: Option<u16>,
}

impl MSFT_NetUDPSetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_PolicyAction::new(),
            dynamic_port_range_number_of_ports: None,
            dynamic_port_range_start_port: None,
        }
    }


    /// Sets the value of DynamicPortRangeNumberOfPorts
    pub fn set_dynamic_port_range_number_of_ports(&mut self, value: u16) {
        self.dynamic_port_range_number_of_ports = Some(value);
    }

    /// Gets the value of DynamicPortRangeNumberOfPorts
    pub fn get_dynamic_port_range_number_of_ports(&self) -> Option<&u16> {
        self.dynamic_port_range_number_of_ports.as_ref()
    }

    /// Sets the value of DynamicPortRangeStartPort
    pub fn set_dynamic_port_range_start_port(&mut self, value: u16) {
        self.dynamic_port_range_start_port = Some(value);
    }

    /// Gets the value of DynamicPortRangeStartPort
    pub fn get_dynamic_port_range_start_port(&self) -> Option<&u16> {
        self.dynamic_port_range_start_port.as_ref()
    }
}

