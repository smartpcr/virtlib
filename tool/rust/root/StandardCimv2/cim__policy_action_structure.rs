// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_PolicyActionStructure struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_PolicyActionStructure {
    #[serde(flatten)]
    pub base: CIM_PolicyComponent,

/// 
    #[serde(rename = "ActionOrder")]
    pub action_order: Option<u16>,
}

impl CIM_PolicyActionStructure {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_PolicyComponent::new(),
            action_order: None,
        }
    }


    /// Sets the value of ActionOrder
    pub fn set_action_order(&mut self, value: u16) {
        self.action_order = Some(value);
    }

    /// Gets the value of ActionOrder
    pub fn get_action_order(&self) -> Option<&u16> {
        self.action_order.as_ref()
    }
}

