// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_EthernetPort struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_EthernetPort {
    #[serde(flatten)]
    pub base: CIM_NetworkPort,

/// Capabilities of the EthernetPort. For example, the Device might support AlertOnLan, WakeOnLan, Load Balancing, or FailOver. If failover or load balancing capabilities are listed, a SpareGroup (failover) or ExtraCapacityGroup (load balancing) should also be defined to completely describe the capability.
    #[serde(rename = "Capabilities")]
    pub capabilities: Vec<EthernetPort_Capabilities>,

/// An array of free-form strings that provides more detailed explanations for any of the EthernetPort features that are indicated in the Capabilities array. Note, each entry of this array is related to the entry in the Capabilities array that is located at the same index.
    #[serde(rename = "CapabilityDescriptions")]
    pub capability_descriptions: Vec<String>,

/// Specifies which capabilities are enabled from the list of all supported ones, which are defined in the Capabilities array.
    #[serde(rename = "EnabledCapabilities")]
    pub enabled_capabilities: Vec<EthernetPort_EnabledCapabilities>,

/// The maximum size of the INFO (non-MAC) field that will be received or transmitted.
    #[serde(rename = "MaxDataSize")]
    pub max_data_size: Option<u32>,

/// An array of free-form strings that provides more detailed explanations for any of the enabled capabilities that are specified as 'Other'.
    #[serde(rename = "OtherEnabledCapabilities")]
    pub other_enabled_capabilities: Vec<String>,
}

impl CIM_EthernetPort {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_NetworkPort::new(),
            capabilities: Vec::new(),
            capability_descriptions: Vec::new(),
            enabled_capabilities: Vec::new(),
            max_data_size: None,
            other_enabled_capabilities: Vec::new(),
        }
    }


    /// Sets the value of Capabilities
    pub fn set_capabilities(&mut self, value: Vec<EthernetPort_Capabilities>) {
        self.capabilities = value;
    }

    /// Gets the value of Capabilities
    pub fn get_capabilities(&self) -> &Vec<EthernetPort_Capabilities> {
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

    /// Sets the value of EnabledCapabilities
    pub fn set_enabled_capabilities(&mut self, value: Vec<EthernetPort_EnabledCapabilities>) {
        self.enabled_capabilities = value;
    }

    /// Gets the value of EnabledCapabilities
    pub fn get_enabled_capabilities(&self) -> &Vec<EthernetPort_EnabledCapabilities> {
        &self.enabled_capabilities
    }

    /// Sets the value of MaxDataSize
    pub fn set_max_data_size(&mut self, value: u32) {
        self.max_data_size = Some(value);
    }

    /// Gets the value of MaxDataSize
    pub fn get_max_data_size(&self) -> Option<&u32> {
        self.max_data_size.as_ref()
    }

    /// Sets the value of OtherEnabledCapabilities
    pub fn set_other_enabled_capabilities(&mut self, value: Vec<String>) {
        self.other_enabled_capabilities = value;
    }

    /// Gets the value of OtherEnabledCapabilities
    pub fn get_other_enabled_capabilities(&self) -> &Vec<String> {
        &self.other_enabled_capabilities
    }
}

