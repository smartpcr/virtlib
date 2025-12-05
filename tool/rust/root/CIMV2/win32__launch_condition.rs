// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_LaunchCondition struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_LaunchCondition {
    #[serde(flatten)]
    pub base: CIM_Check,

/// 
    #[serde(rename = "Condition")]
    pub condition: Option<String>,
}

impl Win32_LaunchCondition {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Check::new(),
            condition: None,
        }
    }


    /// Sets the value of Condition
    pub fn set_condition(&mut self, value: String) {
        self.condition = Some(value);
    }

    /// Gets the value of Condition
    pub fn get_condition(&self) -> Option<&String> {
        self.condition.as_ref()
    }
}

