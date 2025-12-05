// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ElementCapabilities struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ElementCapabilities {

/// The Capabilities object associated with the element.
    #[serde(rename = "Capabilities")]
    pub capabilities: Option<CIM_Capabilities>,

/// Characteristics provides descriptive information about the Capabilities. when the value 2 "Default" is specified, the associated Capabilities shall represent the default capabilities of the associated Managed Element 
/// when the value 2 "Default" is not specified, the Capabilities instance may represent the default capabilities of the Managed Element
/// When the value 3 "Current" is specified, the associated Capabilities shall represent the current capabilities of the associated Managed Element
/// When the value 3 "Current" is not specified, the Capabilities instance may represent the current capabilities of the Managed Element.
    #[serde(rename = "Characteristics")]
    pub characteristics: Vec<ElementCapabilities_Characteristics>,

/// The managed element.
    #[serde(rename = "ManagedElement")]
    pub managed_element: Option<CIM_ManagedElement>,
}

impl CIM_ElementCapabilities {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            capabilities: None,
            characteristics: Vec::new(),
            managed_element: None,
        }
    }


    /// Sets the value of Capabilities
    pub fn set_capabilities(&mut self, value: CIM_Capabilities) {
        self.capabilities = Some(value);
    }

    /// Gets the value of Capabilities
    pub fn get_capabilities(&self) -> Option<&CIM_Capabilities> {
        self.capabilities.as_ref()
    }

    /// Sets the value of Characteristics
    pub fn set_characteristics(&mut self, value: Vec<ElementCapabilities_Characteristics>) {
        self.characteristics = value;
    }

    /// Gets the value of Characteristics
    pub fn get_characteristics(&self) -> &Vec<ElementCapabilities_Characteristics> {
        &self.characteristics
    }

    /// Sets the value of ManagedElement
    pub fn set_managed_element(&mut self, value: CIM_ManagedElement) {
        self.managed_element = Some(value);
    }

    /// Gets the value of ManagedElement
    pub fn get_managed_element(&self) -> Option<&CIM_ManagedElement> {
        self.managed_element.as_ref()
    }
}

