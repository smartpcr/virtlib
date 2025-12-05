// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_SerialController struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_SerialController {
    #[serde(flatten)]
    pub base: CIM_Controller,

/// The Capabilities property defines chip level compatibility for the SerialController. Therefore, this property describes the buffering and other capabilities of the SerialController that might be inherent in the chip hardware. The property is an enumerated integer.
    #[serde(rename = "Capabilities")]
    pub capabilities: Vec<SerialController_Capabilities>,

/// An array of free-form strings that provides more detailed explanations for any of the SerialController features that are indicated in the Capabilities array. Note, each entry of this array is related to the entry in the Capabilities array that is located at the same index.
    #[serde(rename = "CapabilityDescriptions")]
    pub capability_descriptions: Vec<String>,

/// Maximum baud rate in Bits per Second that is supported by the SerialController.
    #[serde(rename = "MaxBaudRate")]
    pub max_baud_rate: Option<u32>,

/// An enumeration that indicates the operational security for the Controller. For example, information that the external interface of the Device is locked out (value=4) or "Boot Bypass" (value=6) can be described using this property.
    #[serde(rename = "Security")]
    pub security: Option<SerialController_Security>,
}

impl CIM_SerialController {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Controller::new(),
            capabilities: Vec::new(),
            capability_descriptions: Vec::new(),
            max_baud_rate: None,
            security: None,
        }
    }


    /// Sets the value of Capabilities
    pub fn set_capabilities(&mut self, value: Vec<SerialController_Capabilities>) {
        self.capabilities = value;
    }

    /// Gets the value of Capabilities
    pub fn get_capabilities(&self) -> &Vec<SerialController_Capabilities> {
        &self.capabilities
    }

    /// Sets the value of CapabilityDescriptions
    pub fn set_capability_descriptions(&mut self, value: Vec<String>) {
        self.capability_descriptions = value;
    }

    /// Gets the value of CapabilityDescriptions
    pub fn get_capability_descriptions(&self) -> &Vec<String> {
        &self.capability_descriptions
    }

    /// Sets the value of MaxBaudRate
    pub fn set_max_baud_rate(&mut self, value: u32) {
        self.max_baud_rate = Some(value);
    }

    /// Gets the value of MaxBaudRate
    pub fn get_max_baud_rate(&self) -> Option<&u32> {
        self.max_baud_rate.as_ref()
    }

    /// Sets the value of Security
    pub fn set_security(&mut self, value: SerialController_Security) {
        self.security = Some(value);
    }

    /// Gets the value of Security
    pub fn get_security(&self) -> Option<&SerialController_Security> {
        self.security.as_ref()
    }
}

