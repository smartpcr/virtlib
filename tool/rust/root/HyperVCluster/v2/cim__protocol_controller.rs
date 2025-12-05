// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ProtocolController struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ProtocolController {
    #[serde(flatten)]
    pub base: CIM_LogicalDevice,

/// Maximum number of Units that can be controlled by or accessed through this ProtocolController.
    #[serde(rename = "MaxUnitsControlled")]
    pub max_units_controlled: Option<u32>,
}

impl CIM_ProtocolController {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalDevice::new(),
            max_units_controlled: None,
        }
    }


    /// Sets the value of MaxUnitsControlled
    pub fn set_max_units_controlled(&mut self, value: u32) {
        self.max_units_controlled = Some(value);
    }

    /// Gets the value of MaxUnitsControlled
    pub fn get_max_units_controlled(&self) -> Option<&u32> {
        self.max_units_controlled.as_ref()
    }
}

