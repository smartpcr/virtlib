// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetAdapter_QosClassificationElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetAdapter_QosClassificationElement {

/// 
    #[serde(rename = "Priority")]
    pub priority: Option<u8>,

/// 
    #[serde(rename = "ProtocolSelector")]
    pub protocol_selector: Option<u16>,

/// 
    #[serde(rename = "ProtocolSpecificValue")]
    pub protocol_specific_value: Option<u16>,
}

impl MSFT_NetAdapter_QosClassificationElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            priority: None,
            protocol_selector: None,
            protocol_specific_value: None,
        }
    }


    /// Sets the value of Priority
    pub fn set_priority(&mut self, value: u8) {
        self.priority = Some(value);
    }

    /// Gets the value of Priority
    pub fn get_priority(&self) -> Option<&u8> {
        self.priority.as_ref()
    }

    /// Sets the value of ProtocolSelector
    pub fn set_protocol_selector(&mut self, value: u16) {
        self.protocol_selector = Some(value);
    }

    /// Gets the value of ProtocolSelector
    pub fn get_protocol_selector(&self) -> Option<&u16> {
        self.protocol_selector.as_ref()
    }

    /// Sets the value of ProtocolSpecificValue
    pub fn set_protocol_specific_value(&mut self, value: u16) {
        self.protocol_specific_value = Some(value);
    }

    /// Gets the value of ProtocolSpecificValue
    pub fn get_protocol_specific_value(&self) -> Option<&u16> {
        self.protocol_specific_value.as_ref()
    }
}

