// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_SelfRegModuleAction struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_SelfRegModuleAction {
    #[serde(flatten)]
    pub base: CIM_Action,

/// 
    #[serde(rename = "Cost")]
    pub cost: Option<u16>,

/// 
    #[serde(rename = "File")]
    pub file: Option<String>,
}

impl Win32_SelfRegModuleAction {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Action::new(),
            cost: None,
            file: None,
        }
    }


    /// Sets the value of Cost
    pub fn set_cost(&mut self, value: u16) {
        self.cost = Some(value);
    }

    /// Gets the value of Cost
    pub fn get_cost(&self) -> Option<&u16> {
        self.cost.as_ref()
    }

    /// Sets the value of File
    pub fn set_file(&mut self, value: String) {
        self.file = Some(value);
    }

    /// Gets the value of File
    pub fn get_file(&self) -> Option<&String> {
        self.file.as_ref()
    }
}

