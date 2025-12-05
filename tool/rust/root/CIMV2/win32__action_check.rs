// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_ActionCheck struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_ActionCheck {

/// 
    #[serde(rename = "Action")]
    pub action: Option<CIM_Action>,

/// 
    #[serde(rename = "Check")]
    pub check: Option<CIM_Check>,
}

impl Win32_ActionCheck {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            action: None,
            check: None,
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

    /// Sets the value of Check
    pub fn set_check(&mut self, value: CIM_Check) {
        self.check = Some(value);
    }

    /// Gets the value of Check
    pub fn get_check(&self) -> Option<&CIM_Check> {
        self.check.as_ref()
    }
}

