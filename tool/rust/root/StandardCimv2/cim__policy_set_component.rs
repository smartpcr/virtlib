// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_PolicySetComponent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_PolicySetComponent {
    #[serde(flatten)]
    pub base: CIM_PolicyComponent,

/// 
    #[serde(rename = "Priority")]
    pub priority: Option<u16>,
}

impl CIM_PolicySetComponent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_PolicyComponent::new(),
            priority: None,
        }
    }


    /// Sets the value of Priority
    pub fn set_priority(&mut self, value: u16) {
        self.priority = Some(value);
    }

    /// Gets the value of Priority
    pub fn get_priority(&self) -> Option<&u16> {
        self.priority.as_ref()
    }
}

