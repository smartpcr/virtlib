// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ParallelController struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ParallelController {
    #[serde(flatten)]
    pub base: CIM_Controller,

/// 
    #[serde(rename = "Capabilities")]
    pub capabilities: Vec<u16>,

/// 
    #[serde(rename = "CapabilityDescriptions")]
    pub capability_descriptions: Vec<String>,

/// 
    #[serde(rename = "DMASupport")]
    pub dmasupport: Option<bool>,
}

impl CIM_ParallelController {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Controller::new(),
            capabilities: Vec::new(),
            capability_descriptions: Vec::new(),
            dmasupport: None,
        }
    }


    /// Sets the value of Capabilities
    pub fn set_capabilities(&mut self, value: Vec<u16>) {
        self.capabilities = value;
    }

    /// Gets the value of Capabilities
    pub fn get_capabilities(&self) -> &Vec<u16> {
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

    /// Sets the value of DMASupport
    pub fn set_dmasupport(&mut self, value: bool) {
        self.dmasupport = Some(value);
    }

    /// Gets the value of DMASupport
    pub fn get_dmasupport(&self) -> Option<&bool> {
        self.dmasupport.as_ref()
    }
}

