// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_SoftwareElementActions struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_SoftwareElementActions {

/// 
    #[serde(rename = "Action")]
    pub action: Option<CIM_Action>,

/// 
    #[serde(rename = "Element")]
    pub element: Option<CIM_SoftwareElement>,
}

impl CIM_SoftwareElementActions {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            action: None,
            element: None,
        }
    }


    /// Sets the value of Action
    pub fn set_action(&mut self, value: CIM_Action) {
        self.action = Some(value);
    }

    /// Gets the value of Action
    pub fn get_action(&self) -> Option<&CIM_Action> {
        self.action.as_ref()
    }

    /// Sets the value of Element
    pub fn set_element(&mut self, value: CIM_SoftwareElement) {
        self.element = Some(value);
    }

    /// Gets the value of Element
    pub fn get_element(&self) -> Option<&CIM_SoftwareElement> {
        self.element.as_ref()
    }
}

