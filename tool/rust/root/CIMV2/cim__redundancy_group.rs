// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_RedundancyGroup struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_RedundancyGroup {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// 
    #[serde(rename = "CreationClassName")]
    pub creation_class_name: Option<String>,

/// 
    #[serde(rename = "RedundancyStatus")]
    pub redundancy_status: Option<u16>,
}

impl CIM_RedundancyGroup {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
            creation_class_name: None,
            redundancy_status: None,
        }
    }


    /// Sets the value of CreationClassName
    pub fn set_creation_class_name(&mut self, value: String) {
        self.creation_class_name = Some(value);
    }

    /// Gets the value of CreationClassName
    pub fn get_creation_class_name(&self) -> Option<&String> {
        self.creation_class_name.as_ref()
    }

    /// Sets the value of RedundancyStatus
    pub fn set_redundancy_status(&mut self, value: u16) {
        self.redundancy_status = Some(value);
    }

    /// Gets the value of RedundancyStatus
    pub fn get_redundancy_status(&self) -> Option<&u16> {
        self.redundancy_status.as_ref()
    }
}

