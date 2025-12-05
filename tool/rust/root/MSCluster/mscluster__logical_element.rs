// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSCluster_LogicalElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSCluster_LogicalElement {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// 
    #[serde(rename = "Characteristics")]
    pub characteristics: Option<u32>,

/// 
    #[serde(rename = "Flags")]
    pub flags: Option<u32>,
}

impl MSCluster_LogicalElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
            characteristics: None,
            flags: None,
        }
    }


    /// Sets the value of Characteristics
    pub fn set_characteristics(&mut self, value: u32) {
        self.characteristics = Some(value);
    }

    /// Gets the value of Characteristics
    pub fn get_characteristics(&self) -> Option<&u32> {
        self.characteristics.as_ref()
    }

    /// Sets the value of Flags
    pub fn set_flags(&mut self, value: u32) {
        self.flags = Some(value);
    }

    /// Gets the value of Flags
    pub fn get_flags(&self) -> Option<&u32> {
        self.flags.as_ref()
    }
}

