// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_Controller struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_Controller {
    #[serde(flatten)]
    pub base: CIM_LogicalDevice,

/// 
    #[serde(rename = "MaxNumberControlled")]
    pub max_number_controlled: Option<u32>,

/// 
    #[serde(rename = "ProtocolSupported")]
    pub protocol_supported: Option<u16>,

/// 
    #[serde(rename = "TimeOfLastReset")]
    pub time_of_last_reset: Option<String>,
}

impl CIM_Controller {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalDevice::new(),
            max_number_controlled: None,
            protocol_supported: None,
            time_of_last_reset: None,
        }
    }


    /// Sets the value of MaxNumberControlled
    pub fn set_max_number_controlled(&mut self, value: u32) {
        self.max_number_controlled = Some(value);
    }

    /// Gets the value of MaxNumberControlled
    pub fn get_max_number_controlled(&self) -> Option<&u32> {
        self.max_number_controlled.as_ref()
    }

    /// Sets the value of ProtocolSupported
    pub fn set_protocol_supported(&mut self, value: u16) {
        self.protocol_supported = Some(value);
    }

    /// Gets the value of ProtocolSupported
    pub fn get_protocol_supported(&self) -> Option<&u16> {
        self.protocol_supported.as_ref()
    }

    /// Sets the value of TimeOfLastReset
    pub fn set_time_of_last_reset(&mut self, value: String) {
        self.time_of_last_reset = Some(value);
    }

    /// Gets the value of TimeOfLastReset
    pub fn get_time_of_last_reset(&self) -> Option<&String> {
        self.time_of_last_reset.as_ref()
    }
}

