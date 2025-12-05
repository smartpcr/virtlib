// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_SerialInterface struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_SerialInterface {
    #[serde(flatten)]
    pub base: CIM_ControlledBy,

/// 
    #[serde(rename = "FlowControlInfo")]
    pub flow_control_info: Option<u16>,

/// 
    #[serde(rename = "NumberOfStopBits")]
    pub number_of_stop_bits: Option<u16>,

/// 
    #[serde(rename = "ParityInfo")]
    pub parity_info: Option<u16>,
}

impl CIM_SerialInterface {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ControlledBy::new(),
            flow_control_info: None,
            number_of_stop_bits: None,
            parity_info: None,
        }
    }


    /// Sets the value of FlowControlInfo
    pub fn set_flow_control_info(&mut self, value: u16) {
        self.flow_control_info = Some(value);
    }

    /// Gets the value of FlowControlInfo
    pub fn get_flow_control_info(&self) -> Option<&u16> {
        self.flow_control_info.as_ref()
    }

    /// Sets the value of NumberOfStopBits
    pub fn set_number_of_stop_bits(&mut self, value: u16) {
        self.number_of_stop_bits = Some(value);
    }

    /// Gets the value of NumberOfStopBits
    pub fn get_number_of_stop_bits(&self) -> Option<&u16> {
        self.number_of_stop_bits.as_ref()
    }

    /// Sets the value of ParityInfo
    pub fn set_parity_info(&mut self, value: u16) {
        self.parity_info = Some(value);
    }

    /// Gets the value of ParityInfo
    pub fn get_parity_info(&self) -> Option<&u16> {
        self.parity_info.as_ref()
    }
}

